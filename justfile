test:
    cargo test --all-features --workspace --all-targets
    cargo test --doc
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-features --workspace --all-targets
build:
    cargo build --all-features --workspace --all-targets
build-no-std:
    cargo build --no-default-features --workspace --all-targets
run-ci: lint build build-no-std test
