# ssz_rs ✂️

An implementation of the `ssz` serialization scheme defined in the [consensus-specs repo](https://github.com/ethereum/consensus-specs).

This repo aims to remain lightweight and relatively free-standing, rather than coupled to other ethereum consensus code/dependencies.

# 🚧 WARNING 🚧

This implementation has **not** been audited for security and is primarily intended for R&D use cases.

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Features

To conform to the SSZ spec, a given Rust type should implement the `SimpleSerialize` trait. Types implementing this trait then obtain:

## Encoding / decoding

`ssz_rs` aims to add as little ceremony over the built-in Rust types as possible. The `ssz_rs_derive` crate provides macros to derive the encoding and decoding routines for SSZ containers and unions (represented as Rust `struct`s and `enum`s, respectively). See the `ssz-rs/examples` for example usage.

## Merkleization

This library provides the hash tree root computation for types implementing `SimpleSerialize`.

## Multiproofs

* *NOTE*: under construction

This library provides tools for generating and verifying multiproofs of SSZ data.

# Testing

This repo includes a copy of the generic SSZ [consensus spec tests](https://github.com/ethereum/consensus-spec-tests) as integration tests for the `ssz_rs` crate. The tests are generated from a local clone of the spec tests repo and the generator script under `ssz-rs/scripts`. Refer to the README there if you need to update/change these tests.
