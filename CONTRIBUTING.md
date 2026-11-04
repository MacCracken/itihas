# Contributing to Itihas

Thank you for your interest in contributing to Itihas.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run `cyrius test tests/itihas.tcyr` to validate (and `./scripts/bench-history.sh` if performance is touched)
5. Open a pull request

## Prerequisites

- Cyrius 6.7.18 (`cyriusly install 6.7.18`) — pinned in `cyrius.cyml`

## Commands

| Command | Description |
|---------|-------------|
| `cyrius build src/main.cyr build/itihas` | Build |
| `CYRIUS_DCE=1 cyrius build src/main.cyr build/itihas` | Build (DCE-optimized) |
| `./build/itihas` | Run the smoke test |
| `cyrius test tests/itihas.tcyr` | Run the full unit suite |
| `cyrius build --features mcp tests-mcp/itihas-mcp.tcyr build/itihas_mcp && ./build/itihas_mcp` | Run the MCP suite (needs the optional bote dep) |
| `./scripts/bench-history.sh` | Run benchmarks + record history |
| `cyrius lint src/*.cyr` | Lint (CI fails on any warning other than line length) |
| `cyrius audit` | fmt, lint, doc-comment, test and bench sweep |
| `cyrius distlib` | Regenerate `dist/itihas.cyr` (CI fails if it is stale) |

## Adding Historical Data

1. Add the row to the module's `build_*()` table (e.g., `src/era.cyr`).
2. Years are signed integers in **historical numbering with no year zero**:
   `(0 - 500)` is 500 BCE and `-1` is 1 BCE. Cyrius has no negative literals,
   so write `(0 - N)`. Use `ERA_ONGOING` for "still ongoing".
3. String fields are `: Str` parameters, so a literal is wrapped automatically.
4. Record the source for every new or changed fact in the matching
   `docs/sources/*.md` file.
5. Update the count assertions in `src/main.cyr` and `tests/itihas.tcyr`.
6. Verify: `cyrius build src/main.cyr build/itihas && ./build/itihas`, then the
   full suite.

## Code Style

- One `var` declaration per variable name per function scope.
- Record types: a `#derive(accessors)` struct is the target (see `src/era.cyr`).
  Modules not yet migrated still use an offset enum plus `load64`/`store64`
  accessors. Keep the offset enum in sync with the struct, and pin each field's
  offset in the tests.
- Hot query paths call the generated `X_field(p)` accessors directly, not the
  public `x_field(p)` wrappers: Cyrius does not inline.
- Every public function gets a `#` doc comment directly above it (or above its
  attribute); `cyrius audit` counts the ones that do not.
- `#must_use` on pure functions whose result it is a bug to discard.
- Data rows may exceed 120 characters: mark each one `#skip-lint`. Code lines
  may not; wrap them instead.

## Testing

- Unit suite in `tests/itihas.tcyr`; MCP suite in `tests-mcp/itihas-mcp.tcyr`
  (kept apart because it needs the optional bote dependency).
- Smoke test in `src/main.cyr` (run via `./build/itihas`).
- Benchmarks in `tests/itihas.bcyr`. The harness refuses to time a
  string-argument wrapper that returns nothing, so a miscompiled call cannot
  pass as a fast one.
- Every repaired defect gets a test that fails on the old code.

## Commits

- One logical change per commit
- Descriptive messages

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
