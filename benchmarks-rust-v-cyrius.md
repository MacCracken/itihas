# Benchmarks: Rust v1.5.0 vs Cyrius v0.1.0

Last Rust run: **2026-04-03T23:06:50Z** (commit `3d1bed9`, criterion 0.5)
Cyrius port: **2026-04-12** (cc3 v3.6.3, 117KB binary)

## Port Summary

| Metric | Rust v1.5.0 | Cyrius v0.1.0 |
|--------|-------------|---------------|
| Source lines | 8,846 | 1,033 |
| Binary | library crate | 117KB static ELF |
| Compiler | rustc 1.89 | cc3 3.6.3 (288KB) |
| Dependencies | serde, thiserror, tracing, criterion | none (vendored stdlib) |
| Test assertions | 26 | 26 |
| Data entities | 297 | 297 |
| Serde roundtrip | yes | not yet (no JSON) |
| Description fields | full text | stripped (32KB str_data limit) |

## Rust Benchmark Results (last run)

Final run from `bench-history.csv`, commit `3d1bed9` (v1.5.0 release).
All times in nanoseconds. Rust uses `LazyLock` caching — `all_*` benchmarks
measure cached access (~0.7ns pointer deref). Filter/search benchmarks measure
linear scan over cached `&'static [T]`.

### Data access (cached)

| Benchmark | Rust (ns) |
|-----------|-----------|
| `all_eras` | 0.73 |
| `all_civilizations` | 0.69 |
| `all_events` | 0.70 |
| `all_calendars` | 0.80 |
| `all_figures` | 0.74 |
| `all_causalities` | 0.73 |
| `all_interactions` | 0.72 |
| `all_sites` | 0.81 |
| `all_routes` | 0.73 |
| `all_campaigns` | 0.71 |

### Lookups and filters

| Benchmark | Rust (ns) | Notes |
|-----------|-----------|-------|
| `eras_containing_500bce` | 50.16 | 25 eras, year range check |
| `civilizations_active_at_500bce` | 428.33 | 53 civs, year range check |
| `civilizations_by_region` | 1570.90 | 53 civs, case-insensitive substring |
| `events_by_category_war` | 470.30 | 105 events, enum match |
| `events_at_year_476` | 59.74 | 105 events, exact year |
| `events_between_500bce_500ce` | 778.80 | 105 events, range filter + sort |
| `calendar_by_name_gregorian` | 45.05 | 8 calendars, case-insensitive substring |
| `figures_by_domain_scientist` | 223.75 | 52 figures, enum match |
| `causes_of_french_revolution` | 241.57 | 13 links, case-insensitive string match |
| `chain_writing_depth3` | 1820.20 | BFS over 13 links, depth 3 |
| `interactions_for_rome` | 906.90 | 22 interactions, case-insensitive match |
| `influence_score_egypt_hittite` | 911.28 | double scan + weight sum |
| `sites_by_region_near_east` | 1085.90 | 32 sites, case-insensitive substring |
| `sites_active_at_500bce` | 165.81 | 32 sites, year range |
| `routes_by_region_east_asia` | 1074.10 | 15 routes, case-insensitive substring |
| `routes_by_commodity_silk` | 1567.40 | 15 routes, case-insensitive substring |
| `campaigns_by_commander_napoleon` | 1432.40 | 14 campaigns, case-insensitive substring |
| `campaigns_between_500bce_500ce` | 283.01 | 14 campaigns, date overlap |

### Performance characteristics

**Rust advantages:**
- `LazyLock` caching gives sub-nanosecond repeated access (pointer deref)
- Case-insensitive string matching via `to_lowercase()` is optimized by the stdlib
- `criterion` provides statistically rigorous timing with warmup and outlier rejection
- Zero allocation on cached paths — `&'static [T]` borrows

**Cyrius characteristics (expected, not yet benchmarked):**
- Init functions allocate all data on first call, cache in global pointer (similar pattern)
- `str_eq` is byte-level comparison — no case folding overhead but case-sensitive
- Linear scan over `vec_get` with pointer deref per element
- All string data lives in bump-allocated heap — no reference counting
- 117KB binary vs Rust library (no standalone binary to compare)

## Cyrius Benchmark Coverage

Ported test coverage mapped to Rust benchmarks:

| Rust benchmark | Cyrius test | Status |
|----------------|-------------|--------|
| `all_eras` (count) | `era_count() == 25` | Ported |
| `eras_containing` | `eras_containing(2025) > 0` | Ported |
| `all_civilizations` (count) | `civ_count() == 53` | Ported |
| `civilizations_active_at` | `civs_active_at(-500) > 0` | Ported |
| `civilizations_by_region` | -- | Not yet (needs Cyrius bench) |
| `all_events` (count) | `event_count() == 105` | Ported |
| `events_by_category` | `events_by_category(CAT_WAR) > 0` | Ported |
| `events_at_year` | -- | Not yet |
| `events_between` | -- | Not yet |
| `all_calendars` (count) | `calendar_count() == 8` | Ported |
| `calendar_by_name` | `calendar_by_name("Gregorian") != 0` | Ported |
| `all_figures` (count) | `figure_count() == 52` | Ported |
| `figures_by_domain` | `figures_by_domain(DOM_RULER) > 0` | Ported |
| `all_causalities` (count) | `causality_count() == 13` | Ported |
| `causes_of` | `causes_of("French Revolution") > 0` | Ported |
| `chain` | -- | Not yet (chain fn exists but not tested) |
| `all_interactions` (count) | `interaction_count() == 21` | Ported |
| `interactions_for` | `interactions_for("Roman Empire") > 0` | Ported |
| `influence_score` | `influence_score("Ancient Egypt","Hittite Empire") > 0` | Ported |
| `all_sites` (count) | `site_count() == 32` | Ported |
| `sites_by_region` | -- | Not yet |
| `sites_active_at` | -- | Not yet |
| `all_routes` (count) | `route_count() == 15` | Ported |
| `routes_by_region` | -- | Not yet |
| `routes_by_commodity` | `routes_by_commodity("silk") > 0` | Ported |
| `all_campaigns` (count) | `campaign_count() == 14` | Ported |
| `campaigns_by_commander` | -- | Not yet |
| `campaigns_between` | -- | Not yet |

**Ported: 20/28** (71%). Remaining 8 are filter/search benchmarks that
need `cyrius bench` (.bcyr harness) to produce comparable timing data.

## Not Yet Ported

| Rust feature | Reason | Plan |
|-------------|--------|------|
| Serde roundtrip (JSON) | No JSON serialization yet | Port when argonaut integration available |
| Case-insensitive lookups | Cyrius has no `to_lowercase` | Add `str_lower` to stdlib or accept case-sensitive |
| Description fields | 32KB str_data compiler limit | Load from external data file or wait for limit bump |
| Display impls | No `Display` trait in Cyrius | `_str()` functions exist for enums |
| `hoosh` module | LLM integration, needs hoosh dep | Post-port |
| `mcp` module | MCP tools, needs bote dep | Post-port |
| `logging` module | tracing integration | Use sakshi dep |
| Integration tests (55 Rust tests) | Cross-module validation | Expand test suite incrementally |
| Fuzz testing | No .fcyr harness yet | Write after core benchmarks |

---

Generated during Cyrius port, 2026-04-12. Rust data from `rust-old/bench-history.csv`.
