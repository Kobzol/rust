pub mod io;

use crate::environment::Environment;
use crate::utils::io::delete_directory;
use humansize::BINARY;
use psutil::disk::disk_usage;

pub fn format_env_variables() -> String {
    let vars = std::env::vars().map(|(key, value)| format!("{key}={value}")).collect::<Vec<_>>();
    vars.join("\n")
}

pub fn print_free_disk_space() -> anyhow::Result<()> {
    let usage = disk_usage("/")?;

    log::info!(
        "Free disk space: {} out of total {} ({:.2}% used)",
        humansize::format_size(usage.free(), BINARY),
        humansize::format_size(usage.total(), BINARY),
        usage.percent()
    );
    Ok(())
}

pub fn clear_llvm_files(env: &dyn Environment) -> anyhow::Result<()> {
    // Bootstrap currently doesn't support rebuilding LLVM when PGO options
    // change (or any other llvm-related options); so just clear out the relevant
    // directories ourselves.
    log::info!("Clearing LLVM build files");
    delete_directory(&env.build_artifacts().join("llvm"))?;
    delete_directory(&env.build_artifacts().join("lld"))?;
    Ok(())
}
