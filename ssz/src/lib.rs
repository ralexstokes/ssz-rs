mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod list;
mod ser;
mod uint;
mod union;
mod vector;

pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use list::List;
pub use ser::{Serialize, SerializeError, BYTES_PER_LENGTH_OFFSET};
pub use uint::U256;
pub use vector::Vector;

/// `SSZSized` is a trait for types that can
/// provide sizing information relevant for the SSZ spec.
pub trait SSZSized {
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    fn size_hint() -> usize;
}

/// `SimpleSerialize` is a marker trait for types
/// conforming to the SSZ spec.
pub trait SimpleSerialize: Serialize + Deserialize + SSZSized {}

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

/// The `prelude` contains common traits and types a user of this library
/// would want to have handy with a simple (single) import.
pub mod prelude {
    pub use crate::bitlist::Bitlist;
    pub use crate::bitvector::Bitvector;
    pub use crate::de::{Deserialize, DeserializeError};
    pub use crate::deserialize;
    pub use crate::list::List;
    pub use crate::ser::{Serialize, SerializeError};
    pub use crate::serialize;
    pub use crate::uint::U256;
    pub use crate::vector::Vector;
    pub use crate::SSZSized;
    pub use crate::SimpleSerialize;
    pub use ssz_derive::SimpleSerialize;
}

pub mod internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::ser::serialize_composite_from_components;
}
