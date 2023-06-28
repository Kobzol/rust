use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;

use rand::Rng;

use crate::runtest::ProcRes;

struct RustcDaemon {
    daemon: Child,
    client: TcpStream,
    cmd_reader: BufReader<TcpStream>,
    stdout_reader: ChildStdout,
    stderr_reader: ChildStderr,
    buffer: String,
    buffer_vec: Vec<u8>,
}

#[derive(serde::Serialize)]
struct RemoteCommand {
    working_dir: Option<String>,
    env: HashMap<String, String>,
    args: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct CommandResult {
    exit_code: i32,
}

impl RustcDaemon {
    fn connect(binary: PathBuf) -> Self {
        let mut rng = rand::thread_rng();
        let port: u16 = rng.gen_range(2000..40000);
        let mut child = Command::new(binary)
            .env("RUSTC_DAEMON", port.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // .stdout(Stdio::inherit())
            // .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        std::thread::sleep(Duration::from_secs(1));
        let client = TcpStream::connect_timeout(
            &SocketAddr::from(([127, 0, 0, 1], port)),
            Duration::from_secs(5),
        )
        .expect("Cannot connect to daemon");
        client.set_nodelay(true).expect("set_nodelay call failed");

        // There's a buffer read race condition with stdout/stderr/socket here, we should just use
        // async.
        // let mut fake_child = Command::new("sleep")
        //     .arg("120")
        //     .stdout(Stdio::piped())
        //     .stderr(Stdio::piped())
        //     .spawn()
        //     .unwrap();
        // let stdout_reader =
        //     BufReader::with_capacity(8192 * 1024, fake_child.stdout.take().unwrap());
        // let stderr_reader =
        //     BufReader::with_capacity(8192 * 1024, fake_child.stderr.take().unwrap());
        let stdout_reader = child.stdout.take().unwrap();
        let stderr_reader = child.stderr.take().unwrap();

        let reader = BufReader::new(client.try_clone().unwrap());
        Self {
            daemon: child,
            client,
            cmd_reader: reader,
            buffer: String::new(),
            stdout_reader,
            stderr_reader,
            buffer_vec: vec![0; 8192],
        }
    }

    fn run(&mut self, command: &Command, cmdline: String) -> ProcRes {
        let mut env: HashMap<String, String> = Default::default(); //std::env::vars().collect();
        for (key, value) in command.get_envs() {
            env.insert(
                key.to_str().unwrap().to_string(),
                value.map(|s| s.to_str().unwrap().to_string()).unwrap_or_else(|| "".to_string()),
            );
        }
        let args: Vec<String> =
            command.get_args().map(|arg| arg.to_str().unwrap().to_string()).collect();

        let cmd = RemoteCommand {
            working_dir: command.get_current_dir().map(|p| p.to_str().unwrap().to_string()),
            env,
            args,
        };
        let cmd = serde_json::to_string(&cmd).unwrap();
        let mut client = &self.client;
        // eprintln!("Starting");
        client.write_all(format!("{cmd}\n").as_bytes()).unwrap();
        client.flush().unwrap();

        // eprintln!("Before read_line");

        const TERMINATOR: &[u8] = b"----------------END-SESSION----------------\n";

        let mut stderr = read_stream(&mut self.stderr_reader, &mut self.buffer_vec);
        let mut stdout = read_stream(&mut self.stdout_reader, &mut self.buffer_vec);

        self.buffer.clear();
        self.cmd_reader.read_line(&mut self.buffer).unwrap();
        // eprintln!("After read_line");

        let result: CommandResult = serde_json::from_str(&self.buffer).unwrap();
        ProcRes { cmdline, status: result.exit_code.into(), stdout, stderr }
    }
}

fn read_stream<T: std::io::Read>(reader: &mut T, buffer: &mut Vec<u8>) -> String {
    const TERMINATOR: &[u8] = b"----------------END-SESSION----------------";

    let mut stream = Vec::new();
    loop {
        let read = reader.read(buffer).unwrap();
        // eprintln!("Read {read} bytes");
        if read == 0 {
            panic!("Daemon ended");
        }
        let input = &buffer[0..read];
        stream.extend_from_slice(&input);

        if stream.ends_with(TERMINATOR) {
            stream.drain(stream.len() - TERMINATOR.len()..);
            return String::from_utf8(stream).unwrap();
        }
    }
}

struct QueueCommand {
    command: Command,
    cmdline: String,
    result: oneshot::Sender<ProcRes>,
}

pub struct RustcDaemonQueue {
    queue: Arc<crossbeam_queue::SegQueue<QueueCommand>>,
    workers: Vec<std::thread::JoinHandle<()>>,
}

impl RustcDaemonQueue {
    pub fn new(rustc_binary: PathBuf, worker_count: usize) -> Self {
        let queue = Arc::new(crossbeam_queue::SegQueue::<QueueCommand>::new());
        let workers: Vec<_> = (0..worker_count)
            .map(|i| {
                let queue = queue.clone();
                let rustc_binary = rustc_binary.clone();
                std::thread::spawn(move || {
                    let mut daemon = RustcDaemon::connect(rustc_binary);
                    while let Some(item) = queue.pop() {
                        let result = daemon.run(&item.command, item.cmdline);
                        item.result.send(result).unwrap()
                    }
                })
            })
            .collect();

        Self { queue, workers }
    }

    pub fn run(&self, command: Command, cmdline: String) -> ProcRes {
        let (tx, rx) = oneshot::channel();
        let item = QueueCommand { command, cmdline, result: tx };
        self.queue.push(item);
        rx.recv().unwrap()
    }
}
