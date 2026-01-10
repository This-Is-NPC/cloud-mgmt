use crate::adapters::workspace_repository::FsWorkspaceRepository;
use crate::ports::ScriptRepository;
use std::error::Error;
use std::path::PathBuf;

use super::ENV_HELP;

pub struct ListOptions {
    pub scripts_dir: PathBuf,
}

pub fn print_help() {
    println!(
        "Usage: omakure scripts\n\n\
Notes:\n\
  Lists scripts recursively (workspace root and .omaken).\n\n\
{ENV_HELP}"
    );
}

pub fn parse_args(args: &[String], scripts_dir: PathBuf) -> Result<ListOptions, Box<dyn Error>> {
    if !args.is_empty() {
        return Err("scripts does not accept arguments".into());
    }
    Ok(ListOptions { scripts_dir })
}

pub fn run(options: ListOptions) -> Result<(), Box<dyn Error>> {
    let repo = FsWorkspaceRepository::new(options.scripts_dir.clone());
    let mut scripts = repo.list_scripts_recursive()?;
    scripts.sort();

    println!("Scripts folder: {}", options.scripts_dir.display());
    if scripts.is_empty() {
        println!("(no scripts found)");
        return Ok(());
    }

    for script in scripts {
        let display_path = script
            .strip_prefix(&options.scripts_dir)
            .unwrap_or(&script)
            .to_string_lossy();
        println!(" - {}", display_path);
    }

    Ok(())
}
