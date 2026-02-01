use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::AppResult;

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub envs_dir: PathBuf,
    pub active: Option<String>,
    pub defaults: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnvFile {
    pub name: String,
}

pub type EnvPreview = Vec<(String, String)>;

pub trait EnvironmentRepository {
    fn list_env_files(&self) -> AppResult<Vec<EnvFile>>;
    fn load_environment_config(&self) -> AppResult<EnvironmentConfig>;
    fn set_active_env(&self, name: Option<&str>) -> AppResult<()>;
    fn load_env_preview(&self, path: &Path) -> AppResult<EnvPreview>;
    fn load_env_defaults(&self, path: &Path) -> AppResult<HashMap<String, String>>;
}
