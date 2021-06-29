mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod list;
mod ser;
mod ssz;
mod uint;
mod vector;

pub use crate::ssz::SSZ;
pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use list::List;
pub use ser::{Serialize, SerializeError};
pub use ssz_derive::*;
pub use uint::U256;
pub use vector::Vector;

pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: SSZ,
{
    let mut result = vec![];
    value.serialize(&mut result)?;
    Ok(result)
}

pub fn deserialize<T>(encoding: &[u8]) -> Result<T, DeserializeError>
where
    T: SSZ,
{
    T::deserialize(encoding)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_demonstration() {
        // #[derive(Serialize)]
        // struct Foo {
        //     a: List<u8, 32>,
        //     b: Vector<u16, 32>,
        //     c: bool,
        //     d: Uint256,
        // }

        // let mut f = Foo::default();
        // f.a.push(32u8);
        // f.b[22] = 2021u16;
        // f.c = true;
        // f.d = Uint256([12u8; 32]);

        // let mut buffer = vec![];
        // let _ = f.serialize(&mut buffer).expect("it works");
        // let another_f: Foo = ssz::deserialize(&buffer);
        // assert_eq!(f, another_f);
    }
}
