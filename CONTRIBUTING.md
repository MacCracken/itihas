# Contributing to Itihas

Thank you for your interest in contributing to Itihas.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run `make check` to validate
5. Open a pull request

## Prerequisites

- Rust stable (MSRV 1.89)
- Components: `rustfmt`, `clippy`
- Optional: `cargo-audit`, `cargo-deny`, `cargo-llvm-cov`

## Makefile Targets

| Command | Description |
|---------|-------------|
| `make check` | fmt + clippy + test + audit |
| `make fmt` | Check formatting |
| `make clippy` | Lint with `-D warnings` |
| `make test` | Run test suite |
| `make audit` | Security audit |
| `make deny` | Supply chain checks |
| `make bench` | Run benchmarks with history tracking |
| `make coverage` | Generate coverage report |
| `make doc` | Build documentation |

## Adding Historical Data

1. Add pre-built data in the relevant module (e.g., `era::all_eras()`)
2. Use `Cow::Borrowed` for all string fields in static data
3. Add serde roundtrip tests
4. Update README module table if needed
5. Add benchmarks for lookup functions

## Code Style

- `cargo fmt` — mandatory
- `cargo clippy -- -D warnings` — zero warnings
- Doc comments on all public items
- `#[non_exhaustive]` on public enums
- No `unsafe` code
- No `println!` — use `tracing` for logging

## Testing

- Unit tests colocated in modules (`#[cfg(test)] mod tests`)
- Integration tests in `tests/`
- Feature-gated tests with `#[cfg(feature = "...")]`
- Target: 80%+ line coverage

## Commits

- Use conventional-style messages
- One logical change per commit

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
