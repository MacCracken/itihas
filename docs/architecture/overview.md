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
│   │                       53 civilizations, by_region/active_at/by_name
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
│   ├── campaign.rs       — military campaigns, battles, commanders
│   │                       14 campaigns with belligerents and outcomes
│   ├── site.rs           — archaeological sites, location, discovery metadata
│   │                       32 sites with period and type classification
│   ├── trade.rs          — historical trade routes, endpoints, commodities
│   │                       15 trade routes with civilization context
│   ├── hoosh.rs          — query types and data-driven answers (feature-gated)
│   ├── mcp.rs            — MCP tool definitions and handlers (feature-gated)
│   └── logging.rs        — optional ITIHAS_LOG env-based tracing init
├── benches/
│   └── benchmarks.rs     — criterion benchmarks (28 benchmarks)
├── tests/
│   └── integration.rs    — cross-module integration tests (89 tests)
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
  ├─→ figure        — by_domain(), by_name()
  ├─→ campaign      — all_campaigns(), by_name(), by_era()
  ├─→ site          — all_sites(), by_type(), by_region()
  └─→ trade         — all_routes(), by_commodity(), by_civilization()
```

## Pre-built Data

| Module | Count | Examples |
|--------|-------|---------|
| Eras | 25 | Bronze Age, Classical Antiquity, Tang Dynasty, Vedic Period |
| Civilizations | 53 | Mesopotamia, Mali Empire, Khmer Empire, Inca Empire |
| Events | 105 | Invention of Writing, Fall of Rome, Moon Landing |
| Causalities | 13 | Writing → Hammurabi, Printing Press → French Revolution |
| Interactions | 22 | Egypt ↔ Hittite (War/Diplomacy), Rome ↔ China (Trade) |
| Calendars | 8 | Gregorian, Julian, Islamic, Hebrew, Chinese, Hindu, Maya, Egyptian |
| Figures | 52 | Hammurabi, Confucius, Hypatia, Mansa Musa, Ada Lovelace |
| Campaigns | 14 | Alexander's Conquests, Punic Wars, Mongol Invasions |
| Sites | 32 | Pompeii, Machu Picchu, Angkor Wat, Troy |
| Trade Routes | 15 | Silk Road, Trans-Saharan, Amber Road |

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
