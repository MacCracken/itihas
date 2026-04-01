# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-04-01

### Added

- **era** — Historical periods with date ranges and civilizational phases. `Era` struct, `EraCategory` enum (Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary). 25 pre-built eras (8 global + 17 regional). `eras_containing(year)`, `by_scope()`, `by_region()`, `by_name()` lookups
- **era** — `EraScope` enum (Global, Regional) for distinguishing world-history vs regional periodizations
- **era** — 9 Chinese dynasties (Xia through Qing), 5 Indian periods (Vedic through Mughal), 3 Mesoamerican periods (Preclassic, Classic, Postclassic)
- **era** — Age of Enlightenment (1600-1789) filling the Renaissance-Industrial gap
- **era** — `Ord` impl for chronological sorting
- **civilization** — Major civilizations with geographic extent, peak period, key traits. 52 pre-built civilizations across all inhabited continents. `by_region()`, `active_at()`, `by_name()` lookups
- **event** — Structured historical events. `Event` struct, `EventCategory` enum (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse). 105 pre-built world events
- **event** — `EventSignificance` enum (Local, Regional, Continental, Global) for impact classification
- **event** — `events_between(start, end)` timeline slicing with chronological sorting
- **event** — `by_category()`, `at_year()`, `by_significance()`, `by_name()` lookups
- **event** — `Ord` impl for chronological sorting
- **causality** — Causal links between events with `CausalStrength` enum (Weak, Moderate, Strong, Direct). 13 pre-built causal links. `causes_of()`, `effects_of()`, `chain()` traversal
- **interaction** — Civilization interaction graph with `InteractionType` enum (Trade, War, CulturalExchange, Alliance, Conquest, Diplomacy). 20 pre-built interactions. `interactions_for()`, `by_type()`, `between()`, `neighbors()` queries
- **interaction** — `influence_score()` weighted scoring and `region_proximity()` geographic proximity analysis
- **calendar** — Calendar system metadata. `CalendarSystem` struct, `CalendarType` enum (Solar, Lunar, Lunisolar, Fixed). 8 pre-built calendar systems. `by_name()` lookup
- **figure** — Historical figures with era/civilization context. `Figure` struct, `FigureDomain` enum (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor). 52 pre-built figures across all domains. `by_domain()`, `by_name()` lookups
- **error** — `ItihasError` with variants: UnknownEra, UnknownCivilization, UnknownCalendar, InvalidYear, EventNotFound, FigureNotFound. Derives Clone, PartialEq, Eq
- **logging** — Optional `ITIHAS_LOG` env-based tracing init (feature-gated behind `logging`)
- All public types: `Display` impls, `Cow<'static, str>` for zero-alloc statics, full serde roundtrip coverage, `#[non_exhaustive]` on all enums and structs, `#[must_use]` on all pure functions
- `no_std` support via `alloc`/`core` imports; `std` feature adds `LazyLock` caching (sub-nanosecond `all_*()` calls)
- `by_name()` lookups on all data modules returning `Result<T, ItihasError>`
- 122 tests (63 unit + 59 integration), 19 criterion benchmarks, 97.68% line coverage
- Documentation: architecture overview, usage guide, API docs
