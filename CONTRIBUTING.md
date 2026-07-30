# Contributing

Contributions are welcome! Every little bit helps, and credit will always be given.

## Quick Start

1. Fork the `binaryornot-rs` repo on GitHub.
2. Clone your fork locally:

   ```sh
   git clone git@github.com:your_name_here/binaryornot-rs.git
   cd binaryornot-rs
   ```

3. Make sure you have Rust installed ([rustup.rs](https://rustup.rs)).

4. Create a branch for your changes:

   ```sh
   git checkout -b name-of-your-change
   ```

5. Make your changes and verify they pass checks:

   ```sh
   just check
   ```

   This runs `cargo fmt --check`, `cargo clippy`, and `cargo test`.

6. Commit and push:

   ```sh
   git add .
   git commit -m "Describe your changes"
   git push origin name-of-your-change
   ```

7. Submit a pull request through GitHub.

## Development Commands

All commands are available via [`just`](https://just.systems/man/):

```sh
just list          # Show available commands
just check         # Format + clippy + tests
just fmt           # Format code
just lint          # Run clippy
just test          # Run Rust tests
just test-pyo3     # Run tests with PyO3 bindings
just build         # Build release binary
just run README.md # Run the CLI
just doc           # Generate docs (opens browser)
just wheel         # Build Python wheel with maturin
just dev-install   # Install Python bindings locally
```

## Pull Request Guidelines

1. Include tests for any new functionality.
2. Update `README.md` if you add features.
3. Ensure `just check` passes (format, clippy, tests).
4. Keep the scope narrow — one change per PR when possible.

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` — all clippy warnings must be resolved.
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

## Releasing

1. Bump version in `src/binaryornot-rs/Cargo.toml` and `src/binaryornot-rs/pyproject.toml`.
2. Update `CHANGELOG.md`.
3. Commit and tag:

   ```sh
   git add .
   git commit -m "Release v<version>"
   git tag -a v<version> -m "Release v<version>"
   git push origin main --tags
   ```

4. The publish workflow will build and publish to crates.io and PyPI.

## Code of Conduct

Please note this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to abide by its terms.
