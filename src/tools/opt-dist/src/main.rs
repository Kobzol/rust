use anyhow::Context;
use log::LevelFilter;

use crate::environment::{create_environment, Environment};
use crate::exec::Bootstrap;
use crate::timer::Timer;
use crate::training::{gather_llvm_bolt_profiles, gather_llvm_profiles, gather_rustc_profiles};
use crate::utils::io::reset_directory;
use crate::utils::{clear_llvm_files, format_env_variables, print_free_disk_space};

mod environment;
mod exec;
mod metrics;
mod timer;
mod training;
mod utils;

fn is_try_build() -> bool {
    std::env::var("DIST_TRY_BUILD").unwrap_or_else(|_| "0".to_string()) != "0"
}

fn execute_pipeline(
    env: &dyn Environment,
    timer: &mut Timer,
    mut dist_args: Vec<String>,
) -> anyhow::Result<()> {
    reset_directory(&env.opt_artifacts())?;
    env.prepare_rustc_perf()?;

    // Stage 1: Build rustc + PGO instrumented LLVM
    let llvm_pgo_profile = timer.section("Stage 1 (LLVM PGO)", |stage| {
        let llvm_profile_dir_root = env.opt_artifacts().join("llvm-pgo");

        stage.section("Build rustc and LLVM", |section| {
            Bootstrap::build(env).llvm_pgo_instrument(&llvm_profile_dir_root).run(section)
        })?;

        let profile = stage
            .section("Gather profiles", |_| gather_llvm_profiles(env, &llvm_profile_dir_root))?;

        print_free_disk_space()?;
        clear_llvm_files(env)?;

        Ok(profile)
    })?;

    // Stage 2: Build PGO instrumented rustc + LLVM
    let rustc_pgo_profile = timer.section("Stage 2 (rustc PGO)", |stage| {
        let rustc_profile_dir_root = env.opt_artifacts().join("rustc-pgo");

        stage.section("Build rustc and LLVM", |section| {
            Bootstrap::build(env).rustc_pgo_instrument(&rustc_profile_dir_root).run(section)
        })?;

        let profile = stage
            .section("Gather profiles", |_| gather_rustc_profiles(env, &rustc_profile_dir_root))?;
        print_free_disk_space()?;
        clear_llvm_files(env)?;

        Ok(profile)
    })?;

    let llvm_bolt_profile = if env.supports_bolt() {
        // Stage 3: Build rustc + BOLT instrumented LLVM
        timer.section("Stage 3 (LLVM BOLT)", |stage| {
            stage.section("Build rustc and LLVM", |stage| {
                Bootstrap::build(env)
                    .llvm_bolt_instrument(&llvm_pgo_profile, &rustc_pgo_profile)
                    .run(stage)
            })?;

            let profile = stage.section("Gather profiles", |_| gather_llvm_bolt_profiles(env))?;
            print_free_disk_space()?;

            // LLVM is not being cleared here, we want to reuse the previous build

            Ok(Some(profile))
        })?
    } else {
        None
    };

    dist_args.extend([
        "--llvm-profile-use".to_string(),
        llvm_pgo_profile.0.to_string(),
        "--rust-profile-use".to_string(),
        rustc_pgo_profile.0.to_string(),
    ]);
    if let Some(llvm_bolt_profile) = llvm_bolt_profile {
        dist_args.extend(["--llvm-bolt-profile-use".to_string(), llvm_bolt_profile.0.to_string()]);
    }

    // Stage 4: Build PGO optimized rustc + PGO/BOLT optimized LLVM
    timer.section("Stage 4 (final build)", |stage| Bootstrap::dist(env, &dist_args).run(stage))?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Make sure that we get backtraces for easier debugging in CI
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .parse_default_env()
        .init();

    let mut build_args: Vec<String> = std::env::args().skip(1).collect();
    log::info!("Running optimized build pipeline with args `{}`", build_args.join(" "));
    log::info!("Environment values\n{}", format_env_variables());

    if let Ok(config) = std::fs::read_to_string("config.toml") {
        log::info!("Contents of `config.toml`:\n{config}");
    }

    // Skip components that are not needed for try builds to speed them up
    if is_try_build() {
        log::info!("Skipping building of unimportant components for a try build");
        for target in [
            "rust-docs",
            "rustc-docs",
            "rust-docs-json",
            "rust-analyzer",
            "rustc-src",
            "clippy",
            "miri",
            "rustfmt",
        ] {
            build_args.extend(["--exclude".to_string(), target.to_string()]);
        }
    }

    let mut timer = Timer::new();
    let env = create_environment();

    let result = execute_pipeline(env.as_ref(), &mut timer, build_args);
    log::info!("Timer results\n{}", timer.format_stats());

    print_free_disk_space()?;

    result.context("Optimized build pipeline has failed")
}
