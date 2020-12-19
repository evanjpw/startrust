use thiserror::Error;

#[derive(Debug, Error)]
pub enum StarTrustError {
    #[error("IoError: {0}")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for StarTrustError {
    fn from(value: std::io::Error) -> Self {
        StarTrustError::IoError(value)
    }
}
