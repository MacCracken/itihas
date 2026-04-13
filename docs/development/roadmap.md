# Development Roadmap

> **Status**: v2.2.0 released | **Current**: 2.2.0 | **Compiler**: cc3 4.0.0

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

## v2.3.0 — Tool Integration (blocked on bote)

| # | Item | Blocked on | Details |
|---|------|-----------|---------|
| 8 | **mcp module** | bote Cyrius port | `tool_definitions()`, `register_handlers()`, `register_all()`. 5 MCP tool handlers: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`. |
| 9 | **daimon module** | bote Cyrius port | `register_tools()`, `host_tool_descriptions()`, `invoke()`. Agent orchestrator integration via McpHostRegistry. |

## Blocked Dependencies

These Cyrius ports must happen in other repos before itihas can integrate:

| Dependency | Repo | Blocks | Status |
|-----------|------|--------|--------|
| **hoosh** | MacCracken/hoosh | v2.2.0 #7 | Ported |
| **bote** | MacCracken/bote | v2.3.0 #8, #9 | Not started |
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
