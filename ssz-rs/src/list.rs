use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::merkleization::{
    merkleize, mix_in_length, pack, Context, MerkleizationError, Merkleized, Root, BYTES_PER_CHUNK,
};
use crate::ser::{serialize_composite, Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

/// A homogenous collection of a variable number of values.
#[derive(PartialEq, Eq, Clone)]
pub struct List<T: SimpleSerialize, const N: usize>(Vec<T>);

impl<T, const N: usize> fmt::Debug for List<T, N>
where
    T: SimpleSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "List<len={}, cap={}>{:?}", self.len(), N, self.0)
    }
}

impl<T, const N: usize> Default for List<T, N>
where
    T: SimpleSerialize,
{
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T, const N: usize> Deref for List<T, N>
where
    T: SimpleSerialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for List<T, N>
where
    T: SimpleSerialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Sized for List<T, N>
where
    T: SimpleSerialize,
{
    fn is_variable_size() -> bool {
        true
    }

    fn size_hint() -> usize {
        0
    }
}

impl<T, const N: usize> Serialize for List<T, N>
where
    T: SimpleSerialize,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        if self.len() > N {
            return Err(SerializeError::TypeBoundsViolated {
                bound: N,
                len: self.len(),
            });
        }
        serialize_composite(&self.0, buffer)
    }
}

impl<T, const N: usize> Deserialize for List<T, N>
where
    T: SimpleSerialize,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        let result = deserialize_homogeneous_composite(encoding)?;
        if result.len() > N {
            return Err(DeserializeError::TypeBoundsViolated {
                bound: N,
                len: result.len(),
            });
        }
        Ok(List(result))
    }
}

impl<T, const N: usize> Merkleized for List<T, N>
where
    T: SimpleSerialize,
{
    fn hash_tree_root(&self, context: &Context) -> Result<Root, MerkleizationError> {
        if T::is_composite_type() {
            let mut chunks = Vec::with_capacity(self.len() * BYTES_PER_CHUNK);
            for (i, elem) in self.iter().enumerate() {
                let chunk = elem.hash_tree_root(context)?;
                let range = i * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK;
                chunks[range].copy_from_slice(&chunk);
            }
            let data_root = merkleize(&chunks, Some(N), context)?;
            Ok(mix_in_length(&data_root, self.len()))
        } else {
            let chunks = pack(self)?;
            let data_root = merkleize(&chunks, Some(chunks.len() / BYTES_PER_CHUNK), context)?;
            Ok(mix_in_length(&data_root, self.len()))
        }
    }
}

impl<T, const N: usize> SimpleSerialize for List<T, N> where T: SimpleSerialize {}

impl<T, const N: usize> FromIterator<T> for List<T, N>
where
    T: SimpleSerialize,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(Vec::from_iter(iter))
    }
}

impl<T, const N: usize> IntoIterator for List<T, N>
where
    T: SimpleSerialize,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a List<T, N>
where
    T: SimpleSerialize,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut List<T, N>
where
    T: SimpleSerialize,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize;

    const COUNT: usize = 32;

    #[test]
    fn encode_list() {
        let mut value: List<u16, COUNT> = List::default();
        for _ in 0..COUNT {
            value.push(33u16);
        }
        let encoding = serialize(&value).expect("can encode");
        let expected = [
            33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8,
            33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8,
            33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8,
            33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8, 33u8, 0u8,
        ];
        assert_eq!(encoding, expected);
    }

    #[test]
    fn decode_list() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let result = List::<u8, COUNT>::deserialize(&bytes).expect("can deserialize");
        let expected: List<u8, COUNT> = List(bytes);
        assert_eq!(result, expected);
    }

    #[test]
    fn roundtrip_list() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let input: List<u8, COUNT> = List(bytes);
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = List::<u8, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }

    #[test]
    fn roundtrip_list_of_list() {
        const COUNT: usize = 4;
        let bytes = vec![List(vec![0u8]), List(vec![]), List(vec![1u8])];
        let input: List<List<u8, 1>, COUNT> = List(bytes);
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = List::<List<u8, 1>, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }
}
