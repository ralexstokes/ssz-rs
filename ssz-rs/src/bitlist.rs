use crate::de::{Deserialize, DeserializeError};
use crate::error::InstanceError;
use crate::merkleization::{
    merkleize, mix_in_length, pack_bytes, MerkleizationError, Merkleized, Node,
};
use crate::ser::{Serialize, SerializeError};
use crate::std::*;
use crate::{SimpleSerialize, Sized};
use bitvec::prelude::{BitVec, Lsb0};

// +1 for length bit
fn byte_length(bound: usize) -> usize {
    (bound + 7 + 1) / 8
}

type BitlistInner = BitVec<u8, Lsb0>;

/// A homogenous collection of a variable number of boolean values.
#[derive(PartialEq, Eq, Clone)]
pub struct Bitlist<const N: usize>(BitlistInner);

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for Bitlist<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let byte_count = byte_length(self.len());
        let mut buf = Vec::with_capacity(byte_count);
        let _ = crate::Serialize::serialize(self, &mut buf).map_err(serde::ser::Error::custom)?;
        let encoding = hex::encode(buf);
        let output = format!("0x{encoding}");
        serializer.collect_str(&output)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for Bitlist<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        if s.len() < 2 {
            return Err(serde::de::Error::custom(
                DeserializeError::ExpectedFurtherInput {
                    provided: s.len(),
                    expected: 2,
                },
            ));
        }
        let bytes = hex::decode(&s[2..]).map_err(serde::de::Error::custom)?;
        let value = crate::Deserialize::deserialize(&bytes).map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl<const N: usize> fmt::Debug for Bitlist<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Bitlist<len={}, cap={}>[", self.len(), N)?;
        let len = self.len();
        let mut bits_written = 0;
        for (index, bit) in self.iter().enumerate() {
            let value = i32::from(*bit);
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

impl<const N: usize> Default for Bitlist<N> {
    fn default() -> Self {
        Self(BitVec::new())
    }
}

impl<const N: usize> Bitlist<N> {
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
        let _ = self.serialize_with_length(&mut data, false);
        pack_bytes(&mut data);
        Ok(data)
    }

    fn serialize_with_length(
        &self,
        buffer: &mut Vec<u8>,
        with_length_bit: bool,
    ) -> Result<usize, SerializeError> {
        if self.len() > N {
            return Err(InstanceError::Bounded {
                bound: N,
                provided: self.len(),
            }
            .into());
        }
        let start_len = buffer.len();
        buffer.extend_from_slice(self.as_raw_slice());

        if with_length_bit {
            let element_count = self.len();
            let marker_index = element_count % 8;
            if marker_index == 0 {
                buffer.push(1u8);
            } else {
                let last = buffer.last_mut().expect("bitlist cannot be empty");
                *last |= 1u8 << marker_index;
            }
        }
        Ok(buffer.len() - start_len)
    }
}

impl<const N: usize> Deref for Bitlist<N> {
    type Target = BitlistInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Bitlist<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Sized for Bitlist<N> {
    fn is_variable_size() -> bool {
        true
    }

    fn size_hint() -> usize {
        0
    }
}

impl<const N: usize> Serialize for Bitlist<N> {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        self.serialize_with_length(buffer, true)
    }
}

impl<const N: usize> Deserialize for Bitlist<N> {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        let max_len = byte_length(N);
        if encoding.is_empty() {
            return Err(DeserializeError::ExpectedFurtherInput {
                provided: 0,
                expected: max_len,
            });
        }

        if encoding.len() > max_len {
            return Err(DeserializeError::AdditionalInput {
                provided: encoding.len(),
                expected: max_len,
            });
        }

        let (last_byte, prefix) = encoding.split_last().unwrap();
        let mut result = BitlistInner::from_slice(prefix);
        let last = BitlistInner::from_element(*last_byte);
        let high_bit_index = 8 - last.trailing_zeros();

        if !last[high_bit_index - 1] {
            return Err(DeserializeError::InvalidByte(*last_byte));
        }

        for bit in last.iter().take(high_bit_index - 1) {
            result.push(*bit);
        }
        // TODO: this seems redundant...
        if result.len() > N {
            return Err(InstanceError::Bounded {
                bound: N,
                provided: result.len(),
            }
            .into());
        }
        Ok(Self(result))
    }
}

impl<const N: usize> Merkleized for Bitlist<N> {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        let chunks = self.pack_bits()?;
        let data_root = merkleize(&chunks, Some((N + 255) / 256))?;
        Ok(mix_in_length(&data_root, self.len()))
    }
}

impl<const N: usize> SimpleSerialize for Bitlist<N> {}

impl<const N: usize> FromIterator<bool> for Bitlist<N> {
    // NOTE: only takes the first `N` values from `iter`.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        let mut result: Bitlist<N> = Default::default();
        for bit in iter.into_iter().take(N) {
            result.push(bit);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;
    use super::*;
    use crate::serialize;

    const COUNT: usize = 256;

    #[test]
    fn encode_bitlist() {
        let value: Bitlist<COUNT> = Bitlist::default();
        let encoding = serialize(&value).expect("can encode");
        let expected = [1u8];
        assert_eq!(encoding, expected);

        let mut value: Bitlist<COUNT> = Bitlist::default();
        value.push(false);
        value.push(true);
        let encoding = serialize(&value).expect("can encode");
        let expected = [6u8];
        assert_eq!(encoding, expected);

        let mut value: Bitlist<COUNT> = Bitlist::default();
        value.push(false);
        value.push(false);
        value.push(false);
        value.push(true);
        value.push(true);
        value.push(false);
        value.push(false);
        value.push(false);
        assert_eq!(value.get(0).expect("test data correct"), false);
        assert_eq!(value.get(3).expect("test data correct"), true);
        assert_eq!(value.get(4).expect("test data correct"), true);
        assert_eq!(value.get(7).expect("test data correct"), false);
        let encoding = serialize(&value).expect("can encode");
        let expected = [24u8, 1u8];
        assert_eq!(encoding, expected);
    }

    #[test]
    fn decode_bitlist() {
        let bytes = vec![1u8];
        let result = Bitlist::<COUNT>::deserialize(&bytes).expect("test data is correct");
        let expected = Bitlist::from_iter(vec![]);
        assert_eq!(result, expected);

        let bytes = vec![24u8, 1u8];
        let result = Bitlist::<COUNT>::deserialize(&bytes).expect("test data is correct");
        let expected =
            Bitlist::from_iter(vec![false, false, false, true, true, false, false, false]);
        assert_eq!(result, expected);

        let bytes = vec![24u8, 2u8];
        let result = Bitlist::<COUNT>::deserialize(&bytes).expect("test data is correct");
        let expected = Bitlist::from_iter(vec![
            false, false, false, true, true, false, false, false, false,
        ]);
        assert_eq!(result, expected);
        let bytes = vec![24u8, 3u8];
        let result = Bitlist::<COUNT>::deserialize(&bytes).expect("test data is correct");
        let expected = Bitlist::from_iter(vec![
            false, false, false, true, true, false, false, false, true,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn roundtrip_bitlist() {
        let input = Bitlist::<COUNT>::from_iter(vec![
            false, false, false, true, true, false, false, false, false, false, false, false,
            false, false, false, true, true, false, false, false, false, false, false, false, true,
        ]);
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Bitlist::<COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }
}
