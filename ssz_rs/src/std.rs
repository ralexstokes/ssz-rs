// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "std")]
pub use std::{ops::DerefMut, ops::Deref, ops::Index, ops::IndexMut, array::TryFromSliceError, slice::IterMut, vec, vec::Vec, error, option::Option, cmp::Ordering, convert::AsRef, convert::TryFrom, convert::TryInto, fmt, iter::FromIterator, iter::Enumerate, slice::SliceIndex, fmt::Debug, default::Default};

#[cfg(not(feature = "std"))]
pub use alloc::{vec, vec::Vec};
#[cfg(not(feature = "std"))]
pub use core::{ops::DerefMut, ops::Deref, ops::Index, ops::IndexMut, slice::SliceIndex, slice::IterMut, option::Option, array::TryFromSliceError, iter::FromIterator, iter::Enumerate, convert::TryFrom, convert::TryInto, convert::AsRef, fmt, fmt::Debug, cmp::Ordering, default::Default};
