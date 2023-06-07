use crate::{
    error::{InstanceError, TypeError},
    lib::*,
    ser::BYTES_PER_LENGTH_OFFSET,
    SimpleSerialize,
};

/// Deserialization errors.
#[derive(Debug)]
pub enum DeserializeError {
    /// More data was expected to be in the buffer.
    ExpectedFurtherInput {
        provided: usize,
        expected: usize,
    },
    /// The buffer contained more data than expected.
    AdditionalInput {
        provided: usize,
        expected: usize,
    },
    InvalidByte(u8),
    /// An invalid instance was encountered.
    InvalidInstance(InstanceError),
    /// An invalid type was encountered.
    InvalidType(TypeError),
    /// The number of bytes used for length offsets wasn't a multiple of BYTES_PER_LENGTH_OFFSET.
    IncompleteLengthOffsets(usize),
    /// An offset was found with start > end.
    OffsetNotIncreasing {
        start: usize,
        end: usize,
    },
}

impl From<InstanceError> for DeserializeError {
    fn from(err: InstanceError) -> Self {
        Self::InvalidInstance(err)
    }
}

impl From<TypeError> for DeserializeError {
    fn from(err: TypeError) -> Self {
        Self::InvalidType(err)
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::ExpectedFurtherInput { provided, expected } => write!(f, "expected at least {expected} bytes when decoding but provided only {provided} bytes"),
            DeserializeError::AdditionalInput { provided, expected } => write!(f, "{provided} bytes given but only expected {expected} bytes"),
            DeserializeError::InvalidByte(b) => write!(
                f,
                "invalid byte {b:x} when decoding data of the expected type"
            ),
            DeserializeError::InvalidInstance(err) => write!(f, "invalid instance: {err}"),
            DeserializeError::InvalidType(err) => write!(f, "invalid type: {err}"),
            DeserializeError::IncompleteLengthOffsets(err) => write!(f, "incomplete length offsets: {err}"),
            DeserializeError::OffsetNotIncreasing { start, end } => write!(f, "invalid offset points to byte {end} before byte {start}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DeserializeError {}

/// A data structure that can be deserialized using SSZ.
pub trait Deserialize {
    /// Deserialize this value from the given SSZ-encoded buffer.
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

fn deserialize_fixed_homogeneous_composite<T>(encoding: &[u8]) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    let remainder = encoding.len() % T::size_hint();
    if remainder != 0 {
        return Err(DeserializeError::AdditionalInput {
            provided: encoding.len(),
            expected: encoding.len() - remainder,
        })
    }

    let mut elements = vec![];
    for chunk in encoding.chunks_exact(T::size_hint()) {
        let element = T::deserialize(chunk)?;
        elements.push(element);
    }
    Ok(elements)
}

fn deserialize_variable_homogeneous_composite<T>(
    encoding: &[u8],
) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    if encoding.is_empty() {
        return Ok(vec![])
    }

    let offsets_len =
        encoding.get(..BYTES_PER_LENGTH_OFFSET).ok_or(DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: BYTES_PER_LENGTH_OFFSET,
        })?;
    let offsets_len = u32::deserialize(offsets_len)?;
    let offsets_len = offsets_len as usize;
    if encoding.len() < offsets_len {
        return Err(DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: offsets_len,
        })
    }
    if offsets_len % BYTES_PER_LENGTH_OFFSET != 0 {
        return Err(DeserializeError::IncompleteLengthOffsets(offsets_len));
    }

    let offsets = &mut encoding
        .get(..offsets_len)
        .ok_or(DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: offsets_len,
        })?
        .chunks_exact(BYTES_PER_LENGTH_OFFSET)
        .map(|chunk| u32::deserialize(chunk).map(|offset| offset as usize))
        .collect::<Result<Vec<usize>, DeserializeError>>()?;
    offsets.push(encoding.len());

    let element_count = offsets_len / BYTES_PER_LENGTH_OFFSET;
    let mut result = Vec::with_capacity(element_count);
    for span in offsets.windows(2) {
        // index is safe because span is a pair; qed
        let start = span[0];
        let end = span[1];
        if start > end {
            return Err(DeserializeError::OffsetNotIncreasing { start, end });
        }

        // index is safe because start <= end; qed
        let element = T::deserialize(&encoding[start..end])?;
        result.push(element);
    }
    Ok(result)
}

pub fn deserialize_homogeneous_composite<T>(encoding: &[u8]) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    if T::is_variable_size() {
        deserialize_variable_homogeneous_composite(encoding)
    } else {
        deserialize_fixed_homogeneous_composite(encoding)
    }
}
