test:
    cargo test --all-features
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets --all-features
build:
    cargo build --all-targets --all-features
build-no-std:
    cargo build --no-default-features
run-ci: lint build test
