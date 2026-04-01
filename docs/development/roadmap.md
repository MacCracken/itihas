# Development Roadmap

> **Status**: 1.0 | **Current**: 1.0.0

## Completed

### 0.1.0 — Scaffold (2026-03-31)

- [x] Core type system: Era, Civilization, Event, CalendarSystem, Figure
- [x] EraCategory: Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary
- [x] EventCategory: War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse
- [x] CalendarType: Solar, Lunar, Lunisolar, Fixed
- [x] FigureDomain: Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor
- [x] 7 pre-built eras (Bronze Age through Information Age)
- [x] 10 pre-built civilizations (Mesopotamia through Ottoman)
- [x] 15 pre-built major world events
- [x] 8 pre-built calendar systems
- [x] 10 pre-built historical figures
- [x] Year-based lookups: eras_containing(), active_at(), by_region()
- [x] ItihasError with 5 variants
- [x] Optional structured logging
- [x] Cow<'static, str> for zero-alloc static data
- [x] Initial criterion benchmarks

### 0.2.0 — Expanded Coverage (2026-04-01)

- [x] Expand to 52 civilizations (sub-Saharan Africa, Southeast Asia, Pacific Islands, pre-Columbian Americas, Europe, Near East, South Asia, East Asia)
- [x] Expand to 105 events (regional events, scientific discoveries, cultural milestones)
- [x] Expand to 52 historical figures across all domains
- [x] Regional era systems: 9 Chinese dynasties, 5 Indian periods, 3 Mesoamerican periods (EraScope enum)
- [x] Event significance ranking: EventSignificance enum (Local, Regional, Continental, Global)
- [x] LazyLock caching for all data (sub-nanosecond all_*() calls)
- [x] no_std support (alloc/core imports)
- [x] Display impls on all public types
- [x] Ord on Era and Event for chronological sorting
- [x] by_name() lookups returning Result for era, civilization, event, figure
- [x] #[non_exhaustive] on all public structs
- [x] ItihasError: Clone, PartialEq, Eq derives; FigureNotFound variant

### 0.3.0 — Causality Chains & Event Graphs (2026-04-01)

- [x] `Causality` struct linking events (cause, effect, CausalStrength)
- [x] Event graph traversal: causes_of(), effects_of(), chain()
- [x] Civilization interaction graph (trade, war, cultural exchange, alliance, conquest, diplomacy)
- [x] Timeline slicing: events_between(start, end)
- [x] Geographic proximity and influence scoring
- [x] 13 pre-built causal links, 20 pre-built interactions
- [x] Usage guide (docs/guides/usage.md)

## Backlog

### 0.4.0 — AI Integration

- [ ] Hoosh client for LLM-powered historical queries
- [ ] MCP tools: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_timeline`, `itihas_figure`
- [ ] Natural language era/event lookup
- [ ] Daimon client for agent registration (when daimon is available)

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Archaeological site metadata
- Historical trade route data
- Military campaign timelines
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation

## v1.0 Criteria

- [x] 50+ civilizations with verified historical data
- [x] 100+ events with proper categorization
- [x] All modules have 80%+ test coverage (97.68% achieved)
- [ ] Criterion benchmarks with 3-point trend history
- [x] Full serde roundtrip tests for all public types
- [ ] sankhya + avatara consuming itihas for era-aware computation
- [x] Documentation: architecture overview, usage guide, API docs
