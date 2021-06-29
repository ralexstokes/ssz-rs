use crate::ssz::SSZ;
use thiserror::Error;

pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
pub const MAXIMUM_LENGTH: usize = 2usize.pow((BYTES_PER_LENGTH_OFFSET * 8) as u32);

#[derive(Error, Debug)]
#[error("{0}")]
pub enum SerializeError {}

pub trait Serialize {
    /// Append an encoding of `self` to the `buffer`.
    /// Return the number of bytes written.
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError>;
}

pub fn serialize_composite_from_components(
    mut fixed: Vec<Option<Vec<u8>>>,
    mut variable: Vec<Vec<u8>>,
    variable_lengths: Vec<usize>,
    fixed_lengths_sum: usize,
    buffer: &mut Vec<u8>,
) -> Result<usize, SerializeError> {
    assert!(fixed_lengths_sum + variable_lengths.iter().sum::<usize>() < MAXIMUM_LENGTH);

    let mut total_bytes_written = 0;

    for (i, part_opt) in fixed.iter_mut().enumerate() {
        if let Some(part) = part_opt {
            total_bytes_written += part.len();
            buffer.append(part);
        } else {
            let variable_lengths_sum = variable_lengths[0..i].iter().sum::<usize>();
            let length = (fixed_lengths_sum + variable_lengths_sum) as u32;
            let mut offset_buffer = Vec::with_capacity(4);
            let _ = length
                .serialize(&mut offset_buffer)
                .expect("can serialize simple type");
            buffer.append(&mut offset_buffer);
            total_bytes_written += 4;
        }
    }

    for part in variable.iter_mut() {
        total_bytes_written += part.len();
        buffer.append(part);
    }

    Ok(total_bytes_written)
}

pub fn serialize_composite<T: SSZ>(
    elements: &[T],
    buffer: &mut Vec<u8>,
) -> Result<usize, SerializeError> {
    let mut fixed = vec![];
    let mut variable = vec![];
    let mut variable_lengths = vec![];
    let mut fixed_lengths_sum = 0;

    for element in elements {
        let mut buffer = Vec::with_capacity(T::size_hint());
        element.serialize(&mut buffer)?;

        let buffer_len = buffer.len();
        if T::is_variable_size() {
            fixed.push(None);
            fixed_lengths_sum += BYTES_PER_LENGTH_OFFSET;
            variable.push(buffer);
            variable_lengths.push(buffer_len);
        } else {
            fixed.push(Some(buffer));
            fixed_lengths_sum += buffer_len;
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
