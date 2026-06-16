# Development Roadmap

> **Status**: v2.3.5 released | **Current**: 2.3.5 | **Compiler**: cyrius 6.2.11

Completed items are in [CHANGELOG.md](../../CHANGELOG.md).
Rust benchmark baseline in [benchmarks-rust-v-cyrius.md](../../benchmarks-rust-v-cyrius.md).

## Completed in 2.0.0

- [x] Port all 10 data modules from Rust to Cyrius (128 functions)
- [x] 297 historical entities across 10 modules
- [x] All Rust query functions ported (by_name, by_region, active_at, by_category, etc.)
- [x] `region_proximity()` with `split_regions()` helper
- [x] `causal_chain()` BFS traversal with depth tracking
- [x] `logging` module via sakshi (`itihas_log_init`, `itihas_log_init_level`)
- [x] 97-assertion test suite (counts, lookups, filters, integration, boundary checks)
- [x] 141KB static ELF binary, no external dependencies
- [x] Rust v1.5.0 source preserved in `rust-old/`

## v2.1.0 — Data, Quality & Serialization

| # | Item | Effort | Details |
|---|------|--------|---------|
| 1 | **Restore description fields** | Medium | cc3 v4.0.0 raised str_data to 256KB (was 32KB). Restore inline descriptions to all 10 data modules from Rust source. |
| 2 | **Case-insensitive lookups** | Low | Add `str_lower()` to lib/str.cyr, update all `by_name()` functions. |
| 3 | **Chronological sort** | Low | Add `vec_sort()` to lib/vec.cyr, apply to `events_between()` and other range queries. |
| 4 | **argonaut integration** | Medium | JSON serialization for all types. argonaut v1.2.0 is ported; needs itihas struct integration. |
| 5 | **`.bcyr` benchmark harness** | Medium | Port 28 criterion benchmarks to Cyrius bench format. Enables direct Rust-vs-Cyrius timing comparison. |

## Completed in 2.2.0

- [x] hoosh module: `answer_from_data()`, `llm_answer()`, `parse_tool_call()`, `resolve_era_lookup()`, 6 tool definitions, HTTP POST client
- [x] 30 new tests (153 total)

## Completed in 2.3.0 — Toolchain/CI Modernization

- [x] Cyrius compiler pin `4.0.0` → `6.0.50` (clean build, no source changes required)
- [x] Manifest migrated `cyrius.toml` → `cyrius.cyml` (`${file:VERSION}`, `repository`, single-source toolchain pin)
- [x] CI/release modernized on the avatara pattern (`install.sh` from the pin, `cyrius lint` hard-gate, `CYRIUS_DCE=1` build, ELF verify, version-consistency, `SHA256SUMS`, changelog-extracted release body)
- [x] `.cyrius-toolchain` retired; stale `cc3` tooling fixed in CLAUDE.md; mandatory per-release benchmark gate added

## v2.3.x — Modernization Arc (planned)

Deeper modernizations beyond the 2.3.0 pin/CI work. Each is its own work-loop cycle
(cleanliness + benchmark gates), in dependency order:

| Ver | Item | Effort | Details |
|-----|------|--------|---------|
| 2.3.1 | **Test + benchmark harnesses** | Large | ✅ Released in 2.3.1. Ported `main.cyr`'s 153 inline asserts to `tests/itihas.tcyr` (`cyrius test`); relocated benchmarks to `tests/itihas.bcyr`; rewrote `scripts/bench-history.sh` for the Cyrius harness with a committed `bench-history.csv`; slimmed `main.cyr` to a smoke test; wired `cyrius test` + benchmark steps into CI. Subsumes old v2.1.0 #5. |
| 2.3.2 | **distlib bundle + lint clean gate** | Medium | ✅ Released in 2.3.2. Added `src/lib.cyr` aggregator + `[lib].modules`; `cyrius distlib` → `dist/itihas.cyr` (consumer-ready); CI dist-freshness gate; release ships the bundle; added `sakshi` to stdlib so the bundle is self-resolving. `cyrius lint` already at zero non-cosmetic warnings under the existing hard gate. |
| 2.3.3 | **Lean-deps audit + de-vendor `./lib/`** | Low | ✅ Released in 2.3.3. Dropped 4 truly-dead deps (`io`, `args`, `toml`, `hashmap`); **kept `net` + `http`** (hoosh's self-contained raw-syscall HTTP client + `net`'s `sockaddr_in` — load-bearing for independent + library use) and `tagged` (`Result`, used transitively by `sakshi`/`net`). Stopped vendoring `./lib/` (now gitignored; cyrius regenerates it from the pinned snapshot), resolving the shadow warning. Unreachable-fns handled by `CYRIUS_DCE=1`. Verified across binary/test/bench/bundle. |
| 2.3.4 | **Language idiom modernization** | Large | ✅ Released in 2.3.4. Verified attributes against the compiler first — key finding: Cyrius uses bare `#attr`, not Rust `#[attr]` (a no-op comment); `#non_exhaustive`/`#inline` aren't Cyrius attributes. Applied enforced `#must_use` across the public API (data modules + serial + hoosh) and `defer` for socket cleanup in `hoosh_post`; corrected CLAUDE.md. Not applicable to itihas's offset-`store64` struct model: width-typed fields, `#derive(Serialize)`. `#regalloc` deferred (needs a measured win). |
| 2.3.5 | **Toolchain bump 6.0.50 → 6.2.11 + `json` → `bayan`** | Low | ✅ Released in 2.3.5. Pinned cyrius 6.2.11 and resynced `lib/`. The 6.1.25 data-format carve removed standalone `json` from the stdlib (now `lib/bayan.cyr`); swapped `[deps] stdlib` `json` → `bayan` (+ `result`/`io` prereqs). hoosh's `json_parse`/`json_get` unchanged via bayan compat aliases. Recorded toolchain-codegen creep on vec/str-heavy paths (no source change). Mirrors avatara 2.7.1/2.7.2. |

