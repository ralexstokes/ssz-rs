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

// #[inline]
// fn write_hex_from_bytes<D: AsRef<[u8]>>(f: &mut fmt::Formatter<'_>, data: D) -> fmt::Result {
//     for i in data.as_ref() {
//         write!(f, "{i:02x}")?;
//     }
//     Ok(())
// }

// pub fn write_bytes_to_lower_hex<T: AsRef<[u8]>>(
//     f: &mut fmt::Formatter<'_>,
//     data: T,
// ) -> fmt::Result {
//     write!(f, "0x")?;
//     write_hex_from_bytes(f, data)
// }

// pub fn write_bytes_to_lower_hex_display<T: AsRef<[u8]> + ExactSizeIterator>(
//     f: &mut fmt::Formatter<'_>,
//     data: T,
// ) -> fmt::Result {
//     let len = data.len();
//     let (first, last) = if len >= 4 { ((0..2), Some(len - 2..len)) } else { ((0..len), None) };
//     let data = data.as_ref();
//     write!(f, "0x")?;
//     write_hex_from_bytes(f, &data[first])?;
//     if let Some(last) = last {
//         write!(f, "…")?;
//         write_hex_from_bytes(f, &data[last])?;
//     }
//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct Fmt(Vec<u8>);

//     impl fmt::Debug for Fmt {
//         fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//             write_bytes_to_lower_hex(f, self.0.iter())
//         }
//     }

//     impl fmt::Display for Fmt {
//         fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//             write!(f, "{:?}", self)
//         }
//     }

//     #[test]
//     fn test_fmt() {
//         let data = Fmt((0u8..3).collect::<Vec<_>>());
//         let s = format!("{data:?}");
//         assert_eq!(s, "0x000102");
//         let s = format!("{data}");
//         assert_eq!(s, "0x000102");
//     }
// }
