# Development Roadmap

> **Status**: Cyrius port | **Current**: 2.0.0

Completed items are in [CHANGELOG.md](../../CHANGELOG.md).

## Completed in 2.0.0

- [x] Port all 10 data modules from Rust to Cyrius
- [x] 297 historical entities (25 eras, 53 civs, 105 events, 52 figures, 13 links, 21 interactions, 8 calendars, 14 campaigns, 32 sites, 15 routes)
- [x] 26-assertion test suite
- [x] 117KB static ELF binary
- [x] Rust source preserved in `rust-old/`

## v2.1.0 — Data Completeness

- [ ] Restore description fields (load from external data file or wait for cc3 str_data limit bump)
- [ ] Add case-insensitive name lookups (needs `str_lower` in Cyrius stdlib)
- [ ] Port remaining Rust integration tests (55 tests in `rust-old/tests/integration.rs`)
- [ ] Write `.bcyr` benchmark harness for Cyrius-native timing

## v2.2.0 — Deps Integration

- [ ] sakshi integration for structured logging
- [ ] argonaut integration for JSON serialization
- [ ] hoosh module port (LLM query resolution)
- [ ] mcp module port (bote tool handlers)

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation
- `events_between()` with chronological sort
- `region_proximity()` (needs string split)
