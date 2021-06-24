use crate::SSZ;
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

pub fn serialize_composite<'a, T, U: 'a>(
    value: T,
    buffer: &mut Vec<u8>,
) -> Result<usize, SerializeError>
where
    T: IntoIterator<Item = &'a U>,
    U: SSZ,
{
    let mut fixed = vec![];
    let mut variable = vec![];

    let mut total_bytes_written = 0;

    let mut element_count = 0;
    for element in value {
        element_count += 1;

        let mut buffer = Vec::with_capacity(U::size_hint());
        let bytes_written = element.serialize(&mut buffer)?;
        total_bytes_written += bytes_written;

        if element.is_variable_size() {
            fixed.push(None);
            variable.push(buffer);
        } else {
            fixed.push(Some(buffer));
            variable.push(vec![]);
        }
    }

    let mut fixed_lengths_sum = 0;
    fixed.iter().for_each(|bytes_opt| {
        let value = if let Some(bytes) = bytes_opt {
            bytes.len()
        } else {
            BYTES_PER_LENGTH_OFFSET
        };
        fixed_lengths_sum += value;
    });
    let variable_lengths = variable.iter().map(|bytes| bytes.len()).collect::<Vec<_>>();

    assert!(fixed_lengths_sum + variable_lengths.iter().sum::<usize>() < MAXIMUM_LENGTH);

    let mut offsets = (0..element_count)
        .into_iter()
        .map(|i| {
            let variable_lengths_sum = variable_lengths[0..i].iter().sum::<usize>();
            let length = (fixed_lengths_sum + variable_lengths_sum) as u32;
            let mut buffer = Vec::with_capacity(4);
            let _ = length
                .serialize(&mut buffer)
                .expect("can serialize simple type");
            total_bytes_written += 4;
            buffer
        })
        .collect::<Vec<_>>();

    for (i, part_opt) in fixed.iter_mut().enumerate() {
        if let Some(part) = part_opt {
            buffer.append(part);
        } else {
            buffer.append(&mut offsets[i]);
        }
    }

    for mut part in variable {
        buffer.append(&mut part);
    }

    Ok(total_bytes_written)
}
