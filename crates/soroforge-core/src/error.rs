use thiserror::Error;

#[derive(Debug, Error)]
pub enum SoroForgeError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Contract error: {0}")]
    Contract(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
