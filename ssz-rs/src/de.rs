use crate::{
    error::{InstanceError, TypeError},
    lib::*,
    ser::BYTES_PER_LENGTH_OFFSET,
    Serializable,
};

/// Deserialization errors.
#[derive(Debug)]
pub enum DeserializeError {
    /// More data was expected to be in the buffer.
    ExpectedFurtherInput { provided: usize, expected: usize },
    /// The buffer contained more data than expected.
    AdditionalInput { provided: usize, expected: usize },
    /// An invalid byte was encountered when deserializing the given type
    InvalidByte(u8),
    /// An invalid instance was encountered.
    InvalidInstance(InstanceError),
    /// An invalid type was encountered.
    InvalidType(TypeError),
    /// The number of bytes used for length offsets wasn't a multiple of BYTES_PER_LENGTH_OFFSET.
    InvalidOffsetsLength(usize),
    /// An offset was found with start > end.
    OffsetNotIncreasing { start: usize, end: usize },
    /// An offset was absent when expected.
    MissingOffset,
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
            DeserializeError::ExpectedFurtherInput { provided, expected } => write!(f, "expected at least {expected} byte(s) when decoding but provided only {provided} byte(s)"),
            DeserializeError::AdditionalInput { provided, expected } => write!(f, "{provided} byte(s) given but only expected (up to) {expected} byte(s)"),
            DeserializeError::InvalidByte(b) => write!(
                f,
                "invalid byte {b:x} when decoding data of the expected type"
            ),
            DeserializeError::InvalidInstance(err) => write!(f, "invalid instance: {err}"),
            DeserializeError::InvalidType(err) => write!(f, "invalid type: {err}"),
            DeserializeError::InvalidOffsetsLength(len) => write!(f, "the offsets length provided {len} is not a multiple of the size per length offset {BYTES_PER_LENGTH_OFFSET} bytes"),
            DeserializeError::OffsetNotIncreasing { start, end } => write!(f, "invalid offset points to byte {end} before byte {start}"),
            DeserializeError::MissingOffset => write!(f, "an offset was missing when deserializing a variable-sized type"),
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
    T: Serializable,
{
    // NOTE: Callers have already validated `encoding` is correctly sized
    debug_assert_eq!(encoding.len() % T::size_hint(), 0);

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
    T: Deserialize,
{
    if encoding.is_empty() {
        return Ok(vec![])
    }

    let offsets_len = encoding.get(..BYTES_PER_LENGTH_OFFSET).ok_or({
        DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: BYTES_PER_LENGTH_OFFSET,
        }
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
        return Err(DeserializeError::InvalidOffsetsLength(offsets_len))
    }

    let offsets = &mut encoding[..offsets_len]
        .chunks_exact(BYTES_PER_LENGTH_OFFSET)
        .map(|chunk| u32::deserialize(chunk).map(|offset| offset as usize))
        .collect::<Result<Vec<usize>, DeserializeError>>()?;
    offsets.push(encoding.len());

    let element_count = offsets_len / BYTES_PER_LENGTH_OFFSET;
    let mut result = Vec::with_capacity(element_count);
    for span in offsets.windows(2) {
        // SAFETY: index is safe because span is a pair; qed
        let start = span[0];
        let end = span[1];
        if start > end {
            return Err(DeserializeError::OffsetNotIncreasing { start, end })
        }

        // SAFETY: index is safe because start <= end; qed
        let element = T::deserialize(&encoding[start..end])?;
        result.push(element);
    }
    Ok(result)
}

pub fn deserialize_homogeneous_composite<T>(encoding: &[u8]) -> Result<Vec<T>, DeserializeError>
where
    T: Serializable,
{
    if T::is_variable_size() {
        deserialize_variable_homogeneous_composite(encoding)
    } else {
        deserialize_fixed_homogeneous_composite(encoding)
    }
}

#[derive(Debug)]
enum Segment {
    Fixed(usize, usize),
    Offset,
}

// `ContainerDeserializer` facilitates the deserialization of possibly variable
// heterogenous composite types.
// Intended use:
// - call `parse` for each field of the container along with the input to decode
// - call `finalize` after parsing each field, and use the resulting spans into the input to
//   deserialize each individual field
// NOTE: mainly intended for private use in the proc derive macro.
#[derive(Debug, Default)]
pub struct ContainerDeserializer {
    segments: Vec<Segment>,
    offsets: Vec<usize>,
    total_bytes_read: usize,
}

impl ContainerDeserializer {
    // NOTE: segments must be parsed in order following the order of the fields of the container.
    pub fn parse<T: Serializable>(&mut self, encoding: &[u8]) -> Result<(), DeserializeError> {
        let start = self.total_bytes_read;
        if T::is_variable_size() {
            let end = start + BYTES_PER_LENGTH_OFFSET;

            let target =
                encoding.get(start..end).ok_or(DeserializeError::ExpectedFurtherInput {
                    provided: encoding.len() - start,
                    expected: BYTES_PER_LENGTH_OFFSET,
                })?;
            let next_offset = u32::deserialize(target)? as usize;

            if let Some(previous_offset) = self.offsets.last() {
                if next_offset < *previous_offset {
                    return Err(DeserializeError::OffsetNotIncreasing {
                        start: *previous_offset,
                        end: next_offset,
                    })
                }

                if *previous_offset > encoding.len() {
                    return Err(DeserializeError::ExpectedFurtherInput {
                        provided: encoding.len() - previous_offset,
                        expected: next_offset - previous_offset,
                    })
                }

                if next_offset > encoding.len() {
                    return Err(DeserializeError::ExpectedFurtherInput {
                        provided: encoding.len() - next_offset,
                        expected: next_offset - previous_offset,
                    })
                }
            }

            self.total_bytes_read += BYTES_PER_LENGTH_OFFSET;
            self.offsets.push(next_offset);
            self.segments.push(Segment::Offset);
        } else {
            let encoded_length = T::size_hint();
            let end = self.total_bytes_read + encoded_length;
            if encoding.len() < self.total_bytes_read {
                return Err(DeserializeError::ExpectedFurtherInput {
                    provided: encoding.len(),
                    expected: self.total_bytes_read,
                })
            }
            if encoding.len() < end {
                return Err(DeserializeError::ExpectedFurtherInput {
                    provided: encoding.len() - self.total_bytes_read,
                    expected: encoded_length,
                })
            }

            self.total_bytes_read += encoded_length;
            self.segments.push(Segment::Fixed(start, end));
        };
        Ok(())
    }

    // Assembles a validated list of (pairs of) indices into `encoding` that point to the
    // slice containing the encoding for each field of the target container.
    // For example, if some container has three fields, the result will have 6 indices into
    // `encoding` for the (start, end) of the encoding of each field.
    pub fn finalize(mut self, encoding: &[u8]) -> Result<Vec<usize>, DeserializeError> {
        self.offsets.push(encoding.len());

        let mut spans = vec![];
        let mut offsets = &self.offsets[..];
        for segment in self.segments {
            match segment {
                Segment::Fixed(start, end) => {
                    spans.push(start);
                    spans.push(end);
                }
                Segment::Offset => {
                    let start = offsets.first().ok_or(DeserializeError::MissingOffset)?;
                    let end = offsets.get(1).ok_or(DeserializeError::MissingOffset)?;

                    if encoding.len() < *start {
                        return Err(DeserializeError::ExpectedFurtherInput {
                            provided: encoding.len(),
                            expected: *start,
                        })
                    }
                    if encoding.len() < *end {
                        return Err(DeserializeError::ExpectedFurtherInput {
                            provided: encoding.len(),
                            expected: *end,
                        })
                    }

                    self.total_bytes_read += end - start;
                    spans.push(*start);
                    spans.push(*end);
                    offsets = &offsets[1..];
                }
            }
        }
        if self.total_bytes_read > encoding.len() {
            return Err(DeserializeError::ExpectedFurtherInput {
                provided: encoding.len(),
                expected: self.total_bytes_read,
            })
        }

        if self.total_bytes_read < encoding.len() {
            return Err(DeserializeError::AdditionalInput {
                provided: encoding.len(),
                expected: self.total_bytes_read,
            })
        }

        Ok(spans)
    }
}
