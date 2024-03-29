test:
    cargo test --all-features --all-targets
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets --all-features
build:
    cargo build --all-targets --all-features
build-no-std:
    cargo build --no-default-features --all-targets
run-ci: lint build build-no-std test
