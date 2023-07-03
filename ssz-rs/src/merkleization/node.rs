use crate::{lib::*, merkleization::BYTES_PER_CHUNK, prelude::*, utils::write_bytes_to_lower_hex};

/// A node in a merkle tree.
#[derive(Default, Clone, Copy, Eq, Hash, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::as_hex"))] [u8; BYTES_PER_CHUNK],
);

impl fmt::LowerHex for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({self:#x})")
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#x}")
    }
}

impl AsRef<[u8]> for Node {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for Node {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl TryFrom<&[u8]> for Node {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let inner = value.try_into()?;
        Ok(Self(inner))
    }
}

impl<T> PartialEq<T> for Node
where
    T: AsRef<[u8]>,
{
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let mut node = Node::default();
        node.as_mut()[2] = 33;
        let node_repr = serde_json::to_string(&node).unwrap();
        let recovered_node: Node = serde_json::from_str(&node_repr).unwrap();
        assert_eq!(node, recovered_node);
    }
}
