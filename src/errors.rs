use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevinitError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("File I/O error: {0}")]
    FileIOError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Kubernetes validation error: {0}")]
    K8sValidationError(String),

    #[error("CI/CD generation error: {0}")]
    CiCdGenerationError(String),

    #[error("Environment configuration error: {0}")]
    EnvConfigError(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Service generation error: {0}")]
    ServiceGenerationError(String),
}