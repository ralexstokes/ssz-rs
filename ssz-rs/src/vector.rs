use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::error::{InstanceError, TypeError};
use crate::lib::*;
use crate::merkleization::{
    merkleize, pack, MerkleCache, MerkleizationError, Merkleized, Node, BYTES_PER_CHUNK,
};
use crate::ser::{serialize_composite, Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};
#[cfg(feature = "serde")]
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

/// A homogenous collection of a fixed number of values.
/// NOTE: a `Vector` of length `0` is illegal.
#[derive(Clone)]
pub struct Vector<T: SimpleSerialize, const N: usize> {
    data: Vec<T>,
    cache: MerkleCache,
}

#[cfg(feature = "serde")]
impl<T: SimpleSerialize + serde::Serialize, const N: usize> serde::Serialize for Vector<T, N> {
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
struct VectorVisitor<T: SimpleSerialize>(PhantomData<Vec<T>>);

#[cfg(feature = "serde")]
impl<'de, T: SimpleSerialize + serde::Deserialize<'de>> serde::de::Visitor<'de>
    for VectorVisitor<T>
{
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
    for Vector<T, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = deserializer.deserialize_seq(VectorVisitor(PhantomData))?;
        Vector::<T, N>::try_from(data).map_err(serde::de::Error::custom)
    }
}

impl<T: SimpleSerialize, const N: usize> AsRef<[T]> for Vector<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T: SimpleSerialize + PartialEq, const N: usize> PartialEq for Vector<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: SimpleSerialize + Eq, const N: usize> Eq for Vector<T, N> {}

impl<T: SimpleSerialize, const N: usize> TryFrom<Vec<T>> for Vector<T, N> {
    type Error = DeserializeError;

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err(TypeError::InvalidBound(N).into());
        }
        if data.len() != N {
            Err(InstanceError::Exact {
                required: N,
                provided: data.len(),
            }
            .into())
        } else {
            let leaf_count = Self::get_leaf_count();
            Ok(Self {
                data,
                cache: MerkleCache::with_leaves(leaf_count),
            })
        }
    }
}

impl<T, const N: usize> fmt::Debug for Vector<T, N>
where
    T: SimpleSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(
                f,
                "Vector<{}, {}>{:#?}",
                any::type_name::<T>(),
                N,
                self.data
            )
        } else {
            write!(f, "Vector<{}, {}>{:?}", any::type_name::<T>(), N, self.data)
        }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: SimpleSerialize + Default + Clone,
{
    fn default() -> Self {
        let data = vec![T::default(); N];
        data.try_into().unwrap()
    }
}

impl<T, const N: usize> Deref for Vector<T, N>
where
    T: SimpleSerialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// NOTE: implement `IndexMut` rather than `DerefMut` to ensure
// the `Vector`'s inner `Vec` is not mutated, but its elements
// can change.
impl<T, Idx: SliceIndex<[T]>, const N: usize> Index<Idx> for Vector<T, N>
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
// mutation of the `Vector`s data.
impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: SimpleSerialize,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let leaf_index = self.get_leaf_index(index);
        self.cache.invalidate(leaf_index);
        &mut self.data[index]
    }
}

impl<T, const N: usize> Sized for Vector<T, N>
where
    T: SimpleSerialize,
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
    T: SimpleSerialize,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        if N == 0 {
            return Err(TypeError::InvalidBound(N).into());
        }
        serialize_composite(&self.data, buffer)
    }
}

impl<T, const N: usize> Deserialize for Vector<T, N>
where
    T: SimpleSerialize,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if N == 0 {
            return Err(TypeError::InvalidBound(N).into());
        }
        if !T::is_variable_size() {
            let expected_length = N * T::size_hint();
            if encoding.len() < expected_length {
                return Err(DeserializeError::ExpectedFurtherInput {
                    provided: encoding.len(),
                    expected: expected_length,
                });
            }
            if encoding.len() > expected_length {
                return Err(DeserializeError::AdditionalInput {
                    provided: encoding.len(),
                    expected: expected_length,
                });
            }
        }
        deserialize_homogeneous_composite(encoding)?.try_into()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: SimpleSerialize,
{
    // the number of leafs in the Merkle tree of this `Vector`
    fn get_leaf_count() -> usize {
        if T::is_composite_type() {
            N
        } else {
            let encoded_length = Self::size_hint();
            (encoded_length + 31) / 32
        }
    }

    fn get_leaf_index(&self, index: usize) -> usize {
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
            merkleize(&chunks, None)
        } else {
            let chunks = pack(&self.data)?;
            merkleize(&chunks, None)
        }
    }
}

impl<T, const N: usize> Merkleized for Vector<T, N>
where
    T: SimpleSerialize,
{
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        if !self.cache.valid() {
            // which leaves are dirty
            // figure out which elements are needed and recompute leaves
            // update cache w/ new leaves
            let root = self.compute_hash_tree_root()?;
            self.cache.update(root);
        }
        Ok(self.cache.root())
    }
}

impl<T, const N: usize> SimpleSerialize for Vector<T, N> where T: SimpleSerialize + Clone {}

impl<T, const N: usize> FromIterator<T> for Vector<T, N>
where
    T: SimpleSerialize + Default,
{
    // Builds a `Vector<T, N>` from the iterator given by `iter`.
    // If `iter` is more than `N` elements, then only the first `N` are taken.
    // If `iter` is less than `N` elements, the Vector is extended with
    // the remainder using `T::default()`.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut data = iter.into_iter().take(N).collect::<Vec<_>>();
        for _ in data.len()..N {
            data.push(T::default())
        }
        data.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::List;
    use crate::serialize;

    const COUNT: usize = 32;

    #[test]
    fn test_from_iter() {
        let data = vec![2u8; 10];
        let vector = Vector::<u8, 1>::from_iter(data.clone());
        assert_eq!(vector[0], 2u8);

        let vector = Vector::<u8, 20>::from_iter(data);
        assert_eq!(vector[..10], [2u8; 10]);
        assert_eq!(vector[10..], [0u8; 10]);
    }

    #[test]
    fn encode_vector() {
        let data = vec![33u16; COUNT];
        let mut value = Vector::<u16, COUNT>::try_from(data).unwrap();

        value[0] = 34u16;
        assert_eq!(value[0], 34u16);
        value[0] = 33u16;
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
        let expected: Vector<u8, COUNT> = bytes.try_into().expect("test data");
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_variable_vector() {
        const COUNT: usize = 4;
        let mut inner: Vec<List<u8, 1>> = (0..4).map(|i| [i].into_iter().collect()).collect();
        let permutation = &mut inner[3];
        let _ = permutation.pop().expect("test data correct");
        let input: Vector<List<u8, 1>, COUNT> = inner.try_into().expect("test data correct");
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let expected = vec![16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 0, 1, 2];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn roundtrip_vector() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let input: Vector<u8, COUNT> = bytes.try_into().expect("test data");
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Vector::<u8, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }

    #[test]
    fn roundtrip_variable_vector() {
        const COUNT: usize = 4;
        let mut inner: Vec<List<u8, 1>> = (0..4).map(|i| [i].into_iter().collect()).collect();
        let permutation = &mut inner[3];
        let _ = permutation.pop().expect("test data correct");
        let input: Vector<List<u8, 1>, COUNT> = inner.try_into().expect("test data correct");
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Vector::<List<u8, 1>, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }
}
