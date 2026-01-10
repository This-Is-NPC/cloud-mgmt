use std::error::Error;
use std::fmt;
use std::io;

/// Application-specific error type for future use.
/// Currently the codebase uses `Box<dyn Error>` but can be
/// gradually migrated to use this type for better error handling.
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    /// I/O error from filesystem operations.
    Io(io::Error),
    /// Schema parsing or validation error.
    Schema(String),
    /// Script execution error.
    Script(String),
    /// Configuration error.
    Config(String),
    /// General application error.
    General(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::Schema(msg) => write!(f, "Schema error: {}", msg),
            AppError::Script(msg) => write!(f, "Script error: {}", msg),
            AppError::Config(msg) => write!(f, "Config error: {}", msg),
            AppError::General(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::General(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::General(msg.to_string())
    }
}

/// Result type alias using AppError.
#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err = AppError::Schema("invalid field".to_string());
        assert_eq!(format!("{}", err), "Schema error: invalid field");
    }

    #[test]
    fn test_app_error_from_string() {
        let err: AppError = "something went wrong".into();
        assert_eq!(format!("{}", err), "something went wrong");
    }

    #[test]
    fn test_app_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = AppError::from(io_err);
        assert!(matches!(err, AppError::Io(_)));
        assert!(format!("{}", err).contains("file not found"));
    }
}
