# Itihas

> **Itihas** (Sanskrit: इतिहास — "thus it was", history, chronicle) — structured world history for AGNOS

Civilizations, eras, events, historical figures, campaigns, sites, trade routes, and calendar system metadata as queryable Cyrius types.

Used by [sankhya](https://github.com/MacCracken/sankhya) (ancient math), [avatara](https://github.com/MacCracken/avatara) (simulation), [kiran](https://github.com/MacCracken/kiran) (game engine), [joshua](https://github.com/MacCracken/joshua) (strategy), [jnana](https://github.com/MacCracken/jnana) (knowledge), [lipi](https://github.com/MacCracken/lipi) (linguistics), and [vidya](https://github.com/MacCracken/vidya) (programming reference).

## Modules

| Module | Description |
|--------|-------------|
| `era` | Historical periods with date ranges, civilizational phases, era category lookup |
| `civilization` | Major civilizations with geographic extent, peak period, key traits |
| `event` | Structured historical events with category, era, and civilizations involved |
| `causality` | Causal links between events with strength classification and chain traversal |
| `interaction` | Civilization interaction graph: trade, war, alliance, influence scoring |
| `calendar` | Calendar system metadata: type, epoch, months, leap rules (not computation) |
| `figure` | Historical figures with era/civilization context and domain classification |
| `campaign` | Military campaigns with battles, commanders, belligerents, and outcomes |
| `site` | Archaeological sites with location, period, type, and discovery metadata |
| `trade` | Historical trade routes with endpoints, commodities, and civilization context |
| `error` | Error code enum for unknown entities and invalid lookups |

## Pre-built Data

| Module | Count |
|--------|-------|
| Eras | 25 (8 global + 17 regional) |
| Civilizations | 53 |
| Events | 105 |
| Causal links | 13 |
| Interactions | 21 |
| Calendars | 8 |
| Figures | 52 |
| Campaigns | 14 (40+ battles) |
| Sites | 32 |
| Trade routes | 15 |

## Quick Start

```sh
# Build
cat src/main.cyr | cc3 > build/itihas && chmod +x build/itihas

# Run tests
./build/itihas

# Or use cyrius tooling
cyrius build src/main.cyr -o build/itihas
```

Requires Cyrius >= 3.6.3 (`cyriusly install 3.6.3`).

## Architecture

```text
itihas (this) — structured world history data
  | provides historical context
sankhya — ancient mathematical systems (calendar math, era arithmetic)
  | computation layer
avatara — historical simulation (era transitions, civilization dynamics)
  | simulation engine
kiran — game engine (historical scenarios, timeline rendering)
  | visual layer
```

Also feeds:
- **joshua** — strategy game (historical civilizations, events)
- **jnana** — knowledge system (historical facts, timeline queries)
- **lipi** — linguistics (historical script/language context)
- **vidya** — programming reference (history of computing)

## Development

```bash
sh tests/test_itihas.sh    # Build + run test suite
```

## Port History

Ported from Rust v1.5.0 (8,846 lines) to Cyrius v2.0.0 (1,591 lines, 128 functions, 97 tests) on 2026-04-12.
141KB static ELF binary. Previous Rust source preserved in `rust-old/` for reference.
See [benchmarks-rust-v-cyrius.md](benchmarks-rust-v-cyrius.md) for comparison.

## License

GPL-3.0-only. See [LICENSE](LICENSE).
