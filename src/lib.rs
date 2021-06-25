mod bitvector;
mod boolean;
mod de;
mod list;
mod ser;
mod ssz;
mod uint;
mod vector;

pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use ser::{Serialize, SerializeError};
pub use vector::Vector;

use crate::ssz::SSZ;

pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: SSZ,
{
    let mut result = vec![];
    value.serialize(&mut result)?;
    Ok(result)
}

pub fn deserialize<T>(encoding: &[u8]) -> Result<T, DeserializeError>
where
    T: SSZ,
{
    T::deserialize(encoding)
}
