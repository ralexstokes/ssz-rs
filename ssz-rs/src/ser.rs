use crate::{
    error::{InstanceError, TypeError},
    lib::*,
    SimpleSerialize,
};

// NOTE: if this is changed, go change in `ssz_derive` as well!
pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
const MAXIMUM_LENGTH: u64 = 2u64.pow((8 * BYTES_PER_LENGTH_OFFSET) as u32);

/// Serialization errors.
#[derive(Debug)]
pub enum SerializeError {
    /// The encoded length exceeds the maximum.
    MaximumEncodedLengthExceeded(usize),
    /// An invalid instance was encountered.
    InvalidInstance(InstanceError),
    /// An invalid type was encountered.
    InvalidType(TypeError),
    /// An unexpected size sanity check was encountered.
    UnexpectedSize(usize, usize),
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
            SerializeError::MaximumEncodedLengthExceeded(size) => write!(
                f,
                "the encoded length is {size} which exceeds the maximum length {MAXIMUM_LENGTH}",
            ),
            SerializeError::InvalidInstance(err) => write!(f, "invalid instance: {err}"),
            SerializeError::InvalidType(err) => write!(f, "invalid type: {err}"),
            SerializeError::UnexpectedSize(size1, size2) => {
                write!(f, "unexpected size: {size1} not equal to {size2}")
            }
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

pub fn serialize_composite_from_components(
    mut fixed: Vec<Option<Vec<u8>>>,
    mut variable: Vec<u8>,
    variable_lengths: Vec<usize>,
    fixed_lengths_sum: usize,
    buffer: &mut Vec<u8>,
) -> Result<usize, SerializeError> {
    if fixed.len() != variable_lengths.len() {
        return Err(SerializeError::UnexpectedSize(fixed.len(), variable_lengths.len()))
    }

    let total_size = fixed_lengths_sum + variable_lengths.iter().sum::<usize>();
    if total_size as u64 >= MAXIMUM_LENGTH {
        return Err(SerializeError::MaximumEncodedLengthExceeded(total_size))
    }

    // SAFETY: `fixed_lengths_sum` fits in `u32` if the total size check holds
    let mut running_length = fixed_lengths_sum as u32;
    for (part_opt, variable_length) in fixed.iter_mut().zip(variable_lengths) {
        if let Some(part) = part_opt {
            buffer.append(part);
        } else {
            // SAFETY: `variable_length` fits in `u32` if the total size check holds
            let bytes_written = running_length.serialize(buffer)?;
            if bytes_written != BYTES_PER_LENGTH_OFFSET {
                return Err(SerializeError::UnexpectedSize(bytes_written, BYTES_PER_LENGTH_OFFSET))
            }

            running_length += variable_length as u32;
        }
    }

    buffer.append(&mut variable);

    Ok(total_size)
}

pub fn serialize_composite<T: SimpleSerialize>(
    elements: &[T],
    buffer: &mut Vec<u8>,
) -> Result<usize, SerializeError> {
    let mut fixed = vec![];
    let mut variable = vec![];
    let mut variable_lengths = vec![];
    let mut fixed_lengths_sum = 0;

    for element in elements {
        let mut element_buffer = Vec::with_capacity(T::size_hint());
        element.serialize(&mut element_buffer)?;

        let element_buffer_len = element_buffer.len();
        if T::is_variable_size() {
            fixed.push(None);
            fixed_lengths_sum += BYTES_PER_LENGTH_OFFSET;
            variable.append(&mut element_buffer);
            variable_lengths.push(element_buffer_len);
        } else {
            fixed.push(Some(element_buffer));
            fixed_lengths_sum += element_buffer_len;
            variable_lengths.push(0)
        }
    }

    serialize_composite_from_components(
        fixed,
        variable,
        variable_lengths,
        fixed_lengths_sum,
        buffer,
    )
}
