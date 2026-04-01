# Architecture Overview

> **Itihas** — structured world history

## Module Map

```
itihas/
├── src/
│   ├── lib.rs            — public API, module re-exports
│   ├── error.rs          — ItihasError enum (non_exhaustive)
│   ├── era.rs            — historical periods, date ranges, era categories
│   │                       7 pre-built eras, eras_containing() lookup
│   ├── civilization.rs   — major civilizations, geographic extent, traits
│   │                       10 pre-built civilizations, by_region/active_at
│   ├── event.rs          — structured historical events, categories
│   │                       15 pre-built events
│   ├── calendar.rs       — calendar system metadata (type, epoch, months)
│   │                       8 pre-built calendar systems
│   ├── figure.rs         — historical figures, domain classification
│   │                       10 pre-built figures
│   └── logging.rs        — optional ITIHAS_LOG env-based tracing init
├── benches/
│   └── benchmarks.rs     — criterion benchmarks (6 benchmarks)
├── tests/
│   └── integration.rs    — cross-module integration tests
└── examples/
    └── basic.rs          — runnable usage example
```

## Data Flow

```
Year / region query
  │
  ├─→ era           — eras_containing(year) → matching Era structs
  ├─→ civilization   — active_at(year), by_region(region) → matching Civilization structs
  ├─→ event         — all_events() → Event structs with category, year, civilizations
  ├─→ calendar      — all_calendars() → CalendarSystem metadata
  └─→ figure        — all_figures() → Figure structs with era/civilization context
```

## Pre-built Data

| Module | Count | Examples |
|--------|-------|---------|
| Eras | 7 | Bronze Age, Classical Antiquity, Renaissance, Information Age |
| Civilizations | 10 | Mesopotamia, Egypt, Indus Valley, Greece, Rome, Maya |
| Events | 15 | Invention of writing, Fall of Rome, Printing press |
| Calendars | 8 | Gregorian, Julian, Islamic, Hebrew, Chinese, Hindu, Maya, Egyptian |
| Figures | 10 | Hammurabi, Aristotle, Ashoka, Gutenberg |

## Dependency Stack

```
itihas (this crate)
  │
  ├── serde      — serialization for all types
  ├── thiserror  — error derivation
  └── tracing    — structured logging
```

## Downstream Consumers

```
itihas
  ├─→ sankhya    — ancient mathematical systems (era-aware calendar arithmetic)
  ├─→ avatara    — historical simulation (civilization dynamics, era transitions)
  ├─→ kiran      — game engine (historical scenarios, timeline rendering)
  ├─→ joshua     — strategy game (historical civilizations, events)
  ├─→ jnana      — knowledge system (historical facts, timeline queries)
  ├─→ lipi       — linguistics (historical script/language context)
  └─→ vidya      — programming reference (history of computing)
```

## Design Principles

- **Data-driven**: Historical data as structured Rust types, not embedded strings
- **Queryable**: Every dataset supports lookup, filtering by year/region
- **Composable**: Each module is independent — consumers pull only what they need
- **Serializable**: All types implement Serialize + Deserialize for data exchange
- **Extensible**: `#[non_exhaustive]` on all enums — new variants without breaking changes
- **Zero-alloc statics**: `Cow<'static, str>` for pre-built data
