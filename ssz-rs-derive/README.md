# ssz_rs_derive

A proc macro that derives the `SimpleSerialize` trait.

Supports:
- struct (as SSZ container) where each field is also `SimpleSerialize`
- tuple struct with one field where the field is `SimpleSerialize`
- enums with "unnamed" and unit members while respecting the rules of SSZ unions

Note: example usage can be found in the tests of the `container` and `union`
modules of the `ssz_rs` crate, along with the `examples` in that crate.
