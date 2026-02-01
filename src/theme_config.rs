use crate::adapters::tui::theme::{theme_file_path, BUILTIN_THEMES};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct ThemeLayout {
    pub config_dir: PathBuf,
    pub themes_dir: PathBuf,
    pub config_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ThemeConfigFile {
    theme: Option<ThemeConfig>,
}

#[derive(Debug, Deserialize)]
struct ThemeConfig {
    name: Option<String>,
}

pub(crate) fn ensure_theme_layout() -> Result<ThemeLayout, Box<dyn Error>> {
    let Some(config_dir) = config_dir() else {
        return Err("Unable to resolve config directory".into());
    };
    let themes_dir = config_dir.join("themes");
    let config_path = config_dir.join("config.toml");

    fs::create_dir_all(&themes_dir)?;
    ensure_builtin_themes(&themes_dir)?;

    if !config_path.exists() {
        write_global_theme(&config_path, "default")?;
    }

    Ok(ThemeLayout {
        config_dir,
        themes_dir,
        config_path,
    })
}

pub(crate) fn load_theme_name(path: &Path) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    let config: ThemeConfigFile = toml::from_str(&contents).ok()?;
    config.theme.and_then(|theme| theme.name)
}

pub(crate) fn write_global_theme(path: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    let mut value = if path.exists() {
        let contents = fs::read_to_string(path)?;
        toml::from_str::<toml::Value>(&contents)?
    } else {
        toml::Value::Table(toml::value::Table::new())
    };

    let table = value
        .as_table_mut()
        .ok_or_else(|| "Config root is not a table".to_string())?;
    let theme_value = table
        .entry("theme".to_string())
        .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));
    match theme_value {
        toml::Value::Table(theme_table) => {
            theme_table.insert("name".to_string(), toml::Value::String(name.to_string()));
        }
        _ => {
            let mut theme_table = toml::value::Table::new();
            theme_table.insert("name".to_string(), toml::Value::String(name.to_string()));
            *theme_value = toml::Value::Table(theme_table);
        }
    }

    let output = toml::to_string_pretty(&value)?;
    fs::write(path, output)?;
    Ok(())
}

fn ensure_builtin_themes(themes_dir: &Path) -> Result<(), Box<dyn Error>> {
    for theme in BUILTIN_THEMES {
        let path = theme_file_path(themes_dir, theme.name);
        if path.exists() {
            continue;
        }
        fs::write(path, theme.contents)?;
    }
    Ok(())
}

fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("omakure"))
}
