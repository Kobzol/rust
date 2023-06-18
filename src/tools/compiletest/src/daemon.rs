use crate::runtest::{CustomExitStatus, ProcRes};
use rand::Rng;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::os::unix::prelude::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Child, Command, ExitCode, ExitStatus};
use std::rc::Rc;
use std::time::Duration;

pub struct RustcDaemon {
    daemon: Child,
    client: TcpStream,
    reader: BufReader<TcpStream>,
    buffer: String,
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
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl RustcDaemon {
    pub fn connect(binary: PathBuf) -> Self {
        let mut rng = rand::thread_rng();
        let port: u16 = rng.gen_range(2000..40000);
        let child = Command::new(binary).env("RUSTC_DAEMON", port.to_string()).spawn().unwrap();

        std::thread::sleep(Duration::from_secs(2));
        let client = TcpStream::connect_timeout(
            &SocketAddr::from(([127, 0, 0, 1], port)),
            Duration::from_secs(5),
        )
        .expect("Cannot connect to daemon");

        let reader = BufReader::new(client.try_clone().unwrap());
        Self { daemon: child, client, reader, buffer: String::new() }
    }

    pub fn run(&mut self, command: &Command, cmdline: String) -> ProcRes {
        let mut env: HashMap<String, String> = std::env::vars().collect();
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
        client.write_all(format!("{cmd}\n").as_bytes()).unwrap();

        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).unwrap();

        let result: CommandResult = serde_json::from_str(&self.buffer).unwrap();
        ProcRes {
            cmdline,
            status: result.exit_code.into(),
            stdout: String::from_utf8_lossy(&result.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
        }
    }
}
