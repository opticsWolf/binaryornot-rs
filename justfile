# Justfile for binaryornot-rs

# Show available commands
list:
    @just --list

alias c := check
alias d := doc
alias r := run
alias t := test

# Run all checks: format, clippy, tests
check:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test

# Format code
fmt:
    cargo fmt

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Run all tests
test:
    cargo test

# Run tests with Python bindings feature
test-py:
    cargo test --features python

# Run tests with output
testv:
    cargo test -- --nocapture

# Build release binary
build:
    cargo build --release

# Run the CLI
run *ARGS:
    cargo run --release -- {{ARGS}}

# Generate documentation (opens in browser)
doc:
    cargo doc --open --no-deps

# Build Python wheel with maturin
wheel:
    maturin build --release

# Install Python bindings locally (editable dev mode)
dev-install:
    maturin develop

# Run Python tests (requires installed wheel)
test-python:
    pytest tests/python/ -v --timeout=60 --timeout-method=thread

# Run benchmark
benchmark:
    python tests/benchmark.py

# Clean build artifacts
clean:
    cargo clean
