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
pub use alloc::{vec, vec::Vec};

#[cfg(feature = "std")]
pub use std::vec::Vec;
