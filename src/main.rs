mod adapters;
mod app_meta;
mod cli;
mod domain;
mod error;
mod history;
mod lua_widget;
mod ports;
mod runtime;
mod search_index;
mod use_cases;
mod util;
mod workspace;

use adapters::script_runner::MultiScriptRunner;
use adapters::tui;
use adapters::workspace_repository::FsWorkspaceRepository;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use use_cases::ScriptService;
use workspace::Workspace;

fn scripts_dir_for(name: &str) -> PathBuf {
    #[cfg(windows)]
    {
        if let Some(documents) = windows_documents_dir() {
            return documents.join(name);
        }

        if let Ok(user_profile) = env::var("USERPROFILE") {
            return PathBuf::from(user_profile).join("Documents").join(name);
        }
    }

    #[cfg(not(windows))]
    {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join("Documents").join(name);
        }
    }

    PathBuf::from("scripts")
}

#[cfg(windows)]
fn windows_documents_dir() -> Option<PathBuf> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let subkeys = [
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders",
    ];

    for subkey in subkeys {
        if let Ok(key) = hkcu.open_subkey(subkey) {
            if let Ok(value) = key.get_value::<String, _>("Personal") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    return Some(PathBuf::from(expand_windows_env_vars(trimmed)));
                }
            }
        }
    }

    None
}

#[cfg(windows)]
fn expand_windows_env_vars(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            output.push(ch);
            continue;
        }

        let mut name = String::new();
        let mut found_end = false;
        while let Some(next) = chars.next() {
            if next == '%' {
                found_end = true;
                break;
            }
            name.push(next);
        }

        if !found_end {
            output.push('%');
            output.push_str(&name);
            break;
        }

        if name.is_empty() {
            output.push('%');
            continue;
        }

        if let Ok(value) = env::var(&name) {
            output.push_str(&value);
        } else {
            output.push('%');
            output.push_str(&name);
            output.push('%');
        }
    }

    output
}

fn default_scripts_dir() -> PathBuf {
    scripts_dir_for("omakure-scripts")
}

fn scripts_dir() -> PathBuf {
    if let Ok(dir) = env::var("OMAKURE_SCRIPTS_DIR") {
        return PathBuf::from(dir);
    }

    if let Ok(dir) = env::var("OVERTURE_SCRIPTS_DIR") {
        return PathBuf::from(dir);
    }

    if let Ok(dir) = env::var("CLOUD_MGMT_SCRIPTS_DIR") {
        return PathBuf::from(dir);
    }

    if cfg!(debug_assertions) {
        let dev_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts");
        if dev_dir.is_dir() {
            return dev_dir;
        }
    }

    let default_dir = default_scripts_dir();
    if default_dir.is_dir() {
        return default_dir;
    }

    for legacy_dir in [
        scripts_dir_for("overture-scripts"),
        scripts_dir_for("cloud-mgmt-scripts"),
    ] {
        if legacy_dir.is_dir() {
            return legacy_dir;
        }
    }

    default_dir
}

fn print_help() {
    println!(
        "Usage: omakure [command]\n\n\
Commands:\n\
  update      Update omakure from GitHub Releases\n\
  uninstall   Remove the omakure binary\n\
  doctor      Check runtime dependencies and workspace\n\
  check       Alias for doctor\n\
  list        List Omaken flavors\n\
  install     Install an Omaken flavor\n\
  scripts     List available scripts\n\
  run         Run a script without the TUI\n\
  init        Create a new script template\n\
  config      Show resolved paths and env\n\
  env         Alias for config\n\
  completion  Generate shell completion\n\
\n\
Options:\n\
  -h, --help     Show this help\n\
  -V, --version  Show version"
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let scripts_dir = scripts_dir();

    if let Some(command) = args.next() {
        let remaining: Vec<String> = args.collect();

        match command.as_str() {
            "update" => {
                if cli::wants_help(&remaining) {
                    cli::update::print_help();
                    return Ok(());
                }
                let options = cli::update::parse_args(&remaining, scripts_dir)?;
                cli::update::run(options)?;
                return Ok(());
            }
            "uninstall" => {
                if cli::wants_help(&remaining) {
                    cli::uninstall::print_help();
                    return Ok(());
                }
                let options = cli::uninstall::parse_args(&remaining, scripts_dir)?;
                cli::uninstall::run(options)?;
                return Ok(());
            }
            "doctor" | "check" => {
                if cli::wants_help(&remaining) {
                    cli::doctor::print_help();
                    return Ok(());
                }
                let options = cli::doctor::parse_args(&remaining, scripts_dir)?;
                cli::doctor::run(options)?;
                return Ok(());
            }
            "list" => {
                if cli::wants_help(&remaining) {
                    cli::omaken::print_list_help();
                    return Ok(());
                }
                let options = cli::omaken::parse_list_args(&remaining, scripts_dir)?;
                cli::omaken::run_list(options)?;
                return Ok(());
            }
            "install" => {
                if cli::wants_help(&remaining) {
                    cli::omaken::print_install_help();
                    return Ok(());
                }
                let options = cli::omaken::parse_install_args(&remaining, scripts_dir)?;
                cli::omaken::run_install(options)?;
                return Ok(());
            }
            "scripts" => {
                if cli::wants_help(&remaining) {
                    cli::list::print_help();
                    return Ok(());
                }
                let options = cli::list::parse_args(&remaining, scripts_dir)?;
                cli::list::run(options)?;
                return Ok(());
            }
            "run" => {
                if cli::run::wants_help(&remaining) {
                    cli::run::print_help();
                    return Ok(());
                }
                let options = cli::run::parse_args(&remaining, scripts_dir)?;
                cli::run::run(options)?;
                return Ok(());
            }
            "init" => {
                if cli::wants_help(&remaining) {
                    cli::init::print_help();
                    return Ok(());
                }
                let options = cli::init::parse_args(&remaining, scripts_dir)?;
                cli::init::run(options)?;
                return Ok(());
            }
            "config" | "env" => {
                if cli::wants_help(&remaining) {
                    cli::config::print_help();
                    return Ok(());
                }
                let options = cli::config::parse_args(&remaining, scripts_dir)?;
                cli::config::run(options)?;
                return Ok(());
            }
            "completion" => {
                if cli::wants_help(&remaining) {
                    cli::completion::print_help();
                    return Ok(());
                }
                let options = cli::completion::parse_args(&remaining)?;
                cli::completion::run(options)?;
                return Ok(());
            }
            "help" | "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "version" | "-V" | "--version" => {
                println!("omakure {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {}
        }
    }

    let workspace = Workspace::new(scripts_dir.clone());
    workspace.ensure_layout()?;

    let repo = Box::new(FsWorkspaceRepository::new(scripts_dir));
    let runner = Box::new(MultiScriptRunner::new());
    let service = ScriptService::new(repo, runner);

    let mut terminal = tui::setup_terminal()?;
    let app_result = tui::run_app(&mut terminal, &service, workspace);
    tui::restore_terminal(&mut terminal)?;
    app_result?;

    Ok(())
}
