use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::ser::{serialize_homogeneous_composite, Serialize, SerializeError};
use crate::SSZ;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

/// A homogenous collection of a fixed number of values.
/// NOTE: a `Vector` of length `0` is illegal.
#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T: SSZ, const N: usize>([T; N]);

impl<T, const N: usize> SSZ for Vector<T, N>
where
    T: SSZ,
{
    fn is_variable_size() -> bool {
        T::is_variable_size()
    }

    fn size_hint() -> usize {
        T::size_hint() * N
    }
}

impl<T, const N: usize> Serialize for Vector<T, N>
where
    T: SSZ,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        assert!(N > 0);
        serialize_homogeneous_composite(self, buffer)
    }
}

impl<T, const N: usize> Deserialize for Vector<T, N>
where
    T: SSZ,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        assert!(N > 0);
        let elements = deserialize_homogeneous_composite(encoding, T::size_hint())?;
        elements
            .try_into()
            .map(Vector)
            .map_err(|_| DeserializeError::InputTooShort)
    }
}

impl<T, const N: usize> Copy for Vector<T, N> where T: Copy + SSZ {}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Copy + SSZ,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N>
where
    T: SSZ,
{
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.0)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N>
where
    T: SSZ,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N>
where
    T: SSZ,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: SSZ + Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: SSZ,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
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
    fn encode_vector() {
        let mut value: Vector<u16, COUNT> = Vector::default();
        for elem in &mut value {
            *elem = 33u16;
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
    fn decode_vector() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let result = Vector::<u8, COUNT>::deserialize(&bytes).expect("can deserialize");
        let expected: Vector<u8, COUNT> = Vector(bytes.try_into().expect("test data"));
        assert_eq!(result, expected);
    }

    #[test]
    fn roundtrip_vector() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let input: Vector<u8, COUNT> = Vector(bytes.try_into().expect("test data"));
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Vector::<u8, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }
}
