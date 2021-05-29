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

pub fn deserialize<T>(encoding: &[u8]) -> Result<T, DeserializeError>
where
    T: Deserialize,
{
    T::deserialize(encoding)
}
