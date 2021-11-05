use crate::de::{Deserialize, DeserializeError};
use crate::merkleization::{Context, MerkleizationError, Merkleized};
use crate::ser::{Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};
use std::array::TryFromSliceError;
use std::convert::AsRef;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

impl Sized for Root {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        32
    }
}

impl Serialize for Root {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(self.as_ref());
        Ok(Self::size_hint())
    }
}

impl Deserialize for Root {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        let byte_size = Self::size_hint();
        if encoding.len() < byte_size {
            return Err(DeserializeError::InputTooShort);
        }
        if encoding.len() > byte_size {
            return Err(DeserializeError::ExtraInput);
        }

        let root = encoding[..byte_size]
            .try_into()
            .expect("slice has right length");
        Ok(root)
    }
}

impl Merkleized for Root {
    fn hash_tree_root(&self, _context: &Context) -> Result<Root, MerkleizationError> {
        Ok(Root(self.0))
    }
}

impl SimpleSerialize for Root {
    fn is_composite_type() -> bool {
        false
    }
}
