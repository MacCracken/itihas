# Development Roadmap

> **Status**: Cyrius port complete | **Current**: 2.0.0 | **Compiler**: cc3 3.6.3

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

## v2.1.0 — Data & Quality

| # | Item | Effort | Details |
|---|------|--------|---------|
| 1 | **Restore description fields** | Medium | cc3 str_data limit is 32KB; itihas needs ~50KB. Either load from external `.toml` data file at runtime, or wait for cc3 str_data expansion. |
| 2 | **Case-insensitive lookups** | Low | Needs `str_lower()` in Cyrius stdlib. Rust used `to_lowercase()` on all `by_name` queries. |
| 3 | **`.bcyr` benchmark harness** | Medium | Port 28 criterion benchmarks to Cyrius bench format. Enables direct Rust-vs-Cyrius timing comparison. |
| 4 | **Chronological sort** | Low | `events_between()` should return results sorted by year. Needs vec sort or insertion-order guarantee. |

## v2.2.0 — Serialization

| # | Item | Blocked on | Details |
|---|------|-----------|---------|
| 5 | **argonaut integration** | argonaut Cyrius port | JSON serialization for all types. `#derive(Serialize)` on structs. Enables serde roundtrip tests. |
| 6 | **TOML data loading** | lib/toml.cyr (in stdlib) | Load description fields from `data/*.toml` at runtime, bypassing str_data limit. |

## v2.3.0 — AI & Tool Integration

| # | Item | Blocked on | Details |
|---|------|-----------|---------|
| 7 | **hoosh module** | hoosh Cyrius port | `answer_from_data()`, `free_form()`, `civilizations_at()`, `events_in_range()`, `figure_lookup()`, `resolve_era_lookup()`. Natural language historical queries via LLM inference. |
| 8 | **mcp module** | bote Cyrius port | `tool_definitions()`, `register_handlers()`, `register_all()`. 5 MCP tool handlers: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`. |
| 9 | **daimon module** | bote Cyrius port | `register_tools()`, `host_tool_descriptions()`, `invoke()`. Agent orchestrator integration via McpHostRegistry. |

## Blocked Dependencies

These Cyrius ports must happen in other repos before itihas can integrate:

| Dependency | Repo | Blocks | Status |
|-----------|------|--------|--------|
| **hoosh** | MacCracken/hoosh | v2.3.0 #7 | Not started |
| **bote** | MacCracken/bote | v2.3.0 #8, #9 | Not started |
| **argonaut** (itihas integration) | MacCracken/argonaut | v2.2.0 #5 | argonaut itself is ported (424 tests), but itihas struct integration not done |

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
