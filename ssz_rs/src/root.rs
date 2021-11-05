use crate::prelude::*;
use std::array::TryFromSliceError;
use std::convert::AsRef;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy, PartialEq, Eq, SimpleSerialize)]
pub struct Root([u8; 32]);

impl Root {
    pub fn from_bytes(root: [u8; 32]) -> Self {
        Self(root)
    }
}

impl fmt::LowerHex for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl AsRef<[u8; 32]> for Root {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for Root {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Index<usize> for Root {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Root {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl TryFrom<&[u8]> for Root {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self(inner))
    }
}

impl PartialEq<[u8; 32]> for Root {
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0 == *other
    }
}
