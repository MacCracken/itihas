# Architecture Overview

> **Itihas** — structured world history

## Module Map

```
itihas/
├── src/
│   ├── lib.rs            — public API, module re-exports
│   ├── error.rs          — ItihasError enum (non_exhaustive)
│   ├── era.rs            — historical periods, date ranges, era categories
│   │                       25 eras (8 global + 17 regional), temporal/scope lookups
│   ├── civilization.rs   — major civilizations, geographic extent, traits
│   │                       52 civilizations, by_region/active_at/by_name
│   ├── event.rs          — structured historical events, categories, significance
│   │                       105 events, timeline slicing, category/significance filters
│   ├── causality.rs      — causal links between events, strength ratings
│   │                       chain traversal: causes_of/effects_of/chain
│   ├── interaction.rs    — civilization interaction graph, influence scoring
│   │                       trade/war/diplomacy, region proximity
│   ├── calendar.rs       — calendar system metadata (type, epoch, months)
│   │                       8 pre-built calendar systems
│   ├── figure.rs         — historical figures, domain classification
│   │                       52 figures across 8 domains
│   └── logging.rs        — optional ITIHAS_LOG env-based tracing init
├── benches/
│   └── benchmarks.rs     — criterion benchmarks (19 benchmarks)
├── tests/
│   └── integration.rs    — cross-module integration tests (59 tests)
└── examples/
    └── basic.rs          — runnable usage example
```

## Data Flow

```
Year / region query
  │
  ├─→ era           — eras_containing(year), by_scope(), by_region()
  ├─→ civilization   — active_at(year), by_region(region), by_name()
  ├─→ event         — events_between(start, end), by_category(), by_significance()
  ├─→ causality     — causes_of(event), effects_of(event), chain(event, depth)
  ├─→ interaction   — interactions_for(civ), between(a, b), influence_score()
  ├─→ calendar      — all_calendars(), by_name()
  └─→ figure        — by_domain(), by_name()
```

## Pre-built Data

| Module | Count | Examples |
|--------|-------|---------|
| Eras | 25 | Bronze Age, Classical Antiquity, Tang Dynasty, Vedic Period |
| Civilizations | 52 | Mesopotamia, Mali Empire, Khmer Empire, Inca Empire |
| Events | 105 | Invention of Writing, Fall of Rome, Moon Landing |
| Causalities | 13 | Writing → Hammurabi, Printing Press → French Revolution |
| Interactions | 20 | Egypt ↔ Hittite (War/Diplomacy), Rome ↔ China (Trade) |
| Calendars | 8 | Gregorian, Julian, Islamic, Hebrew, Chinese, Hindu, Maya, Egyptian |
| Figures | 52 | Hammurabi, Confucius, Hypatia, Mansa Musa, Ada Lovelace |

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
- **Queryable**: Every dataset supports lookup, filtering by year/region/category
- **Composable**: Each module is independent — consumers pull only what they need
- **Serializable**: All types implement Serialize + Deserialize for data exchange
- **Extensible**: `#[non_exhaustive]` on all public enums and structs
- **Zero-alloc statics**: `Cow<'static, str>` + `LazyLock` for sub-nanosecond data access
- **Graph-ready**: Causality chains and interaction graphs for relational queries
- **`no_std` compatible**: All modules use `alloc`/`core` imports; `std` feature adds caching
