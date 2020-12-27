use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StarTrustError {
    #[error("IoError: {0}")]
    IoError(std::io::Error),
    #[error("GameStateError: {0}")]
    GameStateError(String),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(std::num::ParseFloatError),
    #[error("ParseIntError: {0}")]
    ParseIntError(std::num::ParseIntError),
    #[error("TryFromPrimitiveError")]
    TryFromPrimitiveError(String),
    #[error("GeneralError: {0}")]
    GeneralError(String),
}

impl From<std::io::Error> for StarTrustError {
    fn from(value: std::io::Error) -> Self {
        StarTrustError::IoError(value)
    }
}

impl From<std::num::ParseFloatError> for StarTrustError {
    fn from(value: std::num::ParseFloatError) -> Self {
        StarTrustError::ParseFloatError(value)
    }
}

impl From<std::num::ParseIntError> for StarTrustError {
    fn from(value: std::num::ParseIntError) -> Self {
        StarTrustError::ParseIntError(value)
    }
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for StarTrustError {
    fn from(value: TryFromPrimitiveError<T>) -> Self {
        StarTrustError::TryFromPrimitiveError(format!("{}", value))
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
