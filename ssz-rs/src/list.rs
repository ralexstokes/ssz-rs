use crate::{
    de::{deserialize_homogeneous_composite, Deserialize, DeserializeError},
    error::{Error, InstanceError},
    lib::*,
    merkleization::{
        elements_to_chunks, merkleize, mix_in_length, pack, MerkleizationError, Merkleized, Node,
    },
    ser::{serialize_composite, Serialize, SerializeError},
    SimpleSerialize, Sized,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

/// A homogenous collection of a variable number of values.
#[derive(Clone)]
pub struct List<T: SimpleSerialize, const N: usize> {
    data: Vec<T>,
}

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
        List::<T, N>::try_from(data).map_err(|(_, err)| serde::de::Error::custom(err))
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
            write!(f, "List<{}, {}>(len={}){:#?}", any::type_name::<T>(), N, self.len(), self.data)
        } else {
            write!(f, "List<{}, {}>(len={}){:?}", any::type_name::<T>(), N, self.len(), self.data)
        }
    }
}

impl<T, const N: usize> Default for List<T, N>
where
    T: SimpleSerialize + Default,
{
    fn default() -> Self {
        let data = vec![];
        data.try_into()
            // need to drop data so we do not require it as Debug as required by `expect`
            .map_err(|(_, err)| err)
            .expect("any List can be constructed from an empty Vec")
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
    type Error = (Vec<T>, Error);

    fn try_from(data: Vec<T>) -> Result<Self, Self::Error> {
        if data.len() > N {
            let len = data.len();
            Err((data, Error::Instance(InstanceError::Bounded { bound: N, provided: len })))
        } else {
            Ok(Self { data })
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
// the inner data is not mutated without being able to
// track which elements changed
impl<T, Idx: SliceIndex<[T]>, const N: usize> Index<Idx> for List<T, N>
where
    T: SimpleSerialize,
{
    type Output = <Idx as SliceIndex<[T]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for List<T, N>
where
    T: SimpleSerialize,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
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
            return Err(InstanceError::Bounded { bound: N, provided: self.len() }.into())
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
            return Err(InstanceError::Bounded { bound: N, provided: result.len() }.into())
        }
        let result = result.try_into().map_err(|(_, err)| match err {
            Error::Instance(err) => DeserializeError::InvalidInstance(err),
            _ => unreachable!("no other error variant allowed here"),
        })?;
        Ok(result)
    }
}

impl<T, const N: usize> List<T, N>
where
    T: SimpleSerialize,
{
    fn compute_hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        if T::is_composite_type() {
            let count = self.len();
            let chunks = elements_to_chunks(self.data.iter_mut().enumerate(), count)?;
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
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut { inner: self.data.iter_mut() }
    }
}

pub struct IterMut<'a, T, const N: usize>
where
    T: SimpleSerialize,
{
    inner: slice::IterMut<'a, T>,
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N>
where
    T: SimpleSerialize,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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
        let bytes: Vec<List<u8, 1>> =
            vec![vec![0u8].try_into().unwrap(), Default::default(), vec![1u8].try_into().unwrap()];
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

    #[test]
    fn can_iter_list() {
        let bytes = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
        ];
        let mut input: List<u8, COUNT> = bytes.try_into().unwrap();
        for (i, &value) in input.iter().enumerate() {
            assert_eq!(value as usize, i % 8);
        }
        for value in input.iter_mut() {
            *value = 1;
            assert_eq!(*value, 1);
        }
    }
}
