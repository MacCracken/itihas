# Itihas

> **Itihas** (Sanskrit: इतिहास — "thus it was", history, chronicle) — structured world history for AGNOS

Civilizations, eras, events, historical figures, campaigns, sites, trade routes, and calendar system metadata as queryable Rust types.

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
| `error` | `ItihasError` with variants for unknown entities and invalid lookups |

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support |
| `logging` | no | Structured logging via `ITIHAS_LOG` env var |
| `hoosh` | no | Query types and data-driven answer resolution |
| `hoosh-llm` | no | Natural language queries via hoosh LLM inference |
| `mcp` | no | MCP tool definitions and handlers via bote |
| `daimon` | no | Daimon agent orchestrator integration (registers tools on `McpHostRegistry`) |
| `full` | -- | Enables all features |

## Quick Start

```toml
[dependencies]
itihas = "1"
```

```rust
use itihas::era;
use itihas::civilization;

// What eras span 500 BCE?
let eras = era::eras_containing(-500);
for e in &eras {
    println!("{}: {} to {}", e.name, e.start_year, e.end_year);
}

// Which civilizations were active in 500 BCE?
let civs = civilization::active_at(-500);
for c in &civs {
    println!("{} ({})", c.name, c.region);
}
```

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
make check     # fmt + clippy + test + audit
make bench     # Run benchmarks with history tracking
make coverage  # Generate coverage report
make doc       # Build documentation
```

## License

GPL-3.0-only. See [LICENSE](LICENSE).
