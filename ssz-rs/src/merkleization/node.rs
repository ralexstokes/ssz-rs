use crate::{lib::*, prelude::*};
use core::{
    array::TryFromSliceError,
    convert::AsRef,
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Default, Clone, Copy, PartialEq, Eq, SimpleSerialize)]
pub struct Node(pub(crate) [u8; 32]);

#[cfg(feature = "serde")]
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!("{self}"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        let bytes = hex::decode(&s[2..]).map_err(serde::de::Error::custom)?;
        let value = crate::Deserialize::deserialize(&bytes).map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Node {
    pub fn from_bytes(root: [u8; 32]) -> Self {
        Self(root)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::LowerHex for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0[..] {
            write!(f, "{i:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({self:x})")
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#x}")
    }
}

impl AsRef<[u8]> for Node {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Index<usize> for Node {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl TryFrom<&[u8]> for Node {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self(inner))
    }
}

impl PartialEq<[u8; 32]> for Node {
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0 == *other
    }
}
