use super::field_inspect::{FieldsInspect, FieldsIter};
use crate::{ElementsType, SszTypeClass};
use as_any::AsAny;
use std::any::{Any, TypeId};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SszVariableOrIndex {
    Name(&'static str),
    Index(usize),
}

pub trait SszReflect: AsAny {
    fn ssz_type_class(&self) -> SszTypeClass;

    fn list_elem_type(&self) -> Option<&dyn SszReflect> {
        None
    }

    fn list_length(&self) -> Option<usize> {
        None
    }

    fn as_field_inspectable(&self) -> Option<&dyn FieldsInspect> {
        None
    }
}

/// Converts a path (eg. `[7, "foo", 3]` for `x[7].foo[3]`, `[12, "bar", "__len__"]` for
//  `len(x[12].bar)`) into the generalized index representing its position in the Merkle tree.
pub fn get_generalized_index(
    mut typ: &dyn SszReflect,
    path: &'static [SszVariableOrIndex],
) -> usize {
    let mut root = 1usize;

    for p in path {
        if typ.ssz_type_class() == SszTypeClass::Basic {
            break
        }

        if *p == SszVariableOrIndex::Name("__len__") {
            root = root * 2 + 1;
            typ = &0u64;
        } else {
            let (pos, _, _) = get_item_position(typ, p);
            let base_index = if typ.ssz_type_class() == SszTypeClass::Elements(ElementsType::List) {
                2usize
            } else {
                1
            };
            root = root * base_index * chunk_count(typ).next_power_of_two() + pos;
            typ = get_elem_type(typ, p)
        }
    }

    root
}

/// Return the type of the element of an object of the given type with the given index
/// or member variable name (eg. `7` for `x[7]`, `"foo"` for `x.foo`)
fn get_elem_type<'a>(
    typ: &'a dyn SszReflect,
    index_or_name: &'a SszVariableOrIndex,
) -> &'a dyn SszReflect {
    match (typ.ssz_type_class(), index_or_name) {
        (SszTypeClass::Elements(_) | SszTypeClass::Bits, SszVariableOrIndex::Index(_)) =>
            typ.list_elem_type().expect("illegal operation! type isn't a list!"),
        (SszTypeClass::Container, SszVariableOrIndex::Name(name)) => {
            let inspectable = typ
                .as_field_inspectable()
                .expect("Container should have FieldInspect implemented; qed");
            let (_name, value) = FieldsIter::new(inspectable)
                .find(|(field_name, _)| field_name == name)
                .expect("illegal operation! type isn't a container!");
            value
        },
        (a, b) => panic!("illegal type {a:?} supplied with selector {b:?}."),
    }
}

/// Return three variables:
///     (i) the index of the chunk in which the given element of the item is represented;
///     (ii) the starting byte position within the chunk;
///     (iii) the ending byte position within the chunk.
/// For example: for a 6-item list of uint64 values, index=2 will return (0, 16, 24), index=5 will
/// return (1, 8, 16)
fn get_item_position(
    typ: &dyn SszReflect,
    index_or_name: &SszVariableOrIndex,
) -> (usize, usize, usize) {
    match typ.ssz_type_class() {
        SszTypeClass::Elements(_) | SszTypeClass::Bits =>
            match (index_or_name, typ.list_elem_type()) {
                (SszVariableOrIndex::Index(index), Some(elem_typ)) => {
                    let item_len = item_length(elem_typ.as_any());
                    let start = index * item_len;

                    (start / 32, start % 32, start % 32 + item_len)
                },
                _ => panic!("Shouldn't have done that"),
            },

        SszTypeClass::Container => {
            let field_name = match index_or_name {
                SszVariableOrIndex::Name(name) => name,
                _ => panic!("Can't use index for containers!"),
            };
            let inspector = typ
                .as_field_inspectable()
                .expect("Container should have FieldInspect implemented; qed");
            let (index, (_, value)) = match FieldsIter::new(inspector)
                .enumerate()
                .find(|(_, (name, _))| name == field_name)
            {
                Some(i) => i,
                None => panic!("Can't find {field_name} in container: {}", inspector.struct_name()),
            };

            (index, 0, item_length(value.as_any()))
        },

        SszTypeClass::Basic => panic!("illegal operation, basic types cannot be indexed"),
    }
}

/// Return the number of hashes needed to represent the top-level elements in the given type
/// (eg. `x.foo` or `x[7]` but not `x[7].bar` or `x.foo.baz`). In all cases except lists/vectors
/// of basic types, this is simply the number of top-level elements, as each element gets one
/// hash. For lists/vectors of basic types, it is often fewer because multiple basic elements
/// can be packed into one 32-byte chunk.
fn chunk_count(typ: &dyn SszReflect) -> usize {
    match typ.ssz_type_class() {
        SszTypeClass::Basic => 1,
        SszTypeClass::Container => FieldsIter::new(typ.as_field_inspectable().unwrap()).len(),
        SszTypeClass::Elements(_) => {
            let item = typ.list_elem_type().expect("illegal operation!");
            let len = typ.list_length().expect("illegal operation!");
            (len * item_length(item.as_any()) + 31) / 32
        },
        SszTypeClass::Bits => {
            let len = typ.list_length().expect("illegal operation!");
            (len + 255) / 256
        },
    }
}

/// Return the number of bytes in a basic type, or 32 (a full hash) for compound types.
fn item_length(typ: &dyn Any) -> usize {
    let u8_type_id = TypeId::of::<u8>();
    let u16_type_id = TypeId::of::<u16>();
    let u32_type_id = TypeId::of::<u32>();
    let u64_type_id = TypeId::of::<u64>();
    let u128_type_id = TypeId::of::<u128>();
    let bool_type_id = TypeId::of::<bool>();

    match typ.type_id() {
        b if b == u8_type_id => std::mem::size_of::<u8>(),
        b if b == u16_type_id => std::mem::size_of::<u16>(),
        b if b == u32_type_id => std::mem::size_of::<u32>(),
        b if b == u64_type_id => std::mem::size_of::<u64>(),
        b if b == u128_type_id => std::mem::size_of::<u128>(),
        b if b == bool_type_id => std::mem::size_of::<bool>(),
        _ => 32,
    }
}
