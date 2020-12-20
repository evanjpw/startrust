use thiserror::Error;

#[derive(Debug, Error)]
pub enum StarTrustError {
    #[error("IoError: {0}")]
    IoError(std::io::Error),
    #[error("GameStateError: {0}")]
    GameStateError(String),
    #[error("GeneralError: {0}")]
    GeneralError(String),
}

impl From<std::io::Error> for StarTrustError {
    fn from(value: std::io::Error) -> Self {
        StarTrustError::IoError(value)
    }
}

pub type StResult<T> = std::result::Result<T, StarTrustError>;

// impl <T> From<std::io::Result<T>> for StResult<T> {
//     fn from(value: std::io::Result<T>) -> StResult<T> {
//         value.map_err(|e| {
//             let e = e.into();
//             e
//         })
//     }
// }
