# ssz-rs-test-gen

Utility for generating the integration tests for `ssz_rs`.

## How to use

```bash
just clean
just download-integration-tests
just generate-all
```

## How to update

The `ssz-rs-test-gen` utility provides the exact same set of effects for a given version of the `consensus-spec-tests`
and so should only need to be run when the underlying test corpus changes.

To update to a newer set of tests, update the [`spec-test-version`](./spec-test-version) file and re-run the above steps.
