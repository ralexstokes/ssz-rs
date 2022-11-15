use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::merkleization::{
    merkleize, mix_in_length, pack, MerkleCache, MerkleizationError, Merkleized, Node,
    BYTES_PER_CHUNK,
};
use crate::ser::{serialize_composite, Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};
use crate::std::{Enumerate, FromIterator, vec, Vec, fmt, SliceIndex, Deref, Index, IndexMut, IterMut as StdIterMut, Debug, Display, Formatter, any};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use std::marker::PhantomData;

pub enum ListError {
    IncorrectLength { expected: usize, provided: usize }, // elements given that exceeds the list bound of
}

impl Debug for ListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A homogenous collection of a variable number of values.
#[derive(Clone, Default)]
pub struct List<T: SimpleSerialize, const N: usize> {
    data: Vec<T>,
    cache: MerkleCache,
}

// TODO clean up impls here for Vector and List...
#[cfg(feature = "serde")]
impl<T: SimpleSerialize + serde::Serialize, const N: usize> serde::Serialize for List<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for element in &self.data {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
struct ListVisitor<T: SimpleSerialize>(PhantomData<Vec<T>>);

#[cfg(feature = "serde")]
impl<'de, T: SimpleSerialize + serde::Deserialize<'de>> serde::de::Visitor<'de> for ListVisitor<T> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("array of objects")
    }

    fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
    where
        S: serde::de::SeqAccess<'de>,
    {
        serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))
    }
}

#[cfg(feature = "serde")]
impl<'de, T: SimpleSerialize + serde::de::Deserialize<'de>, const N: usize> serde::Deserialize<'de>
    for List<T, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = deserializer.deserialize_seq(ListVisitor(PhantomData))?;
        List::<T, N>::try_from(data).map_err(serde::de::Error::custom)
    }
}

impl<T: SimpleSerialize, const N: usize> AsRef<[T]> for List<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T, const N: usize> fmt::Debug for List<T, N>
where
    T: SimpleSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(
                f,
                "List<{}, {}>(len={}){:#?}",
                any::type_name::<T>(),
                N,
                self.len(),
                self.data
            )
        } else {
            write!(
                f,
                "List<{}, {}>(len={}){:?}",
                any::type_name::<T>(),
                N,
                self.len(),
                self.data
            )
        }
    }
}

impl<T, const N: usize> PartialEq for List<T, N>
where
    T: SimpleSerialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, const N: usize> Eq for List<T, N> where T: SimpleSerialize + Eq {}

impl<T, const N: usize> TryFrom<Vec<T>> for List<T, N>
where
    T: SimpleSerialize,
{
    type Error = ListError;

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        if data.len() > N {
            Err(ListError::IncorrectLength{
                expected: N,
                provided: data.len(),
            })
        } else {
            let leaf_count = Self::get_leaf_count(data.len());
            Ok(Self {
                data,
                cache: MerkleCache::with_leaves(leaf_count),
            })
        }
    }
}

impl<T, const N: usize> Deref for List<T, N>
where
    T: SimpleSerialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// NOTE: implement `IndexMut` rather than `DerefMut` to ensure
// the `List`'s inner `Vec` is not mutated, but its elements
// can change.
impl<T, Idx: SliceIndex<[T]>, const N: usize> Index<Idx> for List<T, N>
where
    T: SimpleSerialize,
{
    type Output = <Idx as SliceIndex<[T]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

// NOTE: had an issue unifying the use of `IndexMut::Idx` for `Vec`
// and `BitVec` that may be unresolveable due to how lifetimes
// are defined for this trait method. For now, only allow "one at a time"
// mutation of the `List`s data.
impl<T, const N: usize> IndexMut<usize> for List<T, N>
where
    T: SimpleSerialize,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let leaf_index = Self::get_leaf_index(index);
        self.cache.invalidate(leaf_index);
        &mut self.data[index]
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
        serialize_composite(&self.data, buffer)
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
        Ok(result.try_into().unwrap())
    }
}

impl<T, const N: usize> List<T, N>
where
    T: SimpleSerialize,
{
    // the number of leafs in the Merkle tree of this `Vector`
    fn get_leaf_count(element_count: usize) -> usize {
        if T::is_composite_type() {
            element_count
        } else {
            let encoded_length = T::size_hint() * element_count;
            (encoded_length + 31) / 32
        }
    }

    fn get_leaf_index(index: usize) -> usize {
        if T::is_composite_type() {
            index
        } else {
            // TODO: compute correct leaf index
            index + 1
        }
    }

    fn compute_hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        if T::is_composite_type() {
            let mut chunks = vec![0u8; self.len() * BYTES_PER_CHUNK];
            for (i, elem) in self.data.iter_mut().enumerate() {
                let chunk = elem.hash_tree_root()?;
                let range = i * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK;
                chunks[range].copy_from_slice(chunk.as_ref());
            }
            let data_root = merkleize(&chunks, Some(N))?;
            Ok(mix_in_length(&data_root, self.len()))
        } else {
            let chunks = pack(self)?;
            let chunk_count = (N * T::size_hint() + 31) / 32;
            let data_root = merkleize(&chunks, Some(chunk_count))?;
            Ok(mix_in_length(&data_root, self.len()))
        }
    }

    pub fn push(&mut self, element: T) {
        self.data.push(element);
        self.cache.resize(self.len());
    }

    pub fn pop(&mut self) -> Option<T> {
        let element = self.data.pop();
        self.cache.resize(self.len());
        element
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.cache.resize(0);
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            inner: self.data.iter_mut().enumerate(),
            cache: &mut self.cache,
        }
    }
}

pub struct IterMut<'a, T, const N: usize>
where
    T: SimpleSerialize,
{
    inner: Enumerate<StdIterMut<'a, T>>,
    cache: &'a mut MerkleCache,
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N>
where
    T: SimpleSerialize,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, next)) = self.inner.next() {
            let leaf_index = List::<T, N>::get_leaf_index(index);
            self.cache.invalidate(leaf_index);
            Some(next)
        } else {
            None
        }
    }
}

impl<T, const N: usize> Merkleized for List<T, N>
where
    T: SimpleSerialize,
{
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        self.compute_hash_tree_root()
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
        Vec::from_iter(iter.into_iter().take(N)).try_into().unwrap()
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
        let expected: List<u8, COUNT> = bytes.try_into().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn roundtrip_list() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let input: List<u8, COUNT> = bytes.try_into().unwrap();
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = List::<u8, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }

    #[test]
    fn roundtrip_list_of_list() {
        const COUNT: usize = 4;
        let bytes: Vec<List<u8, 1>> = vec![
            vec![0u8].try_into().unwrap(),
            Default::default(),
            vec![1u8].try_into().unwrap(),
        ];
        let input: List<List<u8, 1>, COUNT> = bytes.try_into().unwrap();
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = List::<List<u8, 1>, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }

    #[test]
    fn test_ssz_of_nested_list() {
        use crate::prelude::*;
        type Foo = List<List<u8, 16>, 32>;

        let mut value = Foo::default();
        value.push(Default::default());
        let encoding = ssz_rs::serialize(&value).unwrap();

        let mut recovered: Foo = ssz_rs::deserialize(&encoding).unwrap();
        assert_eq!(value, recovered);

        let _ = recovered.hash_tree_root().unwrap();
    }
}
