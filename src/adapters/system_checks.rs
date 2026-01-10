use std::error::Error;
use std::process::Command;

use crate::runtime::{powershell_program, python_program};

/// Check that a command is available and runs successfully.
fn ensure_command(
    program: &str,
    args: &[&str],
    not_found_hint: &str,
    failed_hint: &str,
) -> Result<(), Box<dyn Error>> {
    match Command::new(program).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let message = stderr.trim();
                if message.is_empty() {
                    Err(failed_hint.into())
                } else {
                    Err(format!("{}: {}", failed_hint, message).into())
                }
            }
        }
        Err(err) => Err(format!("{}: {}", not_found_hint, err).into()),
    }
}

#[cfg(windows)]
pub(crate) fn ensure_git_installed() -> Result<(), Box<dyn Error>> {
    ensure_command(
        "git",
        &["--version"],
        "Git not found in PATH. Install Git for Windows (includes bash)",
        "Git found, but `git --version` failed",
    )
}

#[cfg(not(windows))]
pub(crate) fn ensure_git_installed() -> Result<(), Box<dyn Error>> {
    ensure_command(
        "git",
        &["--version"],
        "Git not found in PATH. Install Git and ensure it is in PATH",
        "Git found, but `git --version` failed",
    )
}

#[cfg(windows)]
pub(crate) fn ensure_bash_installed() -> Result<(), Box<dyn Error>> {
    ensure_command(
        "bash",
        &["--version"],
        "Bash not found in PATH. Install Git for Windows or add bash.exe to PATH",
        "Bash found, but `bash --version` failed",
    )
}

#[cfg(not(windows))]
pub(crate) fn ensure_bash_installed() -> Result<(), Box<dyn Error>> {
    ensure_command(
        "bash",
        &["--version"],
        "Bash not found in PATH. Install bash and ensure it is in PATH",
        "Bash found, but `bash --version` failed",
    )
}

pub(crate) fn ensure_jq_installed() -> Result<(), Box<dyn Error>> {
    ensure_command(
        "jq",
        &["--version"],
        "jq not found in PATH. Install jq and ensure it is in PATH",
        "jq found, but `jq --version` failed",
    )
}

pub(crate) fn ensure_powershell_installed() -> Result<(), Box<dyn Error>> {
    let program = powershell_program();
    ensure_command(
        program,
        &["-NoProfile", "-Command", "$PSVersionTable.PSVersion"],
        &format!(
            "{} not found in PATH. Install PowerShell and ensure it is in PATH",
            program
        ),
        &format!("{} found, but PowerShell check failed", program),
    )
}

pub(crate) fn ensure_python_installed() -> Result<(), Box<dyn Error>> {
    let program = python_program();
    ensure_command(
        program,
        &["--version"],
        &format!(
            "{} not found in PATH. Install Python and ensure it is in PATH",
            program
        ),
        &format!("{} found, but `--version` failed", program),
    )
}
