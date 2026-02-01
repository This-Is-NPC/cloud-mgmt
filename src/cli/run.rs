use crate::adapters::script_runner::MultiScriptRunner;
use crate::adapters::workspace_repository::FsWorkspaceRepository;
use crate::cli::args::RunArgs;
use crate::history;
use crate::ports::ScriptRunOutput;
use crate::runtime::script_extensions;
use crate::use_cases::ScriptService;
use crate::workspace::Workspace;
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn run(scripts_dir: PathBuf, options: RunArgs) -> Result<(), Box<dyn Error>> {
    let workspace = Workspace::new(scripts_dir);
    workspace.ensure_layout()?;

    let script_path = resolve_script_path(&options.script, workspace.root())?;

    let repo = Box::new(FsWorkspaceRepository::new(workspace.root().to_path_buf()));
    let runner = Box::new(MultiScriptRunner::new());
    let service = ScriptService::new(repo, runner);

    let run_result = service.run_script(&script_path, &options.args);
    match run_result {
        Ok(output) => {
            let success = output.success;
            let exit_code = output.exit_code.unwrap_or(1);
            print_output(&output);
            let entry = history::success_entry(&workspace, &script_path, &options.args, output);
            let _ = history::record_entry(&workspace, &entry);
            if !success {
                std::process::exit(exit_code);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            let entry =
                history::error_entry(&workspace, &script_path, &options.args, err.to_string());
            let _ = history::record_entry(&workspace, &entry);
            return Err(Box::new(err));
        }
    }

    Ok(())
}

fn resolve_script_path(script: &str, scripts_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let has_separator = script.contains('/') || script.contains('\\');
    let path = PathBuf::from(script);

    if path.is_absolute() {
        return resolve_with_extensions(path);
    }

    if has_separator {
        return resolve_with_extensions(scripts_dir.join(path));
    }

    resolve_with_extensions(scripts_dir.join(script))
}

fn resolve_with_extensions(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if path.exists() {
        if path.is_file() {
            return Ok(path);
        }
        return Err(format!("Script is not a file: {}", path.display()).into());
    }
    if path.extension().is_some() {
        return Err(format!("Script not found: {}", path.display()).into());
    }
    for ext in script_extensions() {
        let mut candidate = path.clone();
        candidate.set_extension(ext);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(format!("Script not found: {}", path.display()).into())
}

fn print_output(output: &ScriptRunOutput) {
    if !output.stdout.trim().is_empty() {
        print!("{}", output.stdout);
        if !output.stdout.ends_with('\n') {
            println!();
        }
    }
    if !output.stderr.trim().is_empty() {
        eprint!("{}", output.stderr);
        if !output.stderr.ends_with('\n') {
            eprintln!();
        }
    }
}
