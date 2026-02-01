use crate::adapters::system_checks::{
    ensure_bash_installed, ensure_git_installed, ensure_jq_installed, ensure_powershell_installed,
    ensure_python_installed,
};
use crate::error::{AppResult, ScriptError};
use crate::ports::{ScriptRunOutput, ScriptRunner};
use crate::runtime::{command_for_script, script_kind, ScriptKind};
use std::path::Path;

pub struct MultiScriptRunner;

impl MultiScriptRunner {
    pub fn new() -> Self {
        Self
    }
}

impl ScriptRunner for MultiScriptRunner {
    fn run(&self, script: &Path, args: &[String]) -> AppResult<ScriptRunOutput> {
        match script_kind(script).ok_or(ScriptError::UnsupportedType)? {
            ScriptKind::Bash => {
                ensure_git_installed()?;
                ensure_bash_installed()?;
                ensure_jq_installed()?;
            }
            ScriptKind::PowerShell => {
                ensure_powershell_installed()?;
            }
            ScriptKind::Python => {
                ensure_python_installed()?;
            }
        }

        let output = command_for_script(script)?.args(args).output()?;
        Ok(ScriptRunOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }
}
