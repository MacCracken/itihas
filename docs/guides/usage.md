# Usage Guide

> **Itihas** (Sanskrit: "thus it was") — structured world history as Rust types

## Philosophy

Itihas provides **queryable historical data**, not historical computation. It answers "what existed when and where" — calendar math, simulation, and AI queries belong in downstream crates (sankhya, avatara, hoosh).

All data is static and cached via `LazyLock`. The first call builds the dataset; subsequent calls return `&'static [T]` references at sub-nanosecond cost.

## Quick Start

```rust
use itihas::era;
use itihas::civilization;
use itihas::event;

// What was happening in 500 BCE?
let eras = era::eras_containing(-500);
let civs = civilization::active_at(-500);
let events = event::events_between(-600, -400);
```

## Modules

### Eras

25 eras: 8 global periodizations + 17 regional (Chinese dynasties, Indian periods, Mesoamerican periods).

```rust
use itihas::era::{self, EraScope};

// All eras
let all = era::all_eras(); // &'static [Era]

// Filter by scope
let global = era::by_scope(&EraScope::Global);   // Bronze Age, Classical Antiquity, etc.
let regional = era::by_scope(&EraScope::Regional); // Tang Dynasty, Vedic Period, etc.

// Filter by region
let east_asia = era::by_region("East Asia"); // Chinese dynasties

// Temporal lookup
let eras_500bce = era::eras_containing(-500);

// Name lookup (case-insensitive, returns Result)
let bronze = era::by_name("bronze age")?;

// Chronological sorting
let mut sorted = era::all_eras().to_vec();
sorted.sort(); // Ord impl sorts by start_year
```

### Civilizations

52 civilizations spanning all inhabited continents.

```rust
use itihas::civilization;

// All civilizations
let all = civilization::all_civilizations(); // &'static [Civilization]

// Region search (case-insensitive substring match)
let near_east = civilization::by_region("Near East");

// Active at a given year
let active_500bce = civilization::active_at(-500);

// Name lookup
let rome = civilization::by_name("Roman Empire")?;

// Access fields
println!("{} — script: {}, peak era: {}", rome.name, rome.script, rome.peak_era);
for trait_name in &rome.traits {
    println!("  - {trait_name}");
}
```

### Events

105 events with category, significance, and timeline slicing.

```rust
use itihas::event::{self, EventCategory, EventSignificance};

// Filter by category
let wars = event::by_category(&EventCategory::War);
let inventions = event::by_category(&EventCategory::Invention);

// Filter by significance
let global = event::by_significance(&EventSignificance::Global);

// Exact year
let events_476 = event::at_year(476);

// Timeline slice (inclusive, sorted chronologically)
let classical = event::events_between(-800, 476);

// Name lookup
let marathon = event::by_name("Battle of Marathon")?;
```

### Causality

Causal chains linking events with strength ratings.

```rust
use itihas::causality;

// What caused the French Revolution?
let causes = causality::causes_of("French Revolution");
for c in &causes {
    println!("{} → French Revolution ({})", c.cause, c.strength);
}

// What did the invention of writing lead to?
let effects = causality::effects_of("Invention of Writing");

// Follow causal chains forward up to 3 steps
let chain = causality::chain("Invention of Writing", 3);
for (event_name, depth) in &chain {
    println!("  depth {depth}: {event_name}");
}
```

### Interactions

Civilization interaction graph with influence scoring.

```rust
use itihas::interaction::{self, InteractionType};

// All interactions involving Rome
let rome = interaction::interactions_for("Roman Empire");

// Filter by type
let wars = interaction::by_type(&InteractionType::War);
let trade = interaction::by_type(&InteractionType::Trade);

// Between two specific civilizations (order-independent)
let egypt_hittite = interaction::between("Ancient Egypt", "Hittite Empire");

// Who interacted with Rome?
let neighbors = interaction::neighbors("Roman Empire");

// Influence score (weighted by interaction type)
let score = interaction::influence_score("Ancient Egypt", "Hittite Empire");

// Geographic proximity (0-100 based on region overlap)
let proximity = interaction::region_proximity("Roman Empire", "Ancient Greece");
```

### Calendar Systems

8 calendar systems with metadata (computation belongs in sankhya).

```rust
use itihas::calendar;

let all = calendar::all_calendars();
let gregorian = calendar::by_name("gregorian")?;
println!("{} — {} months, epoch year {}", gregorian.name, gregorian.months, gregorian.epoch_year);
```

### Historical Figures

52 figures across 8 domains.

```rust
use itihas::figure::{self, FigureDomain};

let scientists = figure::by_domain(&FigureDomain::Scientist);
let aristotle = figure::by_name("Aristotle")?;
println!("{}", aristotle); // "Aristotle (-384 – -322)"
```

### Error Handling

All `by_name()` lookups return `Result<T, ItihasError>`. Filter functions return `Vec<T>` (empty if no matches).

```rust
use itihas::ItihasError;

match itihas::era::by_name("Space Age") {
    Ok(era) => println!("Found: {era}"),
    Err(ItihasError::UnknownEra(name)) => println!("No era named '{name}'"),
    Err(e) => println!("Error: {e}"),
}
```

## Year Conventions

All years use **astronomical year numbering**:
- Negative = BCE (e.g., `-500` = 500 BCE)
- Positive = CE (e.g., `476` = 476 CE)
- Year 0 exists (= 1 BCE in historical convention)
- `i32::MAX` = ongoing (displayed as "present")

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | Yes | Enables `LazyLock` caching (sub-ns `all_*()` calls) |
| `logging` | No | Enables `ITIHAS_LOG` env-based tracing initialization |
| `full` | No | Enables `std` + `logging` |

Without `std`, `all_*()` functions rebuild data each call (still fast, but allocates).

## Display Formatting

All public types implement `Display`:

```rust
let era = itihas::era::by_name("Information Age").unwrap();
println!("{era}"); // "Information Age (1970 – present)"

let event = itihas::event::by_name("French Revolution").unwrap();
println!("{event}"); // "French Revolution (1789)"

let fig = itihas::figure::by_name("Aristotle").unwrap();
println!("{fig}"); // "Aristotle (-384 – -322)"
```

## Serialization

All types derive `Serialize` and `Deserialize` (serde). Roundtrip-tested.

```rust
let era = itihas::era::by_name("Bronze Age").unwrap();
let json = serde_json::to_string(&era).unwrap();
let back: itihas::era::Era = serde_json::from_str(&json).unwrap();
assert_eq!(era, back);
```
