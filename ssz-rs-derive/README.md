# ssz_rs_derive

This package provides the following proc derive macros:
* `Serializable`
* `Merkleized`
* `SimpleSerialize`

`SimpleSerialize` derives the functionality required to implement the main package's `SimpleSerialize` trait.

`Serializable` only derives the encoding and decoding routines, in the event a user does not want to pull in the merkleization machinery.

`Merkleized` only provides an implementation of that trait, if your type only needs to provide the hashing functionality.

Supports:
- struct where each field is also `SimpleSerialize` or `Serializable`
- enums with "unnamed" and unit members while respecting the rules of SSZ unions
- tuple struct with one field where the field is `SimpleSerialize` or `Serializable`
- enums in "wrapper" mode, requiring the `transparent` attribute.

Derivations on structs provide implementations of the relevant traits for a custom struct definition to represent a SSZ container type.

Derivations on enums (without `transparent`) provide implementations of the relevant traits for SSZ union types.

Derivations on tuple structs facilitates the "newtype" pattern and delegates to the inner type for its implementation of the relevant traits.

Derivations on enums *with* `transparent` supports delegation to the inner variants for the implementation of the relevant traits.

Example usage can be found in the tests of the `container` and `union` modules of the `ssz_rs` crate, along with the `examples` in that crate.
