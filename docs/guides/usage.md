# Usage Guide

> **Itihas** (Sanskrit: "thus it was") — structured world history as Cyrius types

## Philosophy

Itihas provides **queryable historical data**, not historical computation. It answers "what existed when and where" — calendar math, simulation, and AI queries belong in downstream projects (sankhya, avatara, hoosh).

All data is initialized lazily on first call and cached in a global pointer. Subsequent calls return the cached vec.

## Quick Start

```cyrius
include "lib/string.cyr"
include "lib/alloc.cyr"
include "lib/vec.cyr"
include "lib/str.cyr"

include "src/era.cyr"
include "src/civilization.cyr"
include "src/event.cyr"

alloc_init();

# What eras span 500 BCE?
var eras = eras_containing((0-500));
# Which civilizations were active in 500 BCE?
var civs = civs_active_at((0-500));
```

## Modules

### Eras

25 eras: 8 global periodizations + 17 regional (Chinese dynasties, Indian periods, Mesoamerican periods).

```cyrius
# All eras
var all = all_eras();           # returns vec
var count = era_count();        # 25

# Filter by scope
var global = eras_by_scope(SCOPE_GLOBAL);
var regional = eras_by_scope(SCOPE_REGIONAL);

# Filter by region
var east_asia = eras_by_region("East Asia");

# Temporal lookup
var eras_500bce = eras_containing((0-500));

# Name lookup (returns pointer or 0)
var bronze = era_by_name("Bronze Age");

# Access fields
var name = era_name(bronze);    # Str
var start = era_start(bronze);  # i64 (year)
```

### Civilizations

53 civilizations spanning all inhabited continents.

```cyrius
var all = all_civilizations();
var count = civ_count();         # 53

# Region search
var near_east = civs_by_region("Near East");

# Active at a given year
var active = civs_active_at((0-500));

# Name lookup
var rome = civ_by_name("Roman Empire");
var name = civ_name(rome);
var region = civ_region(rome);
```

### Events

105 events with category and significance classification.

```cyrius
var all = all_events();
var count = event_count();       # 105

# Filter by category
var wars = events_by_category(CAT_WAR);
var inventions = events_by_category(CAT_INVENTION);

# Name lookup
var fr = event_by_name("French Revolution");
var year = evt_year(fr);         # 1789
```

### Causality

Causal chains linking events with strength ratings.

```cyrius
var count = causality_count();   # 13

# What caused the French Revolution?
var causes = causes_of("French Revolution");

# What did the invention of writing lead to?
var effects = effects_of("Invention of Writing");
```

### Interactions

Civilization interaction graph with influence scoring.

```cyrius
var count = interaction_count(); # 21

# All interactions involving Rome
var rome_int = interactions_for("Roman Empire");

# Between two civilizations
var btw = interactions_between("Ancient Egypt", "Hittite Empire");

# Influence score (weighted by interaction type)
var score = influence_score("Ancient Egypt", "Hittite Empire");
```

### Calendar Systems

8 calendar systems with metadata (computation belongs in sankhya).

```cyrius
var count = calendar_count();    # 8
var greg = calendar_by_name("Gregorian");
var months = cal_months(greg);   # 12
```

### Figures

52 figures across 8 domains.

```cyrius
var count = figure_count();      # 52
var rulers = figures_by_domain(DOM_RULER);
var aristotle = figure_by_name("Aristotle");
```

### Campaigns

14 military campaigns with 40+ battles.

```cyrius
var count = campaign_count();    # 14
var nap = campaign_by_name("Napoleonic Wars");
```

### Sites

32 archaeological sites.

```cyrius
var count = site_count();        # 32
var mp = site_by_name("Machu Picchu");
```

### Trade Routes

15 historical trade routes.

```cyrius
var count = route_count();       # 15
var silk = route_by_name("Silk Road");
var silk_routes = routes_by_commodity("silk");
```

## Year Conventions

All years use **astronomical year numbering**:
- Negative = BCE: use `(0 - 500)` for 500 BCE
- Positive = CE: `476` = 476 CE
- `2147483647` = ongoing (i64 max, equivalent to "present")

## Error Handling

All `by_name()` lookups return a pointer (nonzero on success, 0 on not found).
Filter functions return vecs (empty if no matches, check with `vec_len()`).
