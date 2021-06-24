use crate::de::{Deserialize, DeserializeError};
use crate::ser::{serialize_composite, Serialize, SerializeError};
use crate::SSZ;
use std::ops::{Index, IndexMut};

/// A homogenous collection of a fixed number of values.
/// NOTE: a `Vector` of length `0` is illegal.
#[derive(Debug)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> SSZ for Vector<T, N>
where
    T: SSZ,
{
    fn is_variable_size(&self) -> bool {
        assert!(N > 0);
        self.0[0].is_variable_size()
    }

    fn size_hint() -> usize {
        assert!(N > 0);
        T::size_hint() * N
    }
}

impl<T, const N: usize> Serialize for Vector<T, N>
where
    T: SSZ,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        assert!(N > 0);
        serialize_composite(self, buffer)
    }
}

impl<T, const N: usize> Deserialize for Vector<T, N>
where
    T: SSZ,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        unimplemented!()
    }
}

impl<T, const N: usize> Copy for Vector<T, N> where T: Copy {}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.0)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N> {
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
        Vector([T::default(); N])
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
    use crate::{deserialize, serialize};

    #[test]
    fn encode_vector() {
        const COUNT: usize = 32;
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
        const COUNT: usize = 32;

        let bytes = vec![];
        let result = Vector::<u16, COUNT>::deserialize(&bytes);
        assert!(result.is_ok())
    }

    #[test]
    fn roundtrip_vector() {}
}
