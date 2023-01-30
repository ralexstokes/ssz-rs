This script `gen.py` will generate the integration tests from a local clone of the [consensus spec tests](https://github.com/ethereum/consensus-spec-tests).

1. Clone `https://github.com/ethereum/consensus-spec-tests`.
2. Install the Python deps, e.g. refer to the [poetry](https://python-poetry.org/) metadata.
3. Run something like:
```bash
$ truncate --size 0 ../tests/$TYPE.rs && python gen.py $TYPE >> ../tests/$TYPE.rs && rustfmt ../tests/$TYPE.rs
```
4. Move test data under the integration tests.
```bash
$ mv ssz-rs/tests/data ../tests && rm -rf ssz-rs
```
where `$TYPE` is one of: `boolean, uints, basic_vector, bitlist, bitvector, containers`.

or to generate new test code, altogether:

```bash
for TYPE in boolean uints basic_vector bitlist bitvector containers; do truncate --size 0 ../tests/$TYPE.rs && python gen.py $TYPE >> ../tests/$TYPE.rs && cargo +nightly fmt ../tests/$TYPE.rs; done
```
