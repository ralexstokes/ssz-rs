use crate::{
    error::{InstanceError, TypeError},
    lib::*,
    Serializable,
};

// NOTE: if this is changed, go change in `ssz_derive` as well!
pub(crate) const BYTES_PER_LENGTH_OFFSET: usize = 4;
const MAXIMUM_LENGTH: u64 = 2u64.pow((8 * BYTES_PER_LENGTH_OFFSET) as u32);

/// Serialization errors.
#[derive(Debug)]
pub enum SerializeError {
    /// The encoded length was at least as big as the maximum length possible.
    MaximumEncodedLengthReached(usize),
    /// An invalid instance was encountered.
    InvalidInstance(InstanceError),
    /// An invalid type was encountered.
    InvalidType(TypeError),
}

impl From<InstanceError> for SerializeError {
    fn from(err: InstanceError) -> Self {
        Self::InvalidInstance(err)
    }
}

impl From<TypeError> for SerializeError {
    fn from(err: TypeError) -> Self {
        Self::InvalidType(err)
    }
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::MaximumEncodedLengthReached(size) => write!(
                f,
                "the encoded length is {size} which meets or exceeds the maximum length {MAXIMUM_LENGTH}",
            ),
            SerializeError::InvalidInstance(err) => write!(f, "invalid instance: {err}"),
            SerializeError::InvalidType(err) => write!(f, "invalid type: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerializeError {}

/// A data structure that can be serialized using SSZ.
pub trait Serialize {
    /// Append an encoding of `self` to the `buffer`.
    ///
    /// Returns the number of bytes written.
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError>;
}

// Part represents either a fixed sized part of the serialization
// or an offset pointing to a variably sized part of the serialization
pub enum Part {
    Fixed(Vec<u8>),
    Offset(usize),
}

#[derive(Default)]
pub struct Serializer {
    parts: Vec<Part>,
    variable: Vec<u8>,
    fixed_lengths_sum: usize,
    variable_lengths_sum: usize,
}

impl Serializer {
    pub fn serialize(mut self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        let total_size = self.fixed_lengths_sum + self.variable_lengths_sum;
        if total_size as u64 >= MAXIMUM_LENGTH {
            return Err(SerializeError::MaximumEncodedLengthReached(total_size))
        }

        // SAFETY: `fixed_lengths_sum` fits in `u32` if the total size check holds
        let mut running_length = self.fixed_lengths_sum as u32;
        for part in self.parts {
            match part {
                Part::Fixed(mut data) => {
                    buffer.append(&mut data);
                }
                Part::Offset(offset) => {
                    let bytes_written = running_length.serialize(buffer)?;
                    debug_assert_eq!(bytes_written, BYTES_PER_LENGTH_OFFSET);

                    // SAFETY: `offset` fits in `u32` if the total size check holds
                    running_length += offset as u32;
                }
            }
        }

        buffer.append(&mut self.variable);

        Ok(total_size)
    }

    pub fn with_element<T: Serializable>(&mut self, element: &T) -> Result<(), SerializeError> {
        let mut element_buffer = Vec::with_capacity(T::size_hint());
        element.serialize(&mut element_buffer)?;

        let element_buffer_len = element_buffer.len();
        if T::is_variable_size() {
            self.parts.push(Part::Offset(element_buffer_len));
            self.variable.append(&mut element_buffer);
            self.fixed_lengths_sum += BYTES_PER_LENGTH_OFFSET;
            self.variable_lengths_sum += element_buffer_len;
        } else {
            self.parts.push(Part::Fixed(element_buffer));
            self.fixed_lengths_sum += element_buffer_len;
        }
        Ok(())
    }
}
