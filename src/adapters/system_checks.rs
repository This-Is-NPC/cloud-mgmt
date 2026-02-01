use std::process::Command;

use crate::error::ScriptError;
use crate::runtime::{powershell_program, python_program};

/// Check that a command is available and runs successfully.
fn ensure_command(program: &str, args: &[&str], not_found_hint: &str) -> Result<(), ScriptError> {
    match Command::new(program).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let message = stderr.trim().to_string();
                Err(ScriptError::DependencyCheckFailed {
                    name: program.to_string(),
                    message: if message.is_empty() {
                        "check failed".to_string()
                    } else {
                        message
                    },
                })
            }
        }
        Err(_) => Err(ScriptError::DependencyMissing {
            name: program.to_string(),
            hint: not_found_hint.to_string(),
        }),
    }
}

#[cfg(windows)]
pub(crate) fn ensure_git_installed() -> Result<(), ScriptError> {
    ensure_command(
        "git",
        &["--version"],
        "Install Git for Windows (includes bash)",
    )
}

#[cfg(not(windows))]
pub(crate) fn ensure_git_installed() -> Result<(), ScriptError> {
    ensure_command(
        "git",
        &["--version"],
        "Install Git and ensure it is in PATH",
    )
}

#[cfg(windows)]
pub(crate) fn ensure_bash_installed() -> Result<(), ScriptError> {
    ensure_command(
        "bash",
        &["--version"],
        "Install Git for Windows or add bash.exe to PATH",
    )
}

#[cfg(not(windows))]
pub(crate) fn ensure_bash_installed() -> Result<(), ScriptError> {
    ensure_command(
        "bash",
        &["--version"],
        "Install bash and ensure it is in PATH",
    )
}

pub(crate) fn ensure_jq_installed() -> Result<(), ScriptError> {
    ensure_command("jq", &["--version"], "Install jq and ensure it is in PATH")
}

pub(crate) fn ensure_powershell_installed() -> Result<(), ScriptError> {
    let program = powershell_program();
    ensure_command(
        program,
        &["-NoProfile", "-Command", "$PSVersionTable.PSVersion"],
        &format!("Install PowerShell and ensure {} is in PATH", program),
    )
}

pub(crate) fn ensure_python_installed() -> Result<(), ScriptError> {
    let program = python_program();
    ensure_command(
        program,
        &["--version"],
        &format!("Install Python and ensure {} is in PATH", program),
    )
}
