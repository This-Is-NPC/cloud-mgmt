use crate::adapters::tui::theme::{
    builtin_theme_names, load_theme_from_builtin, load_theme_from_name, theme_file_path, Theme,
    ThemeVariant,
};
use crate::cli::args::{ThemeArgs, ThemeCommand};
use ratatui::style::Color;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(_scripts_dir: PathBuf, args: ThemeArgs) -> Result<(), Box<dyn Error>> {
    match args.command {
        ThemeCommand::List => list_themes(),
        ThemeCommand::Set(args) => set_theme(&args.name),
        ThemeCommand::Preview(args) => preview_theme(&args.name),
        ThemeCommand::Path => print_paths(),
    }
}

fn list_themes() -> Result<(), Box<dyn Error>> {
    let mut builtin = builtin_theme_names();
    builtin.sort();
    println!("Built-in themes:");
    for name in builtin {
        println!(" - {}", name);
    }

    let theme_dir = themes_dir()?;
    let user_themes = if theme_dir.is_dir() {
        read_theme_names(&theme_dir)?
    } else {
        Vec::new()
    };

    println!("\nUser themes ({})", theme_dir.display());
    if user_themes.is_empty() {
        println!(" - (none)");
    } else {
        for name in user_themes {
            println!(" - {}", name);
        }
    }

    Ok(())
}

fn set_theme(name: &str) -> Result<(), Box<dyn Error>> {
    let theme_dir = themes_dir()?;
    ensure_theme_exists(name, &theme_dir)?;

    let config_path = config_path()?;
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_config_theme(&config_path, name)?;

    println!("Theme set to '{}' in {}", name, config_path.display());
    Ok(())
}

fn preview_theme(name: &str) -> Result<(), Box<dyn Error>> {
    let theme_dir = themes_dir()?;
    let theme = if let Some(theme) = load_theme_from_name(name, &theme_dir) {
        theme
    } else if let Some(theme) = load_theme_from_builtin(name) {
        theme
    } else {
        return Err(format!("Theme not found: {}", name).into());
    };

    print_theme_preview(name, &theme);
    Ok(())
}

fn print_paths() -> Result<(), Box<dyn Error>> {
    let config_dir = config_dir()?;
    let theme_dir = themes_dir()?;
    let config_path = config_path()?;
    println!("Config dir: {}", config_dir.display());
    println!("Themes dir: {}", theme_dir.display());
    println!("Config file: {}", config_path.display());
    Ok(())
}

fn print_theme_preview(name: &str, theme: &Theme) {
    println!("Theme: {} ({})", theme.meta.name, name);
    if let Some(author) = theme.meta.author.as_deref() {
        println!("Author: {}", author);
    }
    if let Some(variant) = theme.meta.variant {
        println!("Variant: {}", format_variant(variant));
    }

    println!(
        "Brand: {} -> {}",
        format_color(theme.brand.gradient_start.color()),
        format_color(theme.brand.gradient_end.color())
    );
    println!("Accent: {}", format_color(theme.brand.accent.color()));
    println!(
        "Semantic: success {}, error {}, warning {}, info {}",
        format_color(theme.semantic.success.color()),
        format_color(theme.semantic.error.color()),
        format_color(theme.semantic.warning.color()),
        format_color(theme.semantic.info.color())
    );
    println!(
        "UI text: primary {}, secondary {}, muted {}",
        format_color(theme.ui.text_primary.color()),
        format_color(theme.ui.text_secondary.color()),
        format_color(theme.ui.text_muted.color())
    );
    println!(
        "UI borders: active {}, inactive {}",
        format_color(theme.ui.border_active.color()),
        format_color(theme.ui.border_inactive.color())
    );
    println!("Selection: {}", format_color(theme.ui.selection_fg.color()));
    println!(
        "Status: ok {}, fail {}, error {}",
        format_color(theme.status.ok.color()),
        format_color(theme.status.fail.color()),
        format_color(theme.status.error.color())
    );
}

fn format_variant(variant: ThemeVariant) -> &'static str {
    match variant {
        ThemeVariant::Dark => "dark",
        ThemeVariant::Light => "light",
    }
}

fn format_color(color: Color) -> String {
    match color {
        Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
        _ => format!("{:?}", color),
    }
}

fn ensure_theme_exists(name: &str, theme_dir: &Path) -> Result<(), Box<dyn Error>> {
    let is_builtin = builtin_theme_names().iter().any(|builtin| *builtin == name);
    if is_builtin {
        return Ok(());
    }

    let theme_path = theme_file_path(theme_dir, name);
    if theme_path.is_file() {
        return Ok(());
    }

    Err(format!("Theme not found: {}", name).into())
}

fn read_theme_names(theme_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(theme_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            names.push(stem.to_string());
        }
    }
    names.sort();
    names.dedup();
    Ok(names)
}

fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
    dirs::config_dir()
        .map(|dir| dir.join("omakure"))
        .ok_or_else(|| "Unable to resolve config directory".into())
}

fn themes_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("themes"))
}

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("config.toml"))
}

fn write_config_theme(path: &Path, name: &str) -> Result<(), Box<dyn Error>> {
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
