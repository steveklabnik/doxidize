use std::fmt;
use std::path::Path;
use std::process::{self, Command};

use Result;

#[derive(Debug, Fail)]
pub struct GitFailure {
    command_name: &'static str,
    output: process::Output,
}

// stdout/stderr need some massaging, so let's not derive Display, as this way is clearer.
impl fmt::Display for GitFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stdout = String::from_utf8_lossy(&self.output.stdout);
        let stderr = String::from_utf8_lossy(&self.output.stderr);

        write!(
            f,
            "An error occurred while running '{}'\n\nstdout:\n{}\n\nstderr:\n{}",
            self.command_name, stdout, stderr
        )
    }
}

pub fn init(git_dir: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .arg("init")
        .output()
        .expect("failed to execute git init");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git init",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn initialize_remote(git_dir: &Path, remote_name: &str, has_base_url: bool) -> Result<()> {
    let dot_dot = if has_base_url {
        "../../../../.git"
    } else {
        "../../../.git"
    };

    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["remote", "add", remote_name, dot_dot])
        .output()
        .expect("failed to execute git init");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git remote add",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn reset_to_remote_head(git_dir: &Path, remote_name: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["fetch", remote_name])
        .output()
        .expect("failed to execute git fetch");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git fetch",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn head_revision(git_dir: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("failed to execute git rev-parse");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git rev-parse",
            output: output,
        }.into());
    }

    // we need to strip off the \n
    let mut output = output.stdout;
    output.pop();

    Ok(String::from_utf8(output)?)
}

pub fn add_all(git_dir: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["add", "."])
        .output()
        .expect("failed to execute git add");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git add",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn commit(git_dir: &Path, revision: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["commit", "-m"])
        .arg(format!("\"rebuild pages from {}\"", revision))
        .output()
        .expect("failed to execute git commit");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git commit",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn sync_pages_branch(git_dir: &Path, remote_name: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .arg("push")
        .arg(remote_name)
        .arg("HEAD:gh-pages")
        .output()
        .expect("failed to execute git push");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git push",
            output: output,
        }.into());
    }

    Ok(())
}

pub fn push(git_dir: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(git_dir.as_os_str())
        .args(&["push", "origin", "gh-pages"])
        .output()
        .expect("failed to execute git push");

    if !output.status.success() {
        return Err(GitFailure {
            command_name: "git push",
            output: output,
        }.into());
    }

    Ok(())
}
