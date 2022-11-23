mod core {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;
}

pub use self::core::{cmp, iter, slice, any};

pub use self::core::fmt::{self, Debug, Display, Formatter};
pub use self::core::ops::{Index, IndexMut, Deref, DerefMut};
pub use self::core::slice::{IterMut, SliceIndex};
pub use self::core::array::TryFromSliceError;
pub use self::iter::Enumerate;
pub use self::cmp::Ordering;

#[cfg(not(feature = "std"))]
pub use alloc::{vec::Vec, vec};

#[cfg(feature = "std")]
pub use std::vec::Vec;
