mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod list;
mod ser;
mod ssz;
mod uint;
mod union;
mod vector;

pub use crate::ssz::SSZ;
pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use list::List;
pub use ser::{Serialize, SerializeError, BYTES_PER_LENGTH_OFFSET};
pub use uint::U256;
pub use vector::Vector;

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

pub mod prelude {
    pub use crate::bitlist::Bitlist;
    pub use crate::bitvector::Bitvector;
    pub use crate::de::{Deserialize, DeserializeError};
    pub use crate::deserialize;
    pub use crate::list::List;
    pub use crate::ser::{Serialize, SerializeError};
    pub use crate::serialize;
    pub use crate::ssz::SSZ;
    pub use crate::uint::U256;
    pub use crate::vector::Vector;
    pub use ssz_derive::SimpleSerialize;
}

pub mod internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::ser::serialize_composite_from_components;
}
