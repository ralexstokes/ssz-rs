test:
    cargo test --all-features
fmt:
    cargo fmt
lint: fmt
    cargo clippy
build:
    cargo build
run-ci: lint build test
