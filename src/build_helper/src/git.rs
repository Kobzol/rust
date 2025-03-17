#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::ci::CiEnv;

pub struct GitConfig<'a> {
    pub git_repository: &'a str,
    pub nightly_branch: &'a str,
    pub git_merge_commit_email: &'a str,
}

/// Runs a command and returns the output
pub fn output_result(cmd: &mut Command) -> Result<String, String> {
    let output = match cmd.stderr(Stdio::inherit()).output() {
        Ok(status) => status,
        Err(e) => return Err(format!("failed to run command: {:?}: {}", cmd, e)),
    };
    if !output.status.success() {
        return Err(format!(
            "command did not execute successfully: {:?}\n\
             expected success, got: {}\n{}",
            cmd,
            output.status,
            String::from_utf8(output.stderr).map_err(|err| format!("{err:?}"))?
        ));
    }
    String::from_utf8(output.stdout).map_err(|err| format!("{err:?}"))
}

/// Finds the remote for rust-lang/rust.
/// For example for these remotes it will return `upstream`.
/// ```text
/// origin  https://github.com/pietroalbani/rust.git (fetch)
/// origin  https://github.com/pietroalbani/rust.git (push)
/// upstream        https://github.com/rust-lang/rust (fetch)
/// upstream        https://github.com/rust-lang/rust (push)
/// ```
pub fn get_rust_lang_rust_remote(
    config: &GitConfig<'_>,
    git_dir: Option<&Path>,
) -> Result<String, String> {
    let mut git = Command::new("git");
    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }
    git.args(["config", "--local", "--get-regex", "remote\\..*\\.url"]);
    let stdout = output_result(&mut git)?;

    let rust_lang_remote = stdout
        .lines()
        .find(|remote| remote.contains(config.git_repository))
        .ok_or_else(|| format!("{} remote not found", config.git_repository))?;

    let remote_name =
        rust_lang_remote.split('.').nth(1).ok_or_else(|| "remote name not found".to_owned())?;
    Ok(remote_name.into())
}

pub fn rev_exists(rev: &str, git_dir: Option<&Path>) -> Result<bool, String> {
    let mut git = Command::new("git");
    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }
    git.args(["rev-parse", rev]);
    let output = git.output().map_err(|err| format!("{err:?}"))?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(128) => Ok(false),
        None => Err(format!(
            "git didn't exit properly: {}",
            String::from_utf8(output.stderr).map_err(|err| format!("{err:?}"))?
        )),
        Some(code) => Err(format!(
            "git command exited with status code: {code}: {}",
            String::from_utf8(output.stderr).map_err(|err| format!("{err:?}"))?
        )),
    }
}

/// Returns the master branch from which we can take diffs to see changes.
/// This will usually be rust-lang/rust master, but sometimes this might not exist.
/// This could be because the user is updating their forked master branch using the GitHub UI
/// and therefore doesn't need an upstream master branch checked out.
/// We will then fall back to origin/master in the hope that at least this exists.
pub fn updated_master_branch(
    config: &GitConfig<'_>,
    git_dir: Option<&Path>,
) -> Result<String, String> {
    let upstream_remote = get_rust_lang_rust_remote(config, git_dir)?;
    let branch = config.nightly_branch;
    for upstream_master in [format!("{upstream_remote}/{branch}"), format!("origin/{branch}")] {
        if rev_exists(&upstream_master, git_dir)? {
            return Ok(upstream_master);
        }
    }

    Err("Cannot find any suitable upstream master branch".to_owned())
}

/// Finds the nearest merge commit by comparing the local `HEAD` with the upstream branch's state.
/// To work correctly, the upstream remote must be properly configured using `git remote add <name> <url>`.
/// In most cases `get_closest_merge_commit` is the function you are looking for as it doesn't require remote
/// to be configured.
fn git_upstream_merge_base(
    config: &GitConfig<'_>,
    git_dir: Option<&Path>,
) -> Result<String, String> {
    let updated_master = updated_master_branch(config, git_dir)?;
    let mut git = Command::new("git");
    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }
    Ok(output_result(git.arg("merge-base").arg(&updated_master).arg("HEAD"))?.trim().to_owned())
}

