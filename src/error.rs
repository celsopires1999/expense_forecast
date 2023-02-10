//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),
    /// For starter, to remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error("Entity Validation Error: '{0}'")]
    EntityValidationError(&'static str),

    #[error(transparent)]
    UUID(#[from] uuid::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
