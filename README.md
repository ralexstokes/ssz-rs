# ssz

An implementation of the `ssz` serialization scheme defined in the [eth2.0-specs repo](https://github.com/ethereum/eth2.0-specs).

# Features

## serialization

- uintN
  - [x] encoding
  - [x] decoding
- bool
  - [x] encoding
  - [x] decoding
- vector
  - [x] encoding
  - [x] decoding
- list
  - [x] encoding
  - [x] decoding
- bitvector
  - [x] encoding
  - [x] decoding
- bitlist
  - [x] encoding
  - [x] decoding
- container
  - [x] derive macro
  - [x] encoding
  - [x] decoding
- union
  - [x] derive macro
  - [x] encoding
  - [x] decoding
- other
  - [ ] harden deserialization
  - [ ] turn asserts into errors
  - [ ] run against conformance tests
  - [ ] ensure variants in derive and others
  - [ ] polish derive infra

## Merkleization

- [ ] TODO

## (multi)proofs

- [ ] TODO