/// Searches for the nearest merge commit in the repository that also exists upstream.
///
/// It looks for the most recent commit made by the merge bot by matching the author's email
/// address with the merge bot's email.
pub fn get_closest_merge_commit(
    git_dir: Option<&Path>,
    config: &GitConfig<'_>,
    target_paths: &[PathBuf],
) -> Result<String, String> {
    let mut git = Command::new("git");

    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }

    let channel = include_str!("../../ci/channel").trim();

    let merge_base = {
        if CiEnv::is_ci() &&
            // FIXME: When running on rust-lang managed CI and it's not a nightly build,
            // `git_upstream_merge_base` fails with an error message similar to this:
            // ```
            //    called `Result::unwrap()` on an `Err` value: "command did not execute successfully:
            //    cd \"/checkout\" && \"git\" \"merge-base\" \"origin/master\" \"HEAD\"\nexpected success, got: exit status: 1\n"
            // ```
            // Investigate and resolve this issue instead of skipping it like this.
            (channel == "nightly" || !CiEnv::is_rust_lang_managed_ci_job())
        {
            git_upstream_merge_base(config, git_dir).unwrap()
        } else {
            // For non-CI environments, ignore rust-lang/rust upstream as it usually gets
            // outdated very quickly.
            "HEAD".to_string()
        }
    };

    git.args([
        "rev-list",
        &format!("--author={}", config.git_merge_commit_email),
        "-n1",
        "--first-parent",
        &merge_base,
    ]);

    if !target_paths.is_empty() {
        git.arg("--").args(target_paths);
    }

    Ok(output_result(&mut git)?.trim().to_owned())
}

/// Represents the result of checking whether a set of paths
/// have been modified locally or not.
#[derive(PartialEq, Debug)]
pub enum PathFreshness {
    /// Artifacts should be downloaded from this upstream commit,
    /// there are no local modifications.
    LastModifiedUpstream { upstream: String },
    /// There are local modifications to a certain set of paths.
    /// "Local" essentially means "not-upstream" here.
    /// `upstream` is the latest upstream merge commit that made modifications to the
    /// set of paths.
    HasLocalModifications { upstream: String },
}

/// This function figures out if a set of paths was last modified upstream or
/// if there are some local modifications made to them.
///
/// It can be used to figure out if we should download artifacts from CI or rather
/// build them locally.
///
/// `target_paths` should be a non-empty slice of paths (relative to `git_dir` or the
/// current working directory) whose modifications would invalidate the artifact.
/// Each path can also be a negative match, i.e. `:!foo`. This matches changes outside
/// the `foo` directory.
///
/// The function behaves differently in CI and outside CI.
///
/// - Outside CI, we want to find out if `target_paths` were modified in some local commit on
/// top of the local master branch.
/// If not, we try to find the most recent upstream commit (which we assume are commits
/// made by bors) that modified `target_paths`.
/// We don't want to simply take the latest master commit to avoid changing the output of
/// this function frequently after rebasing on the latest master branch even if `target_paths`
/// were not modified upstream in the meantime. In that case we would be redownloading CI
/// artifacts unnecessarily.
///
/// - In CI, we always fetch only a single parent merge commit, so we do not have access
/// to the full git history.
/// Luckily, we only need to distinguish between two situations. The first is that the current
/// PR made modifications to `target_paths`. If not, then we simply take the latest upstream
/// commit, because on CI there is no need to avoid redownloading.
pub fn check_path_modifications(
    git_dir: Option<&Path>,
    config: &GitConfig<'_>,
    target_paths: &[&str],
    ci_env: CiEnv,
) -> Result<PathFreshness, String> {
    assert!(!target_paths.is_empty());
    for path in target_paths {
        assert!(Path::new(path.trim_start_matches(":!")).is_relative());
    }

    let upstream_sha = if matches!(ci_env, CiEnv::GitHubActions) {
        // Here the situation is different for PR CI and try/auto CI.
        // For PR CI, we have the following history:
        // <merge commit made by GitHub>
        // 1-N PR commits
        // upstream merge commit made by bors
        //
        // For try/auto CI, we have the following history:
        // <**non-upstream** merge commit made by bors>
        // 1-N PR commits
        // upstream merge commit made by bors
        //
        // But on both cases, HEAD should be a merge commit.
        // So if HEAD contains modifications of `target_paths`, our PR has modified
        // them. If not, we can use the only available upstream commit for downloading
        // artifacts.

        // Do not include HEAD, as it is never an upstream commit
        get_closest_upstream_commit(git_dir, config, ci_env)?
    } else {
        // Outside CI, we have to find the most recent upstream commit that
        // modified the set of paths, to have an upstream reference.
        let upstream_sha = get_latest_commit_that_modified_files(
            git_dir,
            target_paths,
            config.git_merge_commit_email,
        )?;
        let Some(upstream_sha) = upstream_sha else {
            eprintln!("No upstream commit that modified paths {target_paths:?} found.");
            eprintln!("Try to fetch more upstream history.");
            return Err("No upstream commit with modifications found".to_string());
        };
        upstream_sha
    };

    // For local environments, we want to find out if something has changed
    // from the latest upstream commit.
    // However, that should be equivalent to checking if something has changed
    // from the latest upstream commit *that modified `target_paths`*, and
    // with this approach we do not need to invoke git an additional time.
    if has_changed_since(git_dir, &upstream_sha, target_paths) {
        Ok(PathFreshness::HasLocalModifications { upstream: upstream_sha })
    } else {
        Ok(PathFreshness::LastModifiedUpstream { upstream: upstream_sha })
    }
}

