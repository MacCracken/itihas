# Development Roadmap

> **Current**: 2.5.0 | **Compiler**: cyrius 6.5.36 | **Tests**: 299 + 39 MCP | **Coverage**: 94% | **Lint**: clean

This file tracks **open work only**. Completed releases are recorded in
[CHANGELOG.md](../../CHANGELOG.md) — they are not duplicated here.

Every item is sequenced as its own work-loop cycle (cleanliness + benchmark
gates per CLAUDE.md) unless explicitly marked as batchable.

---

## 2.4.x — Hardening arc (complete)

Every finding the P(-1) sweep confirmed has now been resolved — see CHANGELOG
`[2.4.3]` for `xalloc`, the tracing restoration, and the two project decisions
below. Nothing from this arc is outstanding.

Decisions taken, recorded here because they govern future work:

- **Civilization-name fields are FREE TEXT, not foreign keys.** 72 of 223
  references (32%) name something with no `civ_new` record — both real polities
  outside the curated 53 (`Maurya Empire`, `Carthage`, `Kingdom of England`) and
  non-polity culture labels (`Pre-Pottery Neolithic`, `Italian city-states`). A
  test asserting every reference resolves would fail on correct data, so none
  exists. Documented for consumers in
  [usage.md](../guides/usage.md#civilization-names-are-free-text) and at each
  affected constructor. A reference whose *dates* contradict the record pointing
  at it is still a data bug (two were fixed in 2.4.2).
- **Data tables are exempt from the line-length rule** via per-line
  `#skip-lint`. 352 data rows plus 8 single-line offset/type enums and hoosh's
  two irreducible string constants are exempted; the two genuinely over-long
  *code* lines were wrapped instead. `cyrius lint` on `src/` went 364 warnings →
  **0**, so the count is a usable gate again rather than a permanent backlog.

## 2.5.0 — Tool integration (complete)

Shipped: `src/mcp.cyr` exposes the dataset as five read-only MCP tools over
bote, with the daimon host-registry integration alongside. The `ERR_*` rename
landed with it. See CHANGELOG `[2.5.0]` and
[mcp-port-reference.md](mcp-port-reference.md).

Consumer note, because it is easy to get wrong: **MCP is opt-in.**
`dist/itihas.cyr` carries no bote dependency. A consumer that wants the tools
includes `src/mcp.cyr` explicitly and declares bote itself.

## 2.6.x — Performance arc

Confirmed by the 2.4.2 audit. Each needs its own before/after CSV; none may be
batched.

Precomputed year-sorted views are **done** (Unreleased) — the four range queries
now binary-search a lazily-built sorted view instead of sorting per call, for
−75.4% / −61.5% / −46.4% / −31.2% and −21.3% on the suite sum.

`*_to_json_into` serializer variants are **done** (Unreleased) — all 11 record
writers append to a caller-supplied builder, for −24.8% on collection
serialization with byte-identical output.

`json_escape_into` bulk-run copying is **done** (Unreleased) — a further −26.7%
on the serialization rows, on top of the `_into` split. All three landed with no
regressions elsewhere in the suite. See CHANGELOG.

| Item | Effort | Detail |
|------|--------|--------|
| **Generic filter/lookup helpers** | Large | ~31 near-identical filter bodies across the 10 data modules. One shape per cycle behind the benchmark gate. Do not start before the 2.4.2 token-membership semantics have settled in use. |
| **Static-data footprint** | Large | The data modules embed ~300 KB of inline description strings. Restructuring to runtime `alloc()` touches every data module; demand-gated. |

## 2.7.x — Structural arc

| Item | Effort | Detail |
|------|--------|--------|
| **Native `#derive(accessors)` struct migration** | Large | 9 data modules left — `era` is **done** (Unreleased). The rest address fields via manual `load64(p+OFFSET)` / `store64` against hand-maintained offset enums. avatara (2.5.3) migrated once the 6.x struct field cap rose 32 → 256, making offset-collision bugs a compile error while keeping compat shims. Strictly **one module per cycle**, each with layout assertions and its own A/B. |

**Rule learned from the `era` migration:** delegating `x_field(p)` to the
generated `X_field(p)` adds a call layer, and Cyrius does not inline. Routing
the hot queries through the wrappers cost +12.3% on `eras_containing_500bce`;
calling the generated accessor directly in hot paths put it back to flat. The
`x_*` names stay as the public API. Every remaining module needs the same
treatment and its own before/after A/B.

## Data work (demand-gated)

| Item | Effort | Detail |
|------|--------|--------|
| **Carthage / Khwarezmian civilization records** | Medium | Adding them moves `civ_count` 53 → 54 and the 338-entity total, cascading into tests, `lib.cyr`, README and overview. It would also let the Punic Wars interaction reference a real record instead of free text. A data cycle, not a hardening one. |
| Historical map data (geographic boundaries per era) | Large | — |
| Cultural diffusion tracking | Large | — |
| Historical population estimates | Medium | — |
| Historical climate data correlation | Medium | — |
| Display/formatting functions | Medium | Rust had `Display` impls on all types; the Cyrius equivalent is `*_to_str()`. |
| `Ord`-equivalent chronological ordering | Low | Rust had `Ord` on Era, Event, Campaign. `vec_sort` plus per-type comparators already cover every query path. |

## External dependencies

| Dependency | Repo | Needed for | Status |
|-----------|------|-----------|--------|
| **hoosh** | MacCracken/hoosh | 2.2.0 | ✅ Integrated (`src/hoosh.cyr`) |
| **bote** | MacCracken/bote | 2.5.0 | ✅ **Ready** — 3.3.7, cyrius 6.5.35, builds clean, full surface exported |
| **argonaut** | MacCracken/argonaut | shared JSON library | Ported upstream (424 tests) but **not integrated**. itihas ships its own `src/serial.cyr`, so this is an optional consolidation rather than a gap. |

## Port coverage

The Rust v1.5.0 tree (`rust-old/`) was removed once the port completed; it
remains in git history. The port is **complete** — every Rust module has a
Cyrius counterpart as of 2.5.0. Counts are the current Cyrius totals, which
exceed the Rust originals because the port split accessors into separate
functions.

| Rust module | Cyrius | Fns | Status |
|------------|--------|-----|--------|
| `era.rs` | `era.cyr` | 19 | Complete |
| `civilization.rs` | `civilization.cyr` | 15 | Complete |
| `event.rs` | `event.cyr` | 16 | Complete |
| `figure.rs` | `figure.cyr` | 8 | Complete |
| `causality.rs` | `causality.cyr` | 11 | Complete |
| `interaction.rs` | `interaction.cyr` | 14 | Complete |
| `calendar.rs` | `calendar.cyr` | 9 | Complete |
| `campaign.rs` | `campaign.cyr` | 21 | Complete |
| `site.rs` | `site.cyr` | 11 | Complete |
| `trade.rs` | `trade.cyr` | 12 | Complete |
| `error.rs` | `error.cyr` | 0 (enum only) | Complete |
| `logging.rs` | `logging.cyr` | 2 | Complete (sakshi) |
| `hoosh.rs` | `hoosh.cyr` | 26 | Complete (2.2.0) |
| `lib.rs` | `lib.cyr` | 0 (aggregator) | Complete |
| `mcp.rs` | `mcp.cyr` | 21 | Complete (2.5.0) |
| — | `util.cyr` | 16 | Cyrius-only (shared helpers) |
| — | `serial.cyr` | 27 | Cyrius-only (JSON; replaces serde) |
| — | `main.cyr` | 0 (smoke test) | Complete |
