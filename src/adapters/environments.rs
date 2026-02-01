use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppResult, EnvironmentError};
pub use crate::ports::{EnvFile, EnvironmentConfig};
use crate::ports::{EnvPreview, EnvironmentRepository};

pub struct FsEnvironmentRepository {
    envs_dir: PathBuf,
}

impl FsEnvironmentRepository {
    pub fn new<P: Into<PathBuf>>(envs_dir: P) -> Self {
        Self {
            envs_dir: envs_dir.into(),
        }
    }
}

impl EnvironmentRepository for FsEnvironmentRepository {
    fn list_env_files(&self) -> AppResult<Vec<EnvFile>> {
        let mut entries = Vec::new();
        let dir = match fs::read_dir(&self.envs_dir) {
            Ok(dir) => dir,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(entries);
            }
            Err(err) => {
                return Err(EnvironmentError::ReadFailed(format!(
                    "Failed to read environments dir {}: {}",
                    self.envs_dir.display(),
                    err
                ))
                .into());
            }
        };

        for entry in dir {
            let entry = entry.map_err(|err| EnvironmentError::ReadFailed(err.to_string()))?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = match path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name.to_string(),
                None => continue,
            };
            if name == "active" {
                continue;
            }
            entries.push(EnvFile { name });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    fn load_environment_config(&self) -> AppResult<EnvironmentConfig> {
        let active = load_active_env_name(&self.envs_dir)?;
        let defaults = if let Some(name) = &active {
            let path = self.envs_dir.join(name);
            if !path.is_file() {
                return Err(EnvironmentError::NotFound {
                    name: path.display().to_string(),
                }
                .into());
            }
            self.load_env_defaults(&path)?
        } else {
            HashMap::new()
        };

        Ok(EnvironmentConfig {
            envs_dir: self.envs_dir.clone(),
            active,
            defaults,
        })
    }

    fn set_active_env(&self, name: Option<&str>) -> AppResult<()> {
        fs::create_dir_all(&self.envs_dir).map_err(|err| {
            EnvironmentError::WriteFailed(format!(
                "Failed to create environments dir {}: {}",
                self.envs_dir.display(),
                err
            ))
        })?;
        let active_path = self.envs_dir.join("active");

        match name {
            Some(name) => {
                let candidate = self.envs_dir.join(name);
                if !candidate.is_file() {
                    return Err(EnvironmentError::NotFound {
                        name: candidate.display().to_string(),
                    }
                    .into());
                }
                fs::write(&active_path, format!("{}\n", name)).map_err(|err| {
                    EnvironmentError::WriteFailed(format!(
                        "Failed to write active environment {}: {}",
                        active_path.display(),
                        err
                    ))
                })?;
            }
            None => {
                if active_path.exists() {
                    fs::remove_file(&active_path).map_err(|err| {
                        EnvironmentError::WriteFailed(format!(
                            "Failed to clear active environment {}: {}",
                            active_path.display(),
                            err
                        ))
                    })?;
                }
            }
        }

        Ok(())
    }

    fn load_env_preview(&self, path: &Path) -> AppResult<EnvPreview> {
        let contents = fs::read_to_string(path).map_err(|err| {
            EnvironmentError::ReadFailed(format!(
                "Failed to read environment file {}: {}",
                path.display(),
                err
            ))
        })?;
        Ok(parse_env_preview(&contents))
    }

    fn load_env_defaults(&self, path: &Path) -> AppResult<HashMap<String, String>> {
        let contents = fs::read_to_string(path).map_err(|err| {
            EnvironmentError::ReadFailed(format!(
                "Failed to read environment file {}: {}",
                path.display(),
                err
            ))
        })?;
        Ok(parse_env_defaults(&contents))
    }
}

pub fn load_env_preview(path: &Path) -> Result<Vec<(String, String)>, String> {
    let repo = FsEnvironmentRepository::new(path.parent().unwrap_or(Path::new(".")));
    repo.load_env_preview(path).map_err(|err| err.to_string())
}

pub fn list_env_files(envs_dir: &Path) -> Result<Vec<EnvFile>, String> {
    let repo = FsEnvironmentRepository::new(envs_dir);
    repo.list_env_files().map_err(|err| err.to_string())
}

pub fn load_environment_config(envs_dir: &Path) -> Result<EnvironmentConfig, String> {
    let repo = FsEnvironmentRepository::new(envs_dir);
    repo.load_environment_config()
        .map_err(|err| err.to_string())
}

pub fn set_active_env(envs_dir: &Path, name: Option<&str>) -> Result<(), String> {
    let repo = FsEnvironmentRepository::new(envs_dir);
    repo.set_active_env(name).map_err(|err| err.to_string())
}

#[allow(dead_code)]
pub fn load_env_defaults(path: &Path) -> Result<HashMap<String, String>, String> {
    let repo = FsEnvironmentRepository::new(path.parent().unwrap_or(Path::new(".")));
    repo.load_env_defaults(path).map_err(|err| err.to_string())
}

fn load_active_env_name(envs_dir: &Path) -> AppResult<Option<String>> {
    let active_path = envs_dir.join("active");
    let contents = match fs::read_to_string(&active_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(EnvironmentError::ReadFailed(format!(
                "Failed to read active environment {}: {}",
                active_path.display(),
                err
            ))
            .into());
        }
    };

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        return Ok(Some(trimmed.to_string()));
    }

    Ok(None)
}

fn parse_env_preview(contents: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();

    for line in contents.lines() {
        let mut trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix("export ") {
            trimmed = stripped.trim();
        }

        let mut parts = trimmed.splitn(2, '=');
        let key = parts.next().unwrap_or("").trim();
        let raw_value = parts.next().unwrap_or("").trim();
        if key.is_empty() {
            continue;
        }
        let mut value = strip_quotes(raw_value).trim().to_string();
        if is_sensitive_key(key) && !value.is_empty() {
            value = "***".to_string();
        }
        entries.push((key.to_string(), value));
    }

    entries
}

fn parse_env_defaults(contents: &str) -> HashMap<String, String> {
    let mut defaults = HashMap::new();

    for line in contents.lines() {
        let mut trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix("export ") {
            trimmed = stripped.trim();
        }

        let mut parts = trimmed.splitn(2, '=');
        let key = parts.next().unwrap_or("").trim();
        let raw_value = parts.next().unwrap_or("").trim();
        if key.is_empty() {
            continue;
        }
        let value = strip_quotes(raw_value).trim();
        if value.is_empty() {
            continue;
        }
        defaults.insert(key.to_ascii_lowercase(), value.to_string());
    }

    defaults
}

fn strip_quotes(value: &str) -> &str {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0] as char;
        let last = trimmed.as_bytes()[trimmed.len() - 1] as char;
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return &trimmed[1..trimmed.len() - 1];
        }
    }
    trimmed
}

fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    let tokens = [
        "password", "secret", "token", "key", "api", "private", "cred",
    ];
    tokens.iter().any(|token| lower.contains(token))
}
