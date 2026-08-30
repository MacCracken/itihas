# Development Roadmap

> **Current**: 2.4.2 | **Compiler**: cyrius 6.5.36 | **Tests**: 299 | **Coverage**: 94%

This file tracks **open work only**. Completed releases are recorded in
[CHANGELOG.md](../../CHANGELOG.md) — they are not duplicated here.

Every item is sequenced as its own work-loop cycle (cleanliness + benchmark
gates per CLAUDE.md) unless explicitly marked as batchable.

---

## 2.4.x — Hardening arc (in progress)

The P(-1) sweep landed in 2.4.2. These are findings it confirmed but did not
batch, in the order they should be taken. **`xalloc` (CWE-690) and the tracing
restoration are done** — see CHANGELOG `[Unreleased]`. What remains needs a
project decision before any code changes, so both are stated as decisions:

| Item | Effort | Detail |
|------|--------|--------|
| **Referential-integrity policy for civ-name fields** | Medium | **Decision needed.** Many figure/site/route records name civilizations with no `civ_new` record. Most are deliberate free-text culture labels (`Upper Paleolithic`, `Neolithic Britain`), not broken keys — so a blanket validation test would fail on correct data. Options: (a) declare these fields free text and add no test; (b) split them into a typed `civ_ref` (validated) and a free-text `culture` field; (c) keep one field and maintain an allow-list of non-civ labels. Nothing should be written until one is chosen. |
| **Line-length lint (364 warnings)** | Low | **Decision needed.** Almost entirely data-table rows whose long description strings read better unwrapped; `src/util.cyr` is already clean and shows the code style is not the problem. Options: (a) raise the limit for `src/*.cyr` data modules; (b) `#skip-lint` the data tables; (c) accept the warnings and stop treating the count as a gate. |

## 2.5.0 — Tool integration (unblocked)

**bote 3.3.7 ships everything required.** The former "blocked on bote toolchain
bring-up" note is resolved: bote is pinned to cyrius 6.5.35, builds clean, and
`dist/bote.cyr` exports the full surface. This is ordinary work now, not a wait.

Port specification: [mcp-port-reference.md](mcp-port-reference.md).

| # | Item | Effort | Detail |
|---|------|--------|--------|
| 1 | **mcp module** | Medium | `tool_definitions()`, `register_handlers()`, `register_all()`. Five read-only tools: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`. |
| 2 | **daimon module** | Medium | `register_tools()`, `host_tool_descriptions()`, `invoke()`. Agent-orchestrator integration via bote's host registry. |
| 3 | **`ERR_*` → `ITIHAS_ERR_*`** | Medium | `cyrius lint` flags all 10 bare `ERR_*` members: a leaf library must prefix, or the flat enum constants collide across libs. itihas is included by 7 consumers, so this is **breaking** and belongs with the minor bump rather than a patch. The codes are currently unused by itihas itself. |

bote API mapping, verified against bote 3.3.7:

| itihas needs | bote provides |
|--------------|---------------|
| `ToolDef::new` | `tool_def_new(name, description, input_schema)` |
| `ToolSchema::new` | `schema_new(schema_type, properties_vec, required_vec)` + `schema_prop_new(key, value)` |
| `ToolAnnotations::read_only()` | `ann_read_only()` |
| `registry.register(def)` | `registry_register(registry, tool_def)` |
| handler binding | `dispatcher_handle(dispatcher, name, &fn)`, `dispatcher_registry(dispatcher)` |
| `McpHostRegistry` | `host_registry_new()`, `host_registry_add(r, entry)`, `host_entry_new(name, url)` |

`bote/src/libro_tools.cyr :: libro_tools_register()` is a complete working
example of this exact registration pattern — use it as the reference.

## 2.6.x — Performance arc

Confirmed by the 2.4.2 audit. Each needs its own before/after CSV; none may be
batched.

| Item | Effort | Detail |
|------|--------|--------|
| **Precomputed year-sorted views** | Large | `events_between` re-sorts immutable data on every call (~1.4 µs of its 2.6 µs). Also applies to `civs_active_at`, `eras_containing`, `campaigns_between`. Changes data-structure lifetime and touches the four hottest benchmarks. |
| **`*_to_json_into(sb, p)` serializer variants** | Large | Every `*_to_json` allocates a private builder and finished `Str`, so `_json_array` copies each record's bytes twice. Adds ~11 functions and rewrites every call site; byte-identical output must be proven per type before the next lands. |
| **Generic filter/lookup helpers** | Large | ~31 near-identical filter bodies across the 10 data modules. One shape per cycle behind the benchmark gate. Do not start before the 2.4.2 token-membership semantics have settled in use. |
| **Static-data footprint** | Large | The data modules embed ~300 KB of inline description strings. Restructuring to runtime `alloc()` touches every data module; demand-gated. |

## 2.7.x — Structural arc

| Item | Effort | Detail |
|------|--------|--------|
| **Native `#derive(accessors)` struct migration** | Large | The 10 data modules plus `serial` address fields via ~242 manual `load64(p+OFFSET)` / `store64` calls against hand-maintained offset enums. avatara (2.5.3) migrated once the 6.x struct field cap rose 32 → 256, making offset-collision bugs a compile error while keeping compat shims. Strictly **one module per cycle**. The `CAMP_START`/`CAMP_END` assertions are currently the only guard against an offset renumber. |

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
remains in git history. Counts are the current Cyrius totals, which exceed the
Rust originals because the port split accessors into separate functions.

| Rust module | Cyrius | Fns | Status |
|------------|--------|-----|--------|
| `era.rs` | `era.cyr` | 17 | Complete |
| `civilization.rs` | `civilization.cyr` | 13 | Complete |
| `event.rs` | `event.cyr` | 14 | Complete |
| `figure.rs` | `figure.cyr` | 8 | Complete |
| `causality.rs` | `causality.cyr` | 11 | Complete |
| `interaction.rs` | `interaction.cyr` | 14 | Complete |
| `calendar.rs` | `calendar.cyr` | 9 | Complete |
| `campaign.rs` | `campaign.cyr` | 19 | Complete |
| `site.rs` | `site.cyr` | 11 | Complete |
| `trade.rs` | `trade.cyr` | 12 | Complete |
| `error.rs` | `error.cyr` | 0 (enum only) | Complete |
| `logging.rs` | `logging.cyr` | 2 | Complete (sakshi) |
| `hoosh.rs` | `hoosh.cyr` | 27 | Complete (2.2.0) |
| `lib.rs` | `lib.cyr` | 0 (aggregator) | Complete |
| `mcp.rs` | — | — | **Open** — unblocked, see 2.5.0 |
| — | `util.cyr` | 11 | Cyrius-only (shared helpers) |
| — | `serial.cyr` | 14 | Cyrius-only (JSON; replaces serde) |
| — | `main.cyr` | 0 (smoke test) | Complete |
