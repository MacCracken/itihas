# Benchmarks: Rust v1.5.0 vs Cyrius v2.0.0

Last Rust run: **2026-04-03T23:06:50Z** (commit `3d1bed9`, criterion 0.5)
Cyrius port: **2026-04-12** (cc3 v3.6.3, 141KB binary)

## Port Summary

| Metric | Rust v1.5.0 | Cyrius v2.0.0 |
|--------|-------------|---------------|
| Source lines | 8,846 | 1,591 |
| Binary | library crate | 141KB static ELF |
| Compiler | rustc 1.89 | cc3 3.6.3 (288KB) |
| Dependencies | serde, thiserror, tracing, criterion | sakshi (vendored stdlib) |
| Functions | ~60 public | 128 |
| Test assertions | 26 | 97 |
| Data entities | 297 | 297 |
| Serde roundtrip | yes | not yet (argonaut planned) |
| Description fields | full text | stripped (32KB str_data limit) |
| Logging | tracing crate | sakshi (vendored) |

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
| `interactions_for_rome` | 906.90 | 21 interactions, case-insensitive match |
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
- 141KB binary vs Rust library (no standalone binary to compare)

## Cyrius Function & Test Coverage

All 28 Rust benchmarked functions have Cyrius equivalents.
Tests exercise correctness; `.bcyr` timing harness needed for performance comparison.

| Rust benchmark | Cyrius function | Tested |
|----------------|----------------|--------|
| `all_eras` | `era_count()` | count=25 |
| `eras_containing_500bce` | `eras_containing()` | 2025, -500, -50000 (empty) |
| `all_civilizations` | `civ_count()` | count=53 |
| `civilizations_active_at_500bce` | `civs_active_at()` | -500, -100000 (empty), boundary checks |
| `civilizations_by_region` | `civs_by_region()` | "Near East" |
| `all_events` | `event_count()` | count=105 |
| `events_by_category_war` | `events_by_category()` | CAT_WAR, CAT_INVENTION |
| `events_at_year_476` | `events_at_year()` | 476, -753, verifies year value |
| `events_between_500bce_500ce` | `events_between()` | -800..476 |
| `all_calendars` | `calendar_count()` | count=8 |
| `calendar_by_name_gregorian` | `calendar_by_name()` | Gregorian, Hijri, Maya, Martian (not found) |
| `all_figures` | `figure_count()` | count=52 |
| `figures_by_domain_scientist` | `figures_by_domain()` | DOM_RULER, DOM_SCIENTIST, DOM_EXPLORER |
| `all_causalities` | `causality_count()` | count=13 |
| `causes_of_french_revolution` | `causes_of()` | French Revolution |
| `chain_writing_depth3` | `causal_chain()` | depth=3, verifies [0]=Hammurabi at depth 1 |
| `all_interactions` | `interaction_count()` | count=21 |
| `interactions_for_rome` | `interactions_for()` | Roman Empire |
| `influence_score_egypt_hittite` | `influence_score()` | Egypt-Hittite >0, symmetry, unrelated=0 |
| `all_sites` | `site_count()` | count=32 |
| `sites_by_region_near_east` | `sites_by_region()` | Near East |
| `sites_active_at_500bce` | `sites_active_at()` | -500 |
| `all_routes` | `route_count()` | count=15 |
| `routes_by_region_east_asia` | `routes_by_region()` | East Asia |
| `routes_by_commodity_silk` | `routes_by_commodity()` | silk |
| `all_campaigns` | `campaign_count()` | count=14 |
| `campaigns_by_commander_napoleon` | `campaigns_by_commander()` | Napoleon |
| `campaigns_between_500bce_500ce` | `campaigns_between()` | -500..-200 |

**Coverage: 28/28** (100%). All functions ported and tested.

Additional Cyrius functions not in Rust benchmarks:
- `events_by_significance()`, `sites_by_type()`, `sites_by_civilization()`,
  `routes_by_type()`, `routes_by_civilization()`, `routes_active_at()`,
  `campaigns_by_region()`, `campaigns_by_outcome()`, `campaigns_by_civilization()`,
  `campaigns_active_at()`, `interactions_by_type()`, `interaction_neighbors()`,
  `region_proximity()`, `itihas_log_init()`

## Port gaps recorded at the time — and what happened to them

This table is part of the 2026-04-12 snapshot. It is kept for the record; every
row's current status is in the right-hand column.

| Rust feature | Reason recorded then | Status now |
|-------------|--------------------|------------|
| Serde roundtrip (JSON) | No JSON serialization yet | **Shipped** — `src/serial.cyr`, `*_to_json` for all 11 types |
| Case-insensitive lookups | Cyrius has no `to_lowercase` | **Shipped** — `str_eq_lower` in `src/util.cyr` (allocation-free; no stdlib change needed) |
| Description fields | 32KB `str_data` compiler limit | **Shipped** — every record carries its description |
| Display impls | No `Display` trait in Cyrius | **Open** — demand-gated; see the roadmap |
| `hoosh` module | Needs hoosh Cyrius port | **Shipped in 2.2.0** — `src/hoosh.cyr`, 27 functions |
| `mcp` module | Needs bote Cyrius port | **Unblocked** — bote 3.3.7 ships the surface; see the roadmap and `docs/development/mcp-port-reference.md` |
| `daimon` module | Needs bote Cyrius port | **Unblocked** — same |
| `.bcyr` benchmarks | Need timing harness | **Shipped** — 28 benchmarks in `tests/itihas.bcyr` |

---

Generated during Cyrius port, 2026-04-12. Updated 2026-04-12.
Rust data captured from the v1.5.0 criterion suite before the port. That tree
(`rust-old/`) was removed once the port completed and lives in git history.
