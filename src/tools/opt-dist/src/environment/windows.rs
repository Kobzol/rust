use crate::environment::Environment;
use crate::utils::io::move_directory;
use camino::Utf8PathBuf;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub(super) struct WindowsEnvironment {
    checkout_dir: Utf8PathBuf,
}

impl WindowsEnvironment {
    pub fn new() -> Self {
        Self { checkout_dir: std::env::current_dir().unwrap().try_into().unwrap() }
    }
}

impl Environment for WindowsEnvironment {
    fn checkout_path(&self) -> Utf8PathBuf {
        self.checkout_dir.clone()
    }

    fn downloaded_llvm_dir(&self) -> Utf8PathBuf {
        self.checkout_path().join("citools/clang-rust")
    }

    fn opt_artifacts(&self) -> Utf8PathBuf {
        self.checkout_path().join("opt-artifacts")
    }

    fn build_root(&self) -> Utf8PathBuf {
        self.checkout_path()
    }

    fn prepare_rustc_perf(&self) -> anyhow::Result<()> {
        const PERF_COMMIT: &str = "9dfaa35193154b690922347ee1141a06ec87a199";

        let url = format!("https://github.com/rust-lang/rustc-perf/archive/{PERF_COMMIT}.zip");
        let response = reqwest::blocking::get(url)?.error_for_status()?.bytes()?.to_vec();

        let mut archive = ZipArchive::new(Cursor::new(response))?;
        archive.extract(self.rustc_perf_dir())?;
        move_directory(
            &self.rustc_perf_dir().join(format!("rustc-perf-{PERF_COMMIT}")),
            &self.rustc_perf_dir(),
        )?;

        Ok(())
    }

    fn supports_bolt(&self) -> bool {
        false
    }
}
