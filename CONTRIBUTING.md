# Contributing to Itihas

Thank you for your interest in contributing to Itihas.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run `cyrius test tests/itihas.tcyr` to validate (and `./scripts/bench-history.sh` if performance is touched)
5. Open a pull request

## Prerequisites

- Cyrius 6.0.50 (`cyriusly install 6.0.50`) — pinned in `cyrius.cyml`

## Commands

| Command | Description |
|---------|-------------|
| `cyrius build src/main.cyr build/itihas` | Build |
| `CYRIUS_DCE=1 cyrius build src/main.cyr build/itihas` | Build (DCE-optimized) |
| `./build/itihas` | Run the smoke test |
| `cyrius test tests/itihas.tcyr` | Run the full 153-assertion suite |
| `./scripts/bench-history.sh` | Run benchmarks + record history |
| `cyrius lint src/*.cyr` | Lint |

## Adding Historical Data

1. Add data entries in the relevant module (e.g., `src/era.cyr`)
2. Use `store64`/`load64` with offset enums for heap struct fields
3. String params annotated `: Str` for auto-coercion
4. Update count assertion in `src/main.cyr`
5. Verify build: `cyrius build src/main.cyr build/itihas && ./build/itihas`

## Code Style

- One `var` declaration per variable name per function scope
- `(0 - N)` for negative year literals (no negative literals in Cyrius)
- Enum values for struct field offsets (e.g., `ERA_NAME=0; ERA_START=8;`)
- Accessor functions for struct fields (e.g., `fn era_name(p) { return load64(p+ERA_NAME); }`)
- Comments with `#`

## Testing

- Full suite in `tests/itihas.tcyr` (run via `cyrius test tests/itihas.tcyr`)
- Smoke test in `src/main.cyr` (run via `./build/itihas`)
- Benchmarks in `tests/itihas.bcyr` (run via `./scripts/bench-history.sh`)
- Target: all data counts verified, all `by_name` lookups working

## Commits

- One logical change per commit
- Descriptive messages

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
