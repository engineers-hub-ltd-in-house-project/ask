use thiserror::Error;

#[derive(Error, Debug)]
pub enum AskError {
    #[error("API authentication failed")]
    AuthenticationFailed,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Keyring error: {0}")]
    KeyringError(#[from] keyring::Error),

    #[error("Configuration file error: {0}")]
    ConfigFileError(#[from] confy::ConfyError),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Streaming error: {0}")]
    StreamError(String),

    #[error("Template error: {0}")]
    TemplateError(String),
}

pub type Result<T> = std::result::Result<T, AskError>;
