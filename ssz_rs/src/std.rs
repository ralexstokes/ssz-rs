// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "std")]
pub use std::{array::TryFromSliceError, cmp::Ordering, convert::AsRef, convert::TryFrom, convert::TryInto, default::Default, fmt, fmt::Debug, iter::FromIterator, iter::Enumerate, ops::DerefMut, ops::Deref, ops::Index, ops::IndexMut, option::Option, slice::IterMut, slice::SliceIndex, vec, vec::Vec};

#[cfg(not(feature = "std"))]
pub use alloc::{vec, vec::Vec};
#[cfg(not(feature = "std"))]
pub use core::{array::TryFromSliceError, cmp::Ordering, convert::AsRef, convert::TryFrom, convert::TryInto, default::Default, fmt, fmt::Debug, iter::Enumerate, iter::FromIterator, ops::DerefMut, ops::Deref, ops::Index, ops::IndexMut, option::Option, slice::IterMut, slice::SliceIndex};
