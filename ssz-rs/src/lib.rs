#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod array;
mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod error;
mod list;
mod merkleization;
mod ser;
#[cfg(feature = "serde")]
mod serde_test;
mod uint;
mod union;
mod utils;
mod vector;

pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use error::Error;
pub use list::List;
pub use merkleization::{MerkleizationError, Merkleized, Node};
pub use ser::{Serialize, SerializeError};
pub use uint::U256;
pub use utils::*;
pub use vector::Vector;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::{any, cmp, iter, slice};

    pub use self::cmp::Ordering;
    pub use self::core::array::TryFromSliceError;
    pub use self::core::fmt::{self, Debug, Display, Formatter};
    pub use self::core::ops::{Deref, DerefMut, Index, IndexMut};
    pub use self::core::slice::{IterMut, SliceIndex};
    pub use self::iter::Enumerate;

    #[cfg(not(feature = "std"))]
    pub use alloc::{format, string::String, vec, vec::Vec};

    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

/// `Sized` is a trait for types that can
/// provide sizing information relevant for the SSZ spec.
pub trait Sized {
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    fn size_hint() -> usize;
}

/// `SimpleSerialize` is a trait for types
/// conforming to the SSZ spec.
pub trait SimpleSerialize: Serialize + Deserialize + Sized + Merkleized + Default {
    fn is_composite_type() -> bool {
        true
    }
}

/// The `prelude` contains common traits and types a user of this library
/// would want to have handy with a simple (single) import.
pub mod prelude {
    pub use crate as ssz_rs;
    pub use crate::bitlist::Bitlist;
    pub use crate::bitvector::Bitvector;
    pub use crate::de::Deserialize;
    pub use crate::de::DeserializeError;
    pub use crate::error::{InstanceError, TypeError};
    pub use crate::list::List;
    pub use crate::merkleization::{
        is_valid_merkle_branch, merkleize, mix_in_selector, pack, pack_bytes, MerkleizationError,
        Merkleized, Node,
    };
    pub use crate::ser::{Serialize, SerializeError};
    pub use crate::uint::U256;
    pub use crate::utils::{deserialize, serialize};
    pub use crate::vector::Vector;
    pub use crate::Error as SimpleSerializeError;
    pub use crate::SimpleSerialize;
    pub use crate::Sized;
    pub use ssz_rs_derive::SimpleSerialize;
}

/// `internal` contains functionality that is exposed purely for the derive proc macro crate
pub mod internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::merkleization::{merkleize, mix_in_selector};
    pub use crate::ser::serialize_composite_from_components;
}
