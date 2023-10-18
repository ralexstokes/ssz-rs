//! An implementation of the [SSZ][ssz] serialization scheme.
//!
//! # Examples
//!
//! De/serialize a simple value:
//!
//! ```
//! # use ssz_rs::prelude::*;
//! let mut buf = Vec::new();
//! 42u64.serialize(&mut buf);
//! assert_eq!(u64::deserialize(&buf).unwrap(), 42);
//! ```
//!
//! De/serialize a custom type using the derive macro:
//!
//! ```
//! # use ssz_rs::prelude::*;
//! #[derive(Debug, Default, PartialEq, Eq, SimpleSerialize)]
//! struct Data {
//!   flag: bool,
//!   value: u64
//! }
//!
//! let mut buf = Vec::new();
//! Data { flag: true, value: 42 }.serialize(&mut buf);
//! assert_eq!(
//!   Data::deserialize(&buf).unwrap(),
//!   Data { flag: true, value: 42 }
//! );
//! ```
//!
//! [ssz]: https://github.com/ethereum/consensus-specs/blob/dev/ssz/simple-serialize.md
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
pub mod serde;
mod uint;
mod union;
pub mod utils;
mod vector;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::{any, cmp, fmt, iter, slice};

    pub use self::{
        cmp::Ordering,
        core::{
            array::TryFromSliceError,
            fmt::{Debug, Display, Formatter},
            ops::{Deref, DerefMut, Index, IndexMut},
            slice::{IterMut, SliceIndex},
            str::FromStr,
        },
        iter::{Enumerate, ExactSizeIterator},
    };

    #[cfg(not(feature = "std"))]
    pub use alloc::{format, string::String, vec, vec::Vec};

    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(feature = "serde")]
    pub use self::core::marker::PhantomData;
}

pub(crate) const BITS_PER_BYTE: u32 = 8;

/// `Serializable` is a trait for types that can be
/// serialized and deserialized according to the SSZ spec.
pub trait Serializable: Serialize + Deserialize {
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    // expected number of bytes for the serialization of this type
    // or 0 if unknown ahead of time
    fn size_hint() -> usize;
}

/// `SimpleSerialize` is a trait for types conforming to the SSZ spec.
/// These types can be encoded and decoded while also supporting the
/// merkelization scheme of SSZ.
pub trait SimpleSerialize: Serializable + Merkleized {}

mod exports {
    pub use crate::{
        bitlist::Bitlist,
        bitvector::Bitvector,
        de::{Deserialize, DeserializeError},
        error::{Error as SimpleSerializeError, InstanceError, TypeError},
        list::List,
        merkleization::{is_valid_merkle_branch, MerkleizationError, Merkleized, Node},
        ser::{Serialize, SerializeError},
        uint::U256,
        utils::{deserialize, serialize},
        vector::Vector,
        Serializable, SimpleSerialize,
    };
}

pub use crate::exports::*;

/// The `prelude` contains common traits and types a user of this library
/// would want to have handy with a simple (single) import.
pub mod prelude {
    pub use crate::exports::*;

    // expose this so the derive macro has everything in scope
    // with a simple `prelude` import
    pub use crate as ssz_rs;
    pub use ssz_rs_derive::{Merkleized, Serializable, SimpleSerialize};
}

#[doc(hidden)]
/// `internal` contains functionality that is exposed purely for the derive proc macro crate
pub mod __internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::{
        de::ContainerDeserializer,
        merkleization::{merkleize, mix_in_selector},
        ser::Serializer,
    };
}
