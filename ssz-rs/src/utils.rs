use crate::{de::DeserializeError, lib::*, ser::SerializeError, SimpleSerialize};

/// `serialize` is a convenience function for taking a value that
/// implements `SimpleSerialize` and attempting to encode it to
/// a `Vec<u8>` according to the SSZ spec.
pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: SimpleSerialize,
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
    T: SimpleSerialize,
{
    T::deserialize(encoding)
}

pub(crate) fn write_bytes_to_lower_hex<T: AsRef<[u8]>>(
    f: &mut fmt::Formatter<'_>,
    data: T,
) -> fmt::Result {
    if f.alternate() {
        write!(f, "0x")?;
    }
    for i in data.as_ref() {
        write!(f, "{i:02x}")?;
    }
    Ok(())
}
