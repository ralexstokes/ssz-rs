use crate::de::Deserialize;
use crate::ser::Serialize;

pub trait SSZ: Serialize + Deserialize {
    // is this type variable or fixed size?
    fn is_variable_size(&self) -> bool;

    // how big should this type be when serialized?
    fn size_hint() -> usize
    where
        Self: Sized;
}
