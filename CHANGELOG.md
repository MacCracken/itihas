# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.1] - 2026-04-01

### Fixed

- **docs** — README now documents all 8 modules (was missing `causality` and `interaction`), all 5 features (was missing `hoosh` and `mcp`), and correct dependency version for v1
- **docs** — Roadmap reflects current state; completed items removed from backlog

## [1.0.0] - 2026-04-01

### Added

- **era** — Historical periods with date ranges and civilizational phases. `Era` struct, `EraCategory` enum (Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary), `EraScope` enum (Global, Regional). 25 pre-built eras: 8 global, 9 Chinese dynasties (Xia–Qing), 5 Indian periods (Vedic–Mughal), 3 Mesoamerican periods. `eras_containing()`, `by_scope()`, `by_region()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **civilization** — Major civilizations with geographic extent, peak period, key traits. 52 pre-built civilizations across all inhabited continents. `by_region()`, `active_at()`, `by_name()` lookups
- **event** — Structured historical events. `Event` struct, `EventCategory` enum (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse), `EventSignificance` enum (Local, Regional, Continental, Global). 105 pre-built world events. `events_between()`, `by_category()`, `at_year()`, `by_significance()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **causality** — Causal links between events with `CausalStrength` enum (Weak, Moderate, Strong, Direct). 13 pre-built causal links. `causes_of()`, `effects_of()`, `chain()` traversal
- **interaction** — Civilization interaction graph with `InteractionType` enum (Trade, War, CulturalExchange, Alliance, Conquest, Diplomacy). 22 pre-built interactions. `interactions_for()`, `by_type()`, `between()`, `neighbors()`, `influence_score()`, `region_proximity()` queries
- **calendar** — Calendar system metadata. `CalendarSystem` struct, `CalendarType` enum (Solar, Lunar, Lunisolar, Fixed). 8 pre-built calendar systems. `by_name()` lookup
- **figure** — Historical figures with era/civilization context. `Figure` struct, `FigureDomain` enum (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor). 52 pre-built figures. `by_domain()`, `by_name()` lookups
- **error** — `ItihasError` with variants: UnknownEra, UnknownCivilization, UnknownCalendar, InvalidYear, EventNotFound, FigureNotFound
- **hoosh** — Query types and data-driven answer resolution for AI integration (`hoosh` feature)
- **mcp** — MCP tool definitions stub for future bote integration (`mcp` feature)
- **logging** — Optional `ITIHAS_LOG` env-based tracing init (`logging` feature)
- `no_std` support via `alloc`/`core`; `std` feature adds `LazyLock` caching
- All public types: `Display` impls, `Cow<'static, str>` for zero-alloc statics, full serde roundtrip, `#[non_exhaustive]` on all enums and structs, `#[must_use]` on all pure functions

[Unreleased]: https://github.com/MacCracken/itihas/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/MacCracken/itihas/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/MacCracken/itihas/releases/tag/v1.0.0
