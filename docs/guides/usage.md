# Usage Guide

> **Itihas** (Sanskrit: "thus it was") — structured world history as Cyrius types

## Philosophy

Itihas provides **queryable historical data**, not historical computation. It answers "what existed when and where" — calendar math, simulation, and AI queries belong in downstream projects (sankhya, avatara, hoosh).

All data is initialized lazily on first call and cached in a global pointer. Subsequent calls return the cached vec.

## Quick Start

```cyrius
# The single-file bundle. `cyrius deps` vendors the stdlib leaves named in
# dist/itihas.deps, and the bundle's own `# Requires` block includes them.
# Inside this repo, `include "src/lib.cyr"` is the equivalent.
include "itihas/dist/itihas.cyr"

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

14 military campaigns with 35 battles.

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

## Civilization names are free text

The `civilization` field on a figure, site, trade route or event — and the
belligerent fields on a campaign — are **free-text labels, not foreign keys into
`all_civilizations()`**. This is a deliberate decision, not an oversight.

Of the 256 such references in the dataset, **97 (38%) name something that has no
`civ_new` record**, and they fall into two groups:

- **Real polities outside the curated set.** The civilization table is 53
  hand-curated entries, not an exhaustive index of every state in history.
  `Maurya Empire`, `Roman Republic`, `Carthage`, `Gupta Empire` and
  `Kingdom of England` are all correct attributions for the records that carry
  them, and none has its own row.
- **Culture and period labels that are not polities at all.**
  `Pre-Pottery Neolithic`, `Neolithic Anatolia`, `Italian city-states` — these
  describe who made something without naming a state, which is often the only
  honest attribution available.

### What this means for you

- A name absent from `all_civilizations()` is still a **valid query**.
  `sites_by_civilization("Nabataean Kingdom")` returns its sites even though
  there is no Nabataean civilization record.
- Matching is on the string. The civilization filters (`sites_by_civilization`,
  `routes_by_civilization`, `campaigns_by_civilization`) match a whole
  `;`-delimited token, case-insensitive, so `"Roman Empire"` does not match the
  distinct polity `"Holy Roman Empire"`. Events carry their civilizations too
  (`evt_civs`), but there is no events-by-civilization filter.
- **Do not** write a test asserting that every civilization reference resolves
  to a record. It would fail on correct data. If you need the subset that does
  resolve, intersect against `all_civilizations()` explicitly at the call site.

If a reference is genuinely wrong — naming a civilization whose own dates
exclude the record pointing at it — that is a data bug and worth reporting. Two
such cases were corrected in 2.4.2 and eleven in 2.6.2, and `tests/itihas.tcyr`
now checks the rule for every reference that resolves to a record.

## Year Conventions

All years use **historical numbering with no year zero**. This is not
astronomical numbering, where -500 would mean 501 BCE.
- Negative = BCE: use `(0 - 500)` for 500 BCE, and `-1` is 1 BCE
- Positive = CE: `476` = 476 CE
- `2147483647` = ongoing (`ERA_ONGOING`, i32 max, equivalent to "present")
- Year `0` does not exist. Queries accept it as a plain integer between -1
  and 1, and no record is dated to it. The one stored 0 is `SITE_DISC_NONE`,
  the discovery year of a site that was never lost.

## Matching Rules

| Lookup | How it matches |
|--------|----------------|
| `*_by_name` | Whole name, case-insensitive |
| `*_by_region` | Substring, case-insensitive; an empty region matches nothing |
| `*_by_civilization`, `routes_by_commodity` | Whole `;`-delimited token, case-insensitive |
| `campaigns_by_commander` | A leading or trailing run of whole words of one commander's name, case-insensitive: `"Napoleon"` and `"Bonaparte"` both match `"Napoleon Bonaparte"`; `"Darius I"` does not match `"Darius III"`, and a middle word (`"E."` in `"Robert E. Lee"`) does not match |
| `*_between(sy, ey)`, `*_active_at`, `*_containing` | Both ends inclusive; an inverted range is empty |

## Collections Are Shared

`all_eras()`, `all_civilizations()` and the other `all_*()` functions return the
cached table itself, not a copy. Treat it as read-only: the year-sorted views
the range queries search are built from it once, so a record pushed onto it
later is visible to `era_count()` and `era_by_name()` but not to
`eras_containing()`.

## Error Handling

All `by_name()` lookups return a pointer (nonzero on success, 0 on not found).
Filter functions return vecs (empty if no matches, check with `vec_len()`).
