use crate::de::Deserialize;
use crate::ser::Serialize;

pub trait SSZ: Serialize + Deserialize {
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    fn size_hint() -> usize;
}
