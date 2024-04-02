# ssz_rs ✂️️

[![build](https://github.com/ralexstokes/ssz-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ralexstokes/ssz-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ssz_rs.svg)](https://crates.io/crates/ssz_rs)
[![docs.rs](https://img.shields.io/docsrs/ssz_rs)](https://docs.rs/ssz_rs/)
[![codecov](https://codecov.io/gh/ralexstokes/ssz-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ralexstokes/ssz-rs)

An implementation of the [`SSZ` serialization scheme defined in the consensus-specs repo](https://github.com/ethereum/consensus-specs/tree/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz).

This repo aims to remain lightweight and relatively free-standing, rather than coupled to other ethereum consensus code/dependencies. It also supports light client use cases with functionality for Merkle proofs and reasoning about generalized indices.

# 🚧 WARNING 🚧

This implementation is primarily intended for R&D use cases and comes with no guarantees including those around performance or security.

That being said, an audit has been completed by [@oak-security](https://github.com/oak-security) you can [find here](https://github.com/oak-security/audit-reports/blob/master/ssz-rs/2023-09-28%20Audit%20Report%20-%20ssz-rs%20v1.0.pdf).
It covers this commit of this repository: [b8729699f07f0d348053251dd6ddf838656849d1](https://github.com/ralexstokes/ssz-rs/commit/b8729699f07f0d348053251dd6ddf838656849d1)

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Features

To conform to the `SSZ` spec, a given Rust type should implement the [`SimpleSerialize` trait](https://docs.rs/ssz_rs/latest/ssz_rs/trait.SimpleSerialize.html). Types implementing this trait then obtain:

## Encoding / decoding

This library provides routines to serialize from and deserialize into a Rust type to/from the corresponding `SSZ` data via the [`Serialize`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.Serialize.html) and [`Deserialize`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.Deserialize.html) traits.

## Merkleization

This library provides the [hash tree root](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/simple-serialize.md#merkleization) computation for types implementing [`HashTreeRoot`](https://docs.rs/ssz_rs/latest/ssz_rs/trait.HashTreeRoot.html).

* *NOTE*: The hashing strategies employed are not particularly sophisticated; users may run into memory or performance issues with the current implementation.

## Merkle proofs

This library provides the ability to reason about [generalized indices](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/merkle-proofs.md#generalized-merkle-tree-index) for a given `SSZ` definition,
along with the ability to generate and verify proofs of data at those indices.

* *NOTE*: Merkle proving is implemented for the "single" proof category, with only experimental support for "multiproofs" defined in the `SSZ` spec.

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
Each of these custom types should behave approximately like Rust's `Vec` type.

For safety, there are only a few ways to construct an instance of each of these custom types:

* `Default::default`

* `TryFrom::try_from`

* `ssz_rs::Deserialize`

Moreover, the `ssz_rs_derive` package provides macros to derive the various trait implementations for `SSZ` containers and unions (represented as Rust `struct`s and `enum`s, respectively).

# Examples

See the [`examples`](./ssz-rs/examples) for example usage of the facilities of this library. There are additional samples of how to use the code in the tests, if the examples don't capture your use case.

# Testing

This repo includes a copy of the [`ssz_generic` consensus spec tests](https://github.com/ethereum/consensus-spec-tests) as integration tests for the `ssz_rs` package, along with hand-written unit tests.
The integration tests are generated via a utility under `ssz-rs-test-gen` package. See the README there for further details.
