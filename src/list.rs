use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::ser::{serialize_homogeneous_composite, Serialize, SerializeError};
use crate::ssz::SSZ;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

/// A homogenous collection of a variable number of values.
#[derive(Debug, PartialEq, Eq)]
pub struct List<T: SSZ, const N: usize>(Vec<T>);

impl<T, const N: usize> List<T, N>
where
    T: SSZ,
{
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T, const N: usize> SSZ for List<T, N>
where
    T: SSZ,
{
    fn is_variable_size() -> bool {
        true
    }

    fn size_hint() -> usize {
        T::size_hint()
    }
}

impl<T, const N: usize> Serialize for List<T, N>
where
    T: SSZ,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        assert!(self.0.len() <= N);
        serialize_homogeneous_composite(self, buffer)
    }
}

impl<T, const N: usize> Deserialize for List<T, N>
where
    T: SSZ,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        let elements = deserialize_homogeneous_composite(encoding)?;
        assert!(elements.len() <= N);
        Ok(List(elements))
    }
}

impl<T, const N: usize> IntoIterator for List<T, N>
where
    T: SSZ,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a List<T, N>
where
    T: SSZ,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut List<T, N>
where
    T: SSZ,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const N: usize> Default for List<T, N>
where
    T: SSZ,
{
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T, const N: usize> Index<usize> for List<T, N>
where
    T: SSZ,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for List<T, N>
where
    T: SSZ,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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
