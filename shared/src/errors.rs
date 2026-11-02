use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoggingError {
    #[error("Conflicting logging arguments: --quiet, --verbose and --debug are mutually exclusive")]
    ConflictingArguments,
}
