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

# Run tests with PyO3 bindings
test-pyo3:
    cargo test --features pyo3-ext

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
    maturin build --release --features pyo3-ext

# Install Python bindings locally
dev-install:
    maturin develop --features pyo3-ext

# Run Python tests (requires installed wheel)
test-python:
    pytest tests/

# Run benchmark
benchmark:
    python tests/benchmark.py

# Clean build artifacts
clean:
    cargo clean