### Deferred / future

- **Static-data `.bss` footprint** — the 10 data modules embed ~300 KB of inline historical strings (descriptions). Restructuring to runtime `alloc()` per the compiler hint touches every data module and is its own large, risky cycle; deferred until there's demand.

### Modernization backlog (from avatara stdlib-usage review, 2.3.5)

Sibling avatara has completed two stdlib modernizations itihas has not. Both are large, behavior-preserving refactors — sequenced as their own work-loop cycles, never batched:

| Item | Effort | Details |
|------|--------|---------|
| **Checked allocation (`xalloc`) — CWE-690** | Medium | itihas has ~21 raw `alloc()` sites that write into the result unchecked; the stdlib `alloc()` returns `0` on OOM (near-NULL write / UB under exhaustion). Route heap allocation through a checked `xalloc(n)` that aborts with a diagnostic, per avatara 2.5.4 / its ADR-009 (Rust/Go abort-on-OOM policy). |
| **Native `#derive(accessors)` struct migration** | Large | The 10 data modules + serial address fields via ~242 manual `load64(p+OFFSET)` / `store64` calls against hand-maintained offset enums. avatara (2.5.3) migrated its profile to a named-field `#derive(accessors)` `struct` once the 6.x struct field cap was raised (32 → 256), making offset-collision bugs a compile error while keeping `prof_*` compat shims. The same applies here: convert each module's offset layout to a native struct, keep the public getters as shims. Touches all 11 modules — strictly one module per cycle, each behind the cleanliness + benchmark gates. |

## v2.4.0 — Tool Integration (blocked on bote)

| # | Item | Blocked on | Details |
|---|------|-----------|---------|
| 8 | **mcp module** | bote toolchain bring-up | `tool_definitions()`, `register_handlers()`, `register_all()`. 5 MCP tool handlers: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`. |
| 9 | **daimon module** | bote toolchain bring-up | `register_tools()`, `host_tool_descriptions()`, `invoke()`. Agent orchestrator integration via McpHostRegistry. |

## Blocked Dependencies

These Cyrius ports must happen in other repos before itihas can integrate:

| Dependency | Repo | Blocks | Status |
|-----------|------|--------|--------|
| **hoosh** | MacCracken/hoosh | v2.2.0 #7 | Ported |
| **bote** | MacCracken/bote | v2.4.0 #8, #9 | Ported — needs toolchain bring-up to latest Cyrius (same modernization itihas got in 2.3.0) before integration |
| **argonaut** (itihas integration) | MacCracken/argonaut | v2.1.0 #3 | argonaut itself is ported (424 tests), but itihas struct integration not done |

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation
- Display/formatting functions (Rust had `Display` impls on all types)
- `Ord`-equivalent sorting (Rust had chronological `Ord` on Era, Event, Campaign)

## Port Coverage

| Rust module | Cyrius | Functions | Status |
|------------|--------|-----------|--------|
| `era.rs` | `era.cyr` | 15 | Complete |
| `civilization.rs` | `civilization.cyr` | 12 | Complete |
| `event.rs` | `event.cyr` | 13 | Complete |
| `figure.rs` | `figure.cyr` | 8 | Complete |
| `causality.rs` | `causality.cyr` | 11 | Complete |
| `interaction.rs` | `interaction.cyr` | 12 | Complete |
| `calendar.rs` | `calendar.cyr` | 9 | Complete |
| `campaign.rs` | `campaign.cyr` | 18 | Complete |
| `site.rs` | `site.cyr` | 11 | Complete |
| `trade.rs` | `trade.cyr` | 12 | Complete |
| `error.rs` | `error.cyr` | 0 (enum only) | Complete |
| `logging.rs` | `logging.cyr` | 2 | Complete (sakshi) |
| `hoosh.rs` | — | — | Blocked on hoosh port |
| `mcp.rs` | — | — | Blocked on bote port |
| `lib.rs` | `main.cyr` | 3 (test harness) | Complete |
