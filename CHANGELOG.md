# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-31

### Added

- **era** — Historical periods with date ranges and civilizational phases. `Era` struct, `EraCategory` enum (Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary). 7 pre-built eras (Bronze Age through Information Age). `eras_containing(year)` lookup
- **civilization** — Major civilizations with geographic extent, peak period, key traits. `Civilization` struct with 10 pre-built civilizations (Mesopotamia, Egypt, Indus Valley, China, Greece, Rome, Maya, Persia, Mongol, Ottoman). `by_region()`, `active_at(year)` lookups
- **event** — Structured historical events. `Event` struct, `EventCategory` enum (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse). 15 pre-built major world events
- **calendar** — Calendar system metadata. `CalendarSystem` struct, `CalendarType` enum (Solar, Lunar, Lunisolar, Fixed). 8 pre-built calendar systems (Gregorian, Julian, Islamic, Hebrew, Chinese, Hindu, Maya, Egyptian)
- **figure** — Historical figures with era/civilization context. `Figure` struct, `FigureDomain` enum (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor). 10 pre-built figures
- **error** — `ItihasError` with variants: UnknownEra, UnknownCivilization, UnknownCalendar, InvalidYear, EventNotFound
- **logging** — Optional `ITIHAS_LOG` env-based tracing init (feature-gated)
- All types: `Cow<'static, str>` for zero-alloc statics, serde roundtrip tests, `#[non_exhaustive]` on enums, `#[must_use]` on pure functions
- Initial criterion benchmarks for era lookup, civilization lookup, event search
