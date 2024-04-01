use crate::{de::DeserializeError, lib::*, ser::SerializeError, Serializable};

/// `serialize` is a convenience function for taking a value that
/// implements `SimpleSerialize` and attempting to encode it to
/// a `Vec<u8>` according to the SSZ spec.
pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serializable,
{
    let mut result = vec![];
    value.serialize(&mut result)?;
    Ok(result)
}

/// `deserialize` is a convenience function for taking an encoding
/// for some value that implements `SimpleSerialize` in a `&[u8]`
/// and attempting to deserialize that value from the byte representation.
pub fn deserialize<T>(encoding: &[u8]) -> Result<T, DeserializeError>
where
    T: Serializable,
{
    T::deserialize(encoding)
}
