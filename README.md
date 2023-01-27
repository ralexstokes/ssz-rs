# ssz-rs ✂️️

[![build](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml)

An implementation of the `SSZ` serialization scheme defined in the [consensus-specs repo](https://github.com/ethereum/consensus-specs).

This repo aims to remain lightweight and relatively free-standing, rather than coupled to other ethereum consensus code/dependencies.

# 🚧 WARNING 🚧

This implementation has **not** been audited for security and is primarily intended for R&D use cases.

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Features

To conform to the SSZ spec, a given Rust type should implement the `SimpleSerialize` trait. Types implementing this trait then obtain:

## Encoding / decoding

`ssz_rs` aims to add as little ceremony over the built-in Rust types as possible. The `ssz_rs_derive` crate provides macros to derive the encoding and decoding routines for SSZ containers and unions (represented as Rust `struct`s and `enum`s, respectively).
See the `ssz_rs/examples` for example usage.

## Merkleization

This library provides the hash tree root computation for types implementing `SimpleSerialize`.

## Multiproofs

* *NOTE*: under construction

This library provides tools for generating and verifying multiproofs of SSZ data.

## `no-std` feature

This library is `no-std` compatible. To build without the standard library, disable the crate's default features.

For example,

```toml
ssz-rs = { version = "...", default-features = false }
```

# Testing

This repo includes a copy of the generic SSZ [consensus spec tests](https://github.com/ethereum/consensus-spec-tests) as integration tests for the `ssz_rs` crate.
The tests are generated from a local clone of the spec tests repo and the generator script under `ssz_rs/scripts`.
Refer to the README there if you need to update/change these tests.


# Versioning

This library is still very much in flux so I suggest using the git remote feature of cargo to use this as a dependency for now.

Once things settle down, a version bump will be published to `crates.io`.

This means the following badges refer to **outdated** software. This message will be removed when this is no longer the case.

[![crates.io](https://img.shields.io/crates/v/ssz_rs.svg)](https://crates.io/crates/ssz_rs)
[![crates.io](https://img.shields.io/crates/v/ssz_rs_derive.svg)](https://crates.io/crates/ssz_rs_derive)
[![docs.rs](https://img.shields.io/docsrs/ssz-rs)](https://docs.rs/ssz-rs/)
