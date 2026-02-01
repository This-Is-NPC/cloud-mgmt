use crate::adapters::workspace_repository::FsWorkspaceRepository;
use crate::ports::ScriptRepository;
use std::error::Error;
use std::path::PathBuf;

pub fn run(scripts_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let repo = FsWorkspaceRepository::new(scripts_dir.clone());
    let mut scripts = repo.list_scripts_recursive()?;
    scripts.sort();

    println!("Scripts folder: {}", scripts_dir.display());
    if scripts.is_empty() {
        println!("(no scripts found)");
        return Ok(());
    }

    for script in scripts {
        let display_path = script
            .strip_prefix(&scripts_dir)
            .unwrap_or(&script)
            .to_string_lossy();
        println!(" - {}", display_path);
    }

    Ok(())
}
