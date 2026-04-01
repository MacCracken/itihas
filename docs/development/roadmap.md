# Development Roadmap

> **Status**: Pre-1.0 | **Current**: 0.1.0

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

## Backlog

### 0.2.0 — Expanded Coverage

- [ ] Expand to 50+ civilizations (sub-Saharan Africa, Southeast Asia, Pacific Islands, pre-Columbian Americas)
- [ ] Expand to 100+ events (regional events, scientific discoveries, cultural milestones)
- [ ] Expand to 50+ historical figures across all domains
- [ ] Regional era systems (Chinese dynasties, Indian yugas, Mesoamerican periods)
- [ ] Event significance ranking (local, regional, continental, global)

### 0.3.0 — Causality Chains & Event Graphs

- [ ] `Causality` struct linking events (cause, effect, strength)
- [ ] Event graph traversal: causes_of(), effects_of(), chain()
- [ ] Civilization interaction graph (trade, war, cultural exchange)
- [ ] Timeline slicing: events_between(start, end)
- [ ] Geographic proximity and influence scoring

### 0.4.0 — AI Integration

- [ ] Daimon client for agent registration
- [ ] Hoosh client for LLM-powered historical queries
- [ ] MCP tools: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_timeline`, `itihas_figure`
- [ ] Natural language era/event lookup

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Archaeological site metadata
- Historical trade route data
- Military campaign timelines
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation

## v1.0 Criteria

- [ ] 50+ civilizations with verified historical data
- [ ] 100+ events with proper categorization
- [ ] All modules have 80%+ test coverage
- [ ] Criterion benchmarks with 3-point trend history
- [ ] Full serde roundtrip tests for all public types
- [ ] sankhya + avatara consuming itihas for era-aware computation
- [ ] Documentation: architecture overview, usage guide, API docs
- [ ] Published on crates.io
