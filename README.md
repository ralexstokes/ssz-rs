# ssz_rs ✂️️

[![build](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/ssz_rs.svg)](https://crates.io/crates/ssz_rs)
[![docs.rs](https://img.shields.io/docsrs/ssz_rs)](https://docs.rs/ssz_rs/)
[![codecov](https://codecov.io/gh/ralexstokes/ssz-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ralexstokes/ssz-rs)

An implementation of the [`SSZ` serialization scheme defined in the consensus-specs repo](https://github.com/ethereum/consensus-specs/tree/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz).

This repo aims to remain lightweight and relatively free-standing, rather than coupled to other ethereum consensus code/dependencies.

# 🚧 WARNING 🚧

This implementation has **not** been audited for security and is primarily intended for R&D use cases.

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Features

To conform to the `SSZ` spec, a given Rust type should implement the [`SimpleSerialize` trait](https://docs.rs/ssz_rs/latest/ssz_rs/trait.SimpleSerialize.html). Types implementing this trait then obtain:

## Encoding / decoding

This library provides routines to serialize from and deserialize into a Rust type to/from the corresponding `SSZ` data via the [`Serialize`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.Serialize.html) and [`Deserialize`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.Deserialize.html) traits.

## Merkleization

This library provides the [hash tree root](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/simple-serialize.md#merkleization) computation for types implementing [`Merkleized`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.Merkleized.html).

* *NOTE*: more sophisticated hashing strategies are possible, users may run into memory or performance issues with the current implementation.

## Multiproofs

This library provides the ability to reason about [generalized indices](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/merkle-proofs.md#generalized-merkle-tree-index) for a given `SSZ` definition,
along with the ability to generate and verify proofs of data at those indices.

* *NOTE*: still under construction

## `no-std` feature

This library is `no-std` compatible. To build without the standard library, disable the crate's default features.

For example, in `Cargo.toml`:

```toml
ssz_rs = { version = "...", default-features = false }
```

## Custom types

This library attempts to provide as minimal an interface over the native Rust types as possible when implementing `SSZ` types.
For example, the `uint64` type from the `SSZ` spec is represented by Rust's native `u64` type.

The library also provides custom types for `List`, `Vector`, `Bitlist` and `Bitvector` following the `SSZ` spec.
Each of these custom types should behave approximately like Rust's `Vec` type. A notable exception is deferring to
the underlying types iteration capabilities; e.g. to iterate a `List` you must explicitly call `.iter()`.

For safety, there are only a few ways to construct an instance of each of these custom types:

* `Default::default`

* `TryFrom::try_from`

* `ssz_rs::Deserialize`

Moreover, the `ssz_rs_derive` package provides macros to derive the encoding and decoding routines for `SSZ` containers and unions (represented as Rust `struct`s and `enum`s, respectively).

# Examples

See the [`examples`](./ssz-rs/examples) for example usage of the facilities of this library.

# Testing

This repo includes a copy of the [`ssz_generic` consensus spec tests](https://github.com/ethereum/consensus-spec-tests) as integration tests for the `ssz_rs` package, along with hand-written unit tests.
The integration tests are generated via a utility under `ssz-rs-test-gen` package. See the README there for further details.
