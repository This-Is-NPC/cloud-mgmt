use crate::app_meta;
use crate::workspace::Workspace;
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn run(scripts_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let exe = env::current_exe()?;
    let workspace = Workspace::new(scripts_dir);
    println!("Version: {}", app_meta::APP_VERSION);
    println!("Binary: {}", exe.display());
    println!("Workspace root: {}", workspace.root().display());
    println!("Omaken dir: {}", workspace.omaken_dir().display());
    println!("History dir: {}", workspace.history_dir().display());
    println!("Workspace config: {}", workspace.config_path().display());
    println!("Environments dir: {}", workspace.envs_dir().display());
    println!(
        "Active environment file: {}",
        workspace.envs_active_path().display()
    );

    print_env_if_set("OMAKURE_SCRIPTS_DIR");
    print_env_if_set("OMAKURE_REPO");
    print_env_if_set("REPO");
    print_env_if_set("VERSION");
    print_env_if_set("OVERTURE_SCRIPTS_DIR");
    print_env_if_set("OVERTURE_REPO");
    print_env_if_set("CLOUD_MGMT_SCRIPTS_DIR");
    print_env_if_set("CLOUD_MGMT_REPO");

    Ok(())
}

fn print_env_if_set(name: &str) {
    if let Ok(value) = env::var(name) {
        println!("{}: {}", name, value);
    }
}
