# Architecture Overview

> **Itihas** — structured world history

## Module Map

```
itihas/
├── src/
│   ├── main.cyr          — entry point, test harness, module includes
│   ├── error.cyr         — ItihasError enum (integer error codes)
│   ├── era.cyr           — historical periods, date ranges, era categories
│   │                       25 eras (8 global + 17 regional), temporal/scope lookups
│   ├── civilization.cyr  — major civilizations, geographic extent, traits
│   │                       53 civilizations, by_region/active_at/by_name
│   ├── event.cyr         — structured historical events, categories, significance
│   │                       105 events, category/significance filters
│   ├── causality.cyr     — causal links between events, strength ratings
│   │                       chain traversal: causes_of/effects_of
│   ├── interaction.cyr   — civilization interaction graph, influence scoring
│   │                       trade/war/diplomacy
│   ├── calendar.cyr      — calendar system metadata (type, epoch, months)
│   │                       8 pre-built calendar systems
│   ├── figure.cyr        — historical figures, domain classification
│   │                       52 figures across 8 domains
│   ├── campaign.cyr      — military campaigns, battles, commanders
│   │                       14 campaigns with belligerents and outcomes
│   ├── site.cyr          — archaeological sites, location, discovery metadata
│   │                       32 sites with period and type classification
│   └── trade.cyr         — historical trade routes, endpoints, commodities
│                           15 trade routes with civilization context
├── tests/
│   ├── test.sh           — basic build + run test
│   └── test_itihas.sh    — full build + test suite runner
└── rust-old/             — preserved Rust v1.5.0 source (reference)
```

## Data Flow

```
Year / region query
  │
  ├─→ era           — eras_containing(year), eras_by_scope(), eras_by_region()
  ├─→ civilization   — civs_active_at(year), civs_by_region(region), civ_by_name()
  ├─→ event         — events_by_category(), event_by_name()
  ├─→ causality     — causes_of(event), effects_of(event)
  ├─→ interaction   — interactions_for(civ), interactions_between(a, b), influence_score()
  ├─→ calendar      — all_calendars(), calendar_by_name()
  ├─→ figure        — figures_by_domain(), figure_by_name()
  ├─→ campaign      — all_campaigns(), campaign_by_name()
  ├─→ site          — all_sites(), site_by_name()
  └─→ trade         — all_routes(), route_by_name(), routes_by_commodity()
```

## Pre-built Data

| Module | Count | Examples |
|--------|-------|---------|
| Eras | 25 | Bronze Age, Classical Antiquity, Tang Dynasty, Vedic Period |
| Civilizations | 53 | Mesopotamia, Mali Empire, Khmer Empire, Inca Empire |
| Events | 105 | Invention of Writing, Fall of Rome, Moon Landing |
| Causalities | 13 | Writing → Hammurabi, Printing Press → French Revolution |
| Interactions | 21 | Egypt ↔ Hittite (War/Diplomacy), Rome ↔ China (Trade) |
| Calendars | 8 | Gregorian, Julian, Islamic, Hebrew, Chinese, Hindu, Maya, Egyptian |
| Figures | 52 | Hammurabi, Confucius, Hypatia, Mansa Musa, Ada Lovelace |
| Campaigns | 14 | Alexander's Conquests, Punic Wars, Mongol Invasions |
| Sites | 32 | Pompeii, Machu Picchu, Angkor Wat, Gobekli Tepe |
| Trade Routes | 15 | Silk Road, Trans-Saharan, Amber Road |

## Heap Structure

Each module uses heap-allocated records via `alloc()` + `store64`/`load64`.
Field offsets defined as enum constants (e.g., `ERA_NAME=0; ERA_START=8;`).
Accessor functions provide read access (e.g., `fn era_name(p) { return load64(p+ERA_NAME); }`).

Data is initialized lazily on first access and cached in a global pointer.
Subsequent calls return the cached vec.

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

- **Data-driven**: Historical data as structured heap records with typed accessors
- **Queryable**: Every dataset supports lookup, filtering by year/region/category
- **Composable**: Each module is independent — consumers include only what they need
- **Zero external deps**: Vendored Cyrius stdlib only
- **Graph-ready**: Causality chains and interaction graphs for relational queries
- **Compact**: 117KB static ELF binary with all 297 entities
