# Contributing to Itihas

Thank you for your interest in contributing to Itihas.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run `sh tests/test_itihas.sh` to validate
5. Open a pull request

## Prerequisites

- Cyrius >= 3.6.3 (`cyriusly install 3.6.3`)
- cc3 compiler on PATH

## Commands

| Command | Description |
|---------|-------------|
| `sh tests/test_itihas.sh` | Build + run full test suite |
| `cat src/main.cyr \| cc3 > build/itihas` | Manual build |
| `./build/itihas` | Run tests |

## Adding Historical Data

1. Add data entries in the relevant module (e.g., `src/era.cyr`)
2. Use `store64`/`load64` with offset enums for heap struct fields
3. String params annotated `: Str` for auto-coercion
4. Update count assertion in `src/main.cyr`
5. Verify build: `cat src/main.cyr | cc3 > build/itihas`

## Code Style

- One `var` declaration per variable name per function scope
- `(0 - N)` for negative year literals (no negative literals in Cyrius)
- Enum values for struct field offsets (e.g., `ERA_NAME=0; ERA_START=8;`)
- Accessor functions for struct fields (e.g., `fn era_name(p) { return load64(p+ERA_NAME); }`)
- Comments with `#`

## Testing

- Assertions in `src/main.cyr` test harness
- `sh tests/test_itihas.sh` for full build + test cycle
- Target: all data counts verified, all `by_name` lookups working

## Commits

- One logical change per commit
- Descriptive messages

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
