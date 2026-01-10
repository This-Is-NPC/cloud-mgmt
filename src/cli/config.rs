use crate::app_meta;
use crate::workspace::Workspace;
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub struct ConfigOptions {
    pub scripts_dir: PathBuf,
}

pub fn print_help() {
    println!(
        "Usage: omakure config\n\n\
Aliases:\n\
  env\n\n\
Notes:\n\
  Prints resolved workspace paths and environment overrides.\n\n\
Environment:\n\
  OMAKURE_SCRIPTS_DIR       Scripts directory override\n\
  OMAKURE_REPO              Default repo for update\n\
  REPO                      Repo override for update\n\
  VERSION                   Version override for update\n\
  OVERTURE_SCRIPTS_DIR      Legacy scripts directory override\n\
  OVERTURE_REPO             Legacy repo override\n\
  CLOUD_MGMT_SCRIPTS_DIR    Legacy scripts directory override\n\
  CLOUD_MGMT_REPO           Legacy repo override"
    );
}

pub fn parse_args(args: &[String], scripts_dir: PathBuf) -> Result<ConfigOptions, Box<dyn Error>> {
    if !args.is_empty() {
        return Err("config does not accept arguments".into());
    }
    Ok(ConfigOptions { scripts_dir })
}

pub fn run(options: ConfigOptions) -> Result<(), Box<dyn Error>> {
    let exe = env::current_exe()?;
    let workspace = Workspace::new(options.scripts_dir);
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
