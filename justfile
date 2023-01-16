test:
    cargo test --all-features
fmt:
    cargo fmt
lint: fmt
    cargo clippy --all-features
build:
    cargo build --all-features
run-ci: lint build test
