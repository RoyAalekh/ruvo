use thiserror::Error;

#[derive(Error, Debug)]
pub enum VenvError {
    #[error("Environment with name {0} already exists")]
    EnvironmentExists(String),

    #[error("Environment {0} not found")]
    EnvironmentNotFound(String),

    #[error("Unsupported environment type: {0}")]
    UnsupportedEnvironmentType(String),

    #[error("Failed to create environment: {0}")]
    CreationError(String),

    #[error("Failed to read/write store data: {0}")]
    StoreError(String),

    #[error("Command execution failed: {0}")]
    CommandError(String),
}