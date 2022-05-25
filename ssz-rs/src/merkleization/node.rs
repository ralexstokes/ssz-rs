use crate::prelude::*;
use crate::std::{Index, IndexMut, Vec, vec, TryFromSliceError, fmt, AsRef};

#[derive(Default, Clone, Copy, PartialEq, Eq, SimpleSerialize)]
pub struct Node(pub(crate) [u8; 32]);

#[cfg(feature = "serde-rs")]
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!("{}", self))
    }
}

#[cfg(feature = "serde-rs")]
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
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({:x})", self)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
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
