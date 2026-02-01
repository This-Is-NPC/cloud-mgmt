use crate::domain::Schema;
use crate::error::AppResult;
use crate::ports::{ScriptRepository, ScriptRunOutput, ScriptRunner, WorkspaceEntry};
use std::io;
use std::path::Path;

pub struct ScriptService {
    repo: Box<dyn ScriptRepository>,
    runner: Box<dyn ScriptRunner>,
}

impl ScriptService {
    pub fn new(repo: Box<dyn ScriptRepository>, runner: Box<dyn ScriptRunner>) -> Self {
        Self { repo, runner }
    }

    pub fn list_entries(&self, dir: &Path) -> io::Result<Vec<WorkspaceEntry>> {
        self.repo.list_entries(dir)
    }

    pub fn load_schema(&self, script: &Path) -> AppResult<Schema> {
        self.repo.read_schema(script)
    }

    pub fn run_script(&self, script: &Path, args: &[String]) -> AppResult<ScriptRunOutput> {
        self.runner.run(script, args)
    }
}