/// Returns true if any of the passed `paths` have changed since the `base` commit.
fn has_changed_since(git_dir: Option<&Path>, base: &str, paths: &[&str]) -> bool {
    let mut git = Command::new("git");

    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }

    git.args(["diff-index", "--quiet", base, "--"]).args(paths);

    // Exit code 0 => no changes
    // Exit code 1 => some changes were detected
    !git.status().expect("cannot run git diff-index").success()
}

/// Returns the latest commit that modified `target_paths`, or `None` if no such commit was found.
/// If `author` is `Some`, only considers commits made by that author.
fn get_latest_commit_that_modified_files(
    git_dir: Option<&Path>,
    target_paths: &[&str],
    author: &str,
) -> Result<Option<String>, String> {
    let mut git = Command::new("git");

    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }

    git.args(["rev-list", "-n1", "--first-parent", "HEAD", "--author", author]);

    if !target_paths.is_empty() {
        git.arg("--").args(target_paths);
    }
    let output = output_result(&mut git)?.trim().to_owned();
    if output.is_empty() { Ok(None) } else { Ok(Some(output)) }
}

/// Returns the most recent commit found in the local history that should definitely
/// exist upstream. We identify upstream commits by the e-mail of the commit author.
///
/// If `include_head` is false, the HEAD (current) commit will be ignored and only
/// its parents will be searched. This is useful for try/auto CI, where HEAD is
/// actually a commit made by bors, although it is not upstream yet.
fn get_closest_upstream_commit(
    git_dir: Option<&Path>,
    config: &GitConfig<'_>,
    env: CiEnv,
) -> Result<String, String> {
    let mut git = Command::new("git");

    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }

    let base = match env {
        CiEnv::None => "HEAD",
        CiEnv::GitHubActions => {
            // On CI, we always have a merge commit at the tip.
            // We thus skip it, because although it can be creatd by
            // `config.git_merge_commit_email`, it should not be upstream.
            "HEAD^1"
        }
    };
    git.args([
        "rev-list",
        &format!("--author={}", config.git_merge_commit_email),
        "-n1",
        "--first-parent",
        &base,
    ]);

    Ok(output_result(&mut git)?.trim().to_owned())
}

/// Returns the files that have been modified in the current branch compared to the master branch.
/// This includes committed changes, uncommitted changes, and changes that are not even staged.
///
/// The `extensions` parameter can be used to filter the files by their extension.
/// Does not include removed files.
/// If `extensions` is empty, all files will be returned.
pub fn get_git_modified_files(
    config: &GitConfig<'_>,
    git_dir: Option<&Path>,
    extensions: &[&str],
) -> Result<Vec<String>, String> {
    let merge_base = get_closest_upstream_commit(git_dir, config, CiEnv::None)?;

    let mut git = Command::new("git");
    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }
    let files = output_result(git.args(["diff-index", "--name-status", merge_base.trim()]))?
        .lines()
        .filter_map(|f| {
            let (status, name) = f.trim().split_once(char::is_whitespace).unwrap();
            if status == "D" {
                None
            } else if Path::new(name).extension().map_or(true, |ext| {
                extensions.is_empty() || extensions.contains(&ext.to_str().unwrap())
            }) {
                Some(name.to_owned())
            } else {
                None
            }
        })
        .collect();
    Ok(files)
}

/// Returns the files that haven't been added to git yet.
pub fn get_git_untracked_files(
    config: &GitConfig<'_>,
    git_dir: Option<&Path>,
) -> Result<Option<Vec<String>>, String> {
    let Ok(_updated_master) = updated_master_branch(config, git_dir) else {
        return Ok(None);
    };
    let mut git = Command::new("git");
    if let Some(git_dir) = git_dir {
        git.current_dir(git_dir);
    }

    let files = output_result(git.arg("ls-files").arg("--others").arg("--exclude-standard"))?
        .lines()
        .map(|s| s.trim().to_owned())
        .collect();
    Ok(Some(files))
}
