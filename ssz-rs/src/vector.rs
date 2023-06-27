use crate::{
    de::{deserialize_homogeneous_composite, Deserialize, DeserializeError},
    error::{Error, InstanceError, TypeError},
    lib::*,
    merkleization::{elements_to_chunks, merkleize, pack, MerkleizationError, Merkleized, Node},
    ser::{serialize_composite, Serialize, SerializeError},
    SimpleSerialize, Sized,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use std::marker::PhantomData;

/// A homogenous collection of a fixed number of values.
/// NOTE: a `Vector` of length `0` is illegal.
#[derive(Clone)]
pub struct Vector<T: SimpleSerialize, const N: usize> {
    data: Vec<T>,
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
        Vector::<T, N>::try_from(data).map_err(|(_, err)| serde::de::Error::custom(err))
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
    type Error = (Vec<T>, Error);

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err((data, Error::Type(TypeError::InvalidBound(N))))
        }
        if data.len() != N {
            let len = data.len();
            Err((data, Error::Instance(InstanceError::Exact { required: N, provided: len })))
        } else {
            Ok(Self { data })
        }
    }
}

impl<T, const N: usize> fmt::Debug for Vector<T, N>
where
    T: SimpleSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(f, "Vector<{}, {}>{:#?}", any::type_name::<T>(), N, self.data)
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
        // SAFETY: there is currently no way to enforce statically
        // that `N` is non-zero with const generics so panics are possible.
        assert!(N > 0);

        let data = vec![T::default(); N];

        // SAFETY: panic can't happen because data.len() == N != 0; qed
        data.try_into()
            // need to drop data so we do not require it as Debug as required by `expect`
            .map_err(|(_, err)| err)
            .expect("any Vector can be constructed with nonzero default data")
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
// the inner data is not mutated without being able to
// track which elements changed
impl<T, Idx: SliceIndex<[T]>, const N: usize> Index<Idx> for Vector<T, N>
where
    T: SimpleSerialize,
{
    type Output = <Idx as SliceIndex<[T]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: SimpleSerialize,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
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
            return Err(TypeError::InvalidBound(N).into())
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
            return Err(TypeError::InvalidBound(N).into())
        }
        if !T::is_variable_size() {
            let expected_length = N * T::size_hint();
            if encoding.len() < expected_length {
                return Err(DeserializeError::ExpectedFurtherInput {
                    provided: encoding.len(),
                    expected: expected_length,
                })
            }
            if encoding.len() > expected_length {
                return Err(DeserializeError::AdditionalInput {
                    provided: encoding.len(),
                    expected: expected_length,
                })
            }
        }
        let inner = deserialize_homogeneous_composite(encoding)?;
        inner.try_into().map_err(|(_, err)| match err {
            Error::Deserialize(err) => err,
            Error::Instance(err) => DeserializeError::InvalidInstance(err),
            Error::Type(err) => DeserializeError::InvalidType(err),
            _ => unreachable!("no other error variant can be returned at this point"),
        })
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: SimpleSerialize,
{
    fn compute_hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        if T::is_composite_type() {
            let count = self.len();
            let chunks = elements_to_chunks(self.data.iter_mut().enumerate(), count)?;
            merkleize(&chunks, None)
        } else {
            let chunks = pack(&self.data)?;
            merkleize(&chunks, None)
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let inner = self.data.iter_mut();
        IterMut { inner }
    }
}

pub struct IterMut<'a, T: 'a> {
    inner: slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T, const N: usize> Merkleized for Vector<T, N>
where
    T: SimpleSerialize,
{
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        self.compute_hash_tree_root()
    }
}

impl<T, const N: usize> SimpleSerialize for Vector<T, N> where T: SimpleSerialize + Clone {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list::List, serialize};

    const COUNT: usize = 32;

    #[test]
    fn test_try_from() {
        let mut data = vec![2u8; 10];
        data.extend_from_slice(&[0u8; 10]);

        let vector = Vector::<u8, 20>::try_from(data).unwrap();
        assert_eq!(vector[..10], [2u8; 10]);
        assert_eq!(vector[10..], [0u8; 10]);
    }

    #[test]
    #[should_panic]
    fn test_try_from_invalid() {
        let data = vec![2u8; 10];
        let vector = Vector::<u8, 1>::try_from(data).unwrap();
        assert_eq!(vector[0], 2u8);
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
    fn decode_vector_with_no_input() {
        let source = vec![];
        let result = Vector::<u8, 6>::deserialize(&source);
        assert!(matches!(result, Err(DeserializeError::ExpectedFurtherInput { .. })));
    }

    #[test]
    fn decode_variable_vector() {
        const COUNT: usize = 4;
        let mut inner: Vec<List<u8, 1>> =
            Vec::from_iter((0..4).map(|i| List::try_from(vec![i]).unwrap()));
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
        let mut inner: Vec<List<u8, 1>> =
            Vec::from_iter((0..4).map(|i| List::try_from(vec![i]).unwrap()));
        let permutation = &mut inner[3];
        let _ = permutation.pop().expect("test data correct");
        let input: Vector<List<u8, 1>, COUNT> = inner.try_into().expect("test data correct");
        let mut buffer = vec![];
        let _ = input.serialize(&mut buffer).expect("can serialize");
        let recovered = Vector::<List<u8, 1>, COUNT>::deserialize(&buffer).expect("can decode");
        assert_eq!(input, recovered);
    }

    #[test]
    fn can_iter_vector() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let mut input: Vector<u8, COUNT> = bytes.try_into().expect("test data");
        for (i, &value) in input.iter().enumerate() {
            assert_eq!(value as usize, i % 8);
        }
        for value in input.iter_mut() {
            *value = 1;
            assert_eq!(*value, 1);
        }
    }
}
