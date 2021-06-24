use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("expected further data when decoding")]
    InputTooShort,
    #[error("invalid data for expected type")]
    InvalidInput,
}

pub trait Deserialize {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}
