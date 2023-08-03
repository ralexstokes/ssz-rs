# ssz_rs_derive

This package provides two proc derive macros: `SimpleSerialize` and `Serializable`.

`SimpleSerialize` derives the functionality required to implement the main package's `SimpleSerialize` trait.

`Serializable` only derives the encoding and decoding routines, in the event a user does not want to pull in the merkleization machinery.

Supports:
- struct (as SSZ container) where each field is also `SimpleSerialize` or `Serializable`
- tuple struct with one field where the field is `SimpleSerialize` or `Serializable`
- enums with "unnamed" and unit members while respecting the rules of SSZ unions

Note: example usage can be found in the tests of the `container` and `union`
modules of the `ssz_rs` crate, along with the `examples` in that crate.
