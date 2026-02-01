use std::path::Path;

use crate::error::AppResult;
use crate::ports::{EnvFile, EnvPreview, EnvironmentConfig, EnvironmentRepository};

pub struct EnvironmentService {
    repo: Box<dyn EnvironmentRepository>,
}

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
}
