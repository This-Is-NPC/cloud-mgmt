use std::collections::HashMap;
use std::path::Path;

use crate::error::AppResult;
use crate::ports::{EnvFile, EnvPreview, EnvironmentConfig, EnvironmentRepository};

#[allow(dead_code)]
pub struct EnvironmentService {
    repo: Box<dyn EnvironmentRepository>,
}

#[allow(dead_code)]
impl EnvironmentService {
    pub fn new(repo: Box<dyn EnvironmentRepository>) -> Self {
        Self { repo }
    }

    pub fn list_env_files(&self) -> AppResult<Vec<EnvFile>> {
        self.repo.list_env_files()
    }

    pub fn load_environment_config(&self) -> AppResult<EnvironmentConfig> {
        self.repo.load_environment_config()
    }

    pub fn set_active_env(&self, name: Option<&str>) -> AppResult<()> {
        self.repo.set_active_env(name)
    }

    pub fn load_env_preview(&self, path: &Path) -> AppResult<EnvPreview> {
        self.repo.load_env_preview(path)
    }

    pub fn load_env_defaults(&self, path: &Path) -> AppResult<HashMap<String, String>> {
        self.repo.load_env_defaults(path)
    }

    pub fn get_default(&self, path: &Path, field_name: &str) -> AppResult<Option<String>> {
        let defaults = self.load_env_defaults(path)?;
        Ok(defaults.get(field_name).cloned())
    }
}
