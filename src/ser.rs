use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub enum SerializeError {}

pub trait Serialize {
    /// Append an encoding of `self` to the `buffer`.
    /// Return the number of bytes written.
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError>;
}

pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    let mut result = vec![];
    value.serialize(&mut result)?;
    Ok(result)
}
