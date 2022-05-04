use crate::de::{Deserialize, DeserializeError};
use crate::merkleization::{merkleize, pack_bytes, MerkleizationError, Merkleized, Node};
use crate::ser::{Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};
use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Lsb0};
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

type BitvectorInner = BitVec<u8, Lsb0>;

/// A homogenous collection of a fixed number of boolean values.
///
/// NOTE: a `Bitvector` of length `0` is illegal.
///
/// NOTE: once `const_generics` and `const_evaluatable_checked` features stabilize,
/// this type can use something like
/// bitvec::array::BitArray<T, {N / 8}> where T: BitRegister, [T; {N / 8}]: BitViewSized
///
/// Refer: <https://stackoverflow.com/a/65462213>
#[derive(PartialEq, Eq, Clone)]
pub struct Bitvector<const N: usize>(BitvectorInner);

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for Bitvector<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = Vec::with_capacity((N + 7) / 8);
        let _ = crate::Serialize::serialize(self, &mut buf).map_err(serde::ser::Error::custom)?;
        serializer.collect_str(&hex::encode(buf))
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for Bitvector<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        let value = crate::Deserialize::deserialize(&bytes).map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl<const N: usize> fmt::Debug for Bitvector<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Bitvector<{}>[", N)?;
        let len = self.len();
        let mut bits_written = 0;
        for (index, bit) in self.iter().enumerate() {
            let value = if *bit { 1 } else { 0 };
            write!(f, "{}", value)?;
            bits_written += 1;
            if bits_written % 4 == 0 && index != len - 1 {
                write!(f, "_")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<const N: usize> Default for Bitvector<N> {
    fn default() -> Self {
        assert!(N > 0);
        Self(BitVec::repeat(false, N))
    }
}

impl<const N: usize> Bitvector<N> {
    /// Return the bit at `index`. `None` if index is out-of-bounds.
    pub fn get(&mut self, index: usize) -> Option<bool> {
        self.0.get(index).map(|value| *value)
    }

    /// Set the bit at `index` to `value`. Return the previous value
    /// or `None` if index is out-of-bounds.
    pub fn set(&mut self, index: usize, value: bool) -> Option<bool> {
        self.get_mut(index).map(|mut slot| {
            let old = *slot;
            *slot = value;
            old
        })
    }

    fn pack_bits(&self) -> Result<Vec<u8>, MerkleizationError> {
        let mut data = vec![];
        let _ = self.serialize(&mut data)?;
        pack_bytes(&mut data);
        Ok(data)
    }
}

impl<const N: usize> Deref for Bitvector<N> {
    type Target = BitvectorInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Bitvector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Sized for Bitvector<N> {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        (N + 7) / 8
    }
}

impl<const N: usize> Serialize for Bitvector<N> {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        if N == 0 {
            return Err(SerializeError::IllegalType { bound: N });
        }
        let bytes_to_write = Self::size_hint();
        buffer.reserve(bytes_to_write);
        for byte in self.chunks(8) {
            buffer.push(byte.load());
        }
        Ok(bytes_to_write)
    }
}

impl<const N: usize> Deserialize for Bitvector<N> {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if N == 0 {
            return Err(DeserializeError::IllegalType { bound: N });
        }

        let expected_length = (N + 7) / 8;
        if encoding.len() < expected_length {
            return Err(DeserializeError::InputTooShort);
        }
        if encoding.len() > expected_length {
            return Err(DeserializeError::ExtraInput);
        }

        let mut result = Self::default();
        for (slot, byte) in result.chunks_mut(8).zip(encoding.iter().copied()) {
            slot.store_le(byte);
        }
        let remainder_count = N % 8;
        if remainder_count != 0 {
            let last_byte = encoding.last().unwrap();
            let remainder_bits = last_byte >> remainder_count;
            if remainder_bits != 0 {
                return Err(DeserializeError::ExtraInput);
            }
        }
        Ok(result)
    }
}

impl<const N: usize> Merkleized for Bitvector<N> {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        let chunks = self.pack_bits()?;
        merkleize(&chunks, Some((N + 255) / 256))
    }
}

impl<const N: usize> SimpleSerialize for Bitvector<N> {}

impl<const N: usize> FromIterator<bool> for Bitvector<N> {
    // NOTE: only takes the first `N` values from `iter` and
    // uses the default `false` for missing values.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        assert!(N > 0);

        let mut result: Bitvector<N> = Default::default();
        for (index, bit) in iter.into_iter().enumerate().take(N) {
            result.set(index, bit);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize;

    const COUNT: usize = 12;

    #[test]
    fn encode_bitvector() {
        let value: Bitvector<4> = Bitvector::default();
        let encoding = serialize(&value).expect("can encode");
        let expected = [0u8];
        assert_eq!(encoding, expected);

        let value: Bitvector<COUNT> = Bitvector::default();
        let encoding = serialize(&value).expect("can encode");
        let expected = [0u8, 0u8];
        assert_eq!(encoding, expected);

        let mut value: Bitvector<COUNT> = Bitvector::default();
        value.set(3, true).expect("test data correct");
        value.set(4, true).expect("test data correct");
        assert_eq!(value.get(4).expect("test data correct"), true);
        assert_eq!(value.get(0).expect("test data correct"), false);
        let encoding = serialize(&value).expect("can encode");
        let expected = [24u8, 0u8];
        assert_eq!(encoding, expected);
    }

    #[test]
    fn decode_bitvector() {
        let bytes = vec![12u8];
        let result = Bitvector::<4>::deserialize(&bytes).expect("test data is correct");
        let expected = Bitvector::from_iter(vec![false, false, true, true]);
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_bitvector_several() {
        let bytes = vec![24u8, 1u8];
        let result = Bitvector::<COUNT>::deserialize(&bytes).expect("test data is correct");
        let expected = Bitvector::from_iter(vec![
            false, false, false, true, true, false, false, false, true, false, false, false,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn roundtrip_bitvector() {
        let input = Bitvector::<COUNT>::from_iter(vec![
            false, false, false, true, true, false, false, false, false, false, false, false,
        ]);
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Bitvector::<COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }
}
