extern crate core;

mod array;
mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod list;
mod merkleization;
mod ser;
#[cfg(feature = "serde")]
mod serde_test;
mod uint;
mod union;
mod vector;

use crate::{list::Error as ListError, vector::Error as VectorError};
pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use list::List;
pub use merkleization::{
    calculate_merkle_root, calculate_multi_merkle_root, field_inspect, generate_proof,
    get_generalized_index, is_valid_merkle_branch, verify_merkle_multiproof, verify_merkle_proof,
    Context as MerkleizationContext, GeneralizedIndex, MerkleizationError, Merkleized, Node,
    SszReflect, SszVariableOrIndex,
};
pub use ser::{Serialize, SerializeError};
use thiserror::Error;
pub use uint::U256;
pub use vector::Vector;

/// `Sized` is a trait for types that can
/// provide sizing information relevant for the SSZ spec.
pub trait Sized
where
    Self: std::marker::Sized,
{
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    fn size_hint() -> usize;
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ElementsType {
    Vector,
    List,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SszTypeClass {
    Basic,
    Bits(ElementsType),
    Elements(ElementsType),
    Container,
    Union,
}

/// `SimpleSerialize` is a trait for types
/// conforming to the SSZ spec.
pub trait SimpleSerialize: Serialize + Deserialize + Sized + Merkleized + Default {
    fn is_composite_type() -> bool {
        true
    }
}

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

#[derive(Debug, Error)]
#[error("{0}")]
pub enum SimpleSerializeError {
    Serialize(#[from] SerializeError),
    Deserialize(#[from] DeserializeError),
    Merkleization(#[from] MerkleizationError),
    List(#[from] ListError),
    Vector(#[from] VectorError),
}

/// The `prelude` contains common traits and types a user of this library
/// would want to have handy with a simple (single) import.
pub mod prelude {
    pub use crate as ssz_rs;
    pub use crate::{
        bitlist::Bitlist,
        bitvector::Bitvector,
        de::{Deserialize, DeserializeError},
        deserialize,
        list::List,
        merkleization::{
            is_valid_merkle_branch, merkleize, mix_in_selector, pack, pack_bytes,
            MerkleizationError, Merkleized, Node,
        },
        ser::{Serialize, SerializeError},
        serialize,
        uint::U256,
        vector::Vector,
        MerkleizationContext, SimpleSerialize, SimpleSerializeError, Sized,
    };
    pub use ssz_rs_derive::SimpleSerialize;
}

/// `internal` contains functionality that is exposed purely for the derive proc macro crate
pub mod internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::{
        merkleization::{merkleize, mix_in_selector},
        ser::serialize_composite_from_components,
    };
}
