# ssz_rs ✂️

An implementation of the `ssz` serialization scheme defined in the [consensus-specs repo](https://github.com/ethereum/consensus-specs).

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
  - [ ] harden deserialization, strengthen typing, bounds assertions
  - [ ] run against conformance tests
  - [ ] other spec conformance? (invariants, etc)

## merkleization

- [x] naive hash tree root
- [x] variant with virtualized padding hashes
- [ ] cache the hash tree root calculation

## (multi)proofs

- [ ] TODO
