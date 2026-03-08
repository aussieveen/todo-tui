use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AppError {
    #[error("failed to load todo file: {0}")]
    Load(String),
    #[error("failed to save todo file: {0}")]
    Save(String),
}
