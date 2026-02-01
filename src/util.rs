use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Set executable permissions on Unix systems (no-op on Windows).
#[cfg(not(windows))]
pub fn set_executable_permissions(path: &Path) -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
}

#[cfg(windows)]
pub fn set_executable_permissions(_path: &Path) -> Result<(), Box<dyn Error>> {
    Ok(())
}

/// Quote a string for use in PowerShell commands.
pub fn ps_quote(input: &str) -> String {
    format!("'{}'", input.replace('\'', "''"))
}

/// Read a directory, returning an empty list if missing.
pub fn read_dir_or_empty(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    match fs::read_dir(dir) {
        Ok(entries) => entries.collect(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(err),
    }
}

/// Read a file, returning None if missing.
pub fn read_file_if_exists(path: &Path) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// RAII guard that removes a temporary directory when dropped.
pub struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ps_quote_simple() {
        assert_eq!(ps_quote("hello"), "'hello'");
    }

    #[test]
    fn test_ps_quote_with_single_quote() {
        assert_eq!(ps_quote("it's"), "'it''s'");
    }

    #[test]
    fn test_ps_quote_empty() {
        assert_eq!(ps_quote(""), "''");
    }
}
