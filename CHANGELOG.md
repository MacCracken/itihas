# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.5.0] - 2026-04-03

### Added

- **mcp** — 5 MCP tool invoke handlers wired to bote `Dispatcher`: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`
- **mcp** — `register_handlers()` registers all handlers on an existing dispatcher; `register_all()` registers definitions + handlers in one call
- **mcp** — All tools annotated with `ToolAnnotations::read_only()` (MCP 2025-11-25)
- **mcp** — MCP-compliant response format: `content` array with `text` type, `isError` flag for errors
- **mcp** — Case-insensitive `EventCategory` and `FigureDomain` parsing from string params
- **mcp** — `itihas_era` tool schema now includes `region` parameter
- **mcp** — `itihas_event` tool schema now includes `category` parameter
- **mcp** — 24 new handler tests (was 4 definition tests, now 28 total)
- **site** — Archaeological sites module: `Site` struct, `SiteType` enum (Settlement, Temple, Burial, Fortress, Monument, Palace, Workshop, Cave, Port). 32 pre-built sites across 10 regions. `all_sites()`, `by_region()`, `by_civilization()`, `by_type()`, `active_at()`, `by_name()` lookups
- **trade** — Trade routes module: `TradeRoute` struct, `RouteType` enum (Land, Maritime, River, Mixed). 15 pre-built routes spanning 4500 years. `all_routes()`, `by_region()`, `by_commodity()`, `by_type()`, `by_civilization()`, `active_at()`, `by_name()` lookups
- **error** — `SiteNotFound` and `RouteNotFound` variants added to `ItihasError`
- 6 new criterion benchmarks for site and trade modules (25 total)
- **civilization** — Added Arab Caliphates (632–1517) to civilization data (53 total)
- **daimon** — Daimon agent orchestrator integration (`daimon` feature): `mcp::daimon::register_tools()` registers all itihas tools on a `McpHostRegistry`, `host_tool_descriptions()` converts tool definitions to daimon's `McpToolDescription` format, `invoke()` dispatches tool calls and returns typed `McpToolResult`
- 7 new daimon integration tests (registry registration, tool invocation, error propagation, serde roundtrip)
- **hoosh-llm** — Natural language historical queries via LLM inference (`hoosh-llm` feature): `hoosh::llm::answer()` sends NL questions to hoosh with 6 tool definitions, LLM selects the appropriate lookup, itihas resolves from data. `resolve_era_lookup()` for era-specific queries. Falls back to pure LLM generation for questions outside data coverage
- 12 new hoosh-llm tests (tool def validation, tool call parsing, era resolution, missing args handling)
- **campaign** — Military campaigns module: `Campaign` struct, `Battle` struct, `CampaignOutcome` enum (Victory, Defeat, Stalemate, Treaty, Inconclusive). 14 pre-built campaigns with 40+ battles spanning 2500 years. `all_campaigns()`, `by_region()`, `by_civilization()`, `by_commander()`, `by_outcome()`, `active_at()`, `campaigns_between()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **error** — `CampaignNotFound` variant added to `ItihasError`
- 3 new criterion benchmarks for campaign module (28 total)

### Fixed

- **era** — Age of Enlightenment start_year 1600→1648 (1600 was Scientific Revolution, not Enlightenment)
- **era** — Renaissance end_year 1600→1610 (closes gap with Enlightenment for early 17th-century events)
- **era** — Mesoamerican Classic category Classical→Medieval (250–900 CE is Medieval by EraCategory definition)
- **civilization** — Maya peak_era "Classical Antiquity"→"Mesoamerican Classic" (has its own defined era)
- **civilization** — Mali Empire script "N'ko script"→"Arabic script" (N'Ko invented 1949, centuries after Mali)
- **event** — Decline of Indus Valley year -1300→-1900 (major cities abandoned by ~1700 BCE)
- **event** — Assassination of Julius Caesar: category War→Revolution, significance Regional→Continental
- **event** — French Revolution era "Industrial Age"→"Age of Enlightenment" (quintessential Enlightenment event)
- **event** — Fall of Constantinople: added Byzantine Empire to civilizations_involved
- **event** — Gutenberg Printing Press: added Holy Roman Empire to civilizations_involved
- **event** — Rise of Islam: added Arab Caliphates to civilizations_involved (was empty)
- **event** — First Crusade: added Holy Roman Empire to civilizations_involved
- **event** — Punic Wars: civilizations_involved "Phoenicia"→"Carthage" (Carthage was independent by 264 BCE)
- **event** — Norman Conquest: civilizations_involved "Viking/Norse"→"Normandy" (Normans ≠ Vikings by 1066)
- **event** — Polynesian Settlement of NZ: civilizations_involved "Tonga Empire"→"Polynesia" (settlers from eastern Polynesia)
- **event** — Tokugawa Shogunate era "Age of Enlightenment"→"Renaissance" (1603 predates Enlightenment)
- **figure** — Julius Caesar civilization "Roman Empire"→"Roman Republic" (died 44 BCE; Empire starts 27 BCE)
- **figure** — Hannibal Barca civilization "Phoenicia"→"Carthage" (Carthaginian, not generically Phoenician)
- **figure** — Muhammad civilization "Rashidun Caliphate"→"Quraysh" (Caliphate formed after his death)
- **figure** — Guru Nanak civilization "Mughal Empire"→"Lodhi Sultanate" (born 1469; Mughals founded 1526)
- **figure** — Aryabhata description: corrected overclaim about pioneering zero (Brahmagupta formalized zero)
- **site** — Lalibela civilization "Kingdom of Aksum"→"Zagwe Dynasty" (Aksum fell centuries before construction)
- **site** — Mycenae civilization "Ancient Greece"→"Mycenaean Civilization" (predates Classical Greece)
- **trade** — Trans-Saharan Trade start_year -500→300 (organized camel caravan trade began ~3rd century CE)
- **interaction** — Mali Empire trade partner "Ancient Egypt"→"Arab Caliphates" (Ancient Egypt ended 30 BCE)
- **causality** — Fall of Western Rome→Rise of Islam: strength Moderate→Weak (Byzantine-Sassanid exhaustion was the more direct factor)
- **calendar** — Egyptian Civil calendar type Solar→Fixed (no leap correction; drifted against solar year)
- **campaign** — Alexander belligerents_a "Ancient Greece"→"Macedon" (Macedonian army, not a Greek coalition)
- **campaign** — Second Punic War belligerents_b "Roman Empire"→"Roman Republic" (Republic, not Empire in 218 BCE)
- **campaign** — Gallic Wars belligerents_a "Roman Empire"→"Roman Republic" (Republic, not Empire in 58 BCE)
- **campaign** — Russo-Japanese War outcome Treaty→Victory (decisive Japanese win despite treaty ending)
- **campaign** — Russo-Japanese War: added Aleksei Kuropatkin to commanders (was missing Russian side)

## [1.0.1] - 2026-04-01

### Fixed

- **docs** — README now documents all 8 modules (was missing `causality` and `interaction`), all 5 features (was missing `hoosh` and `mcp`), and correct dependency version for v1
- **docs** — Roadmap reflects current state; completed items removed from backlog

## [1.0.0] - 2026-04-01

### Added

- **era** — Historical periods with date ranges and civilizational phases. `Era` struct, `EraCategory` enum (Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary), `EraScope` enum (Global, Regional). 25 pre-built eras: 8 global, 9 Chinese dynasties (Xia–Qing), 5 Indian periods (Vedic–Mughal), 3 Mesoamerican periods. `eras_containing()`, `by_scope()`, `by_region()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **civilization** — Major civilizations with geographic extent, peak period, key traits. 52 pre-built civilizations across all inhabited continents. `by_region()`, `active_at()`, `by_name()` lookups
- **event** — Structured historical events. `Event` struct, `EventCategory` enum (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse), `EventSignificance` enum (Local, Regional, Continental, Global). 105 pre-built world events. `events_between()`, `by_category()`, `at_year()`, `by_significance()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **causality** — Causal links between events with `CausalStrength` enum (Weak, Moderate, Strong, Direct). 13 pre-built causal links. `causes_of()`, `effects_of()`, `chain()` traversal
- **interaction** — Civilization interaction graph with `InteractionType` enum (Trade, War, CulturalExchange, Alliance, Conquest, Diplomacy). 22 pre-built interactions. `interactions_for()`, `by_type()`, `between()`, `neighbors()`, `influence_score()`, `region_proximity()` queries
- **calendar** — Calendar system metadata. `CalendarSystem` struct, `CalendarType` enum (Solar, Lunar, Lunisolar, Fixed). 8 pre-built calendar systems. `by_name()` lookup
- **figure** — Historical figures with era/civilization context. `Figure` struct, `FigureDomain` enum (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor). 52 pre-built figures. `by_domain()`, `by_name()` lookups
- **error** — `ItihasError` with variants: UnknownEra, UnknownCivilization, UnknownCalendar, InvalidYear, EventNotFound, FigureNotFound
- **hoosh** — Query types and data-driven answer resolution for AI integration (`hoosh` feature)
- **mcp** — MCP tool definitions stub for future bote integration (`mcp` feature)
- **logging** — Optional `ITIHAS_LOG` env-based tracing init (`logging` feature)
- `no_std` support via `alloc`/`core`; `std` feature adds `LazyLock` caching
- All public types: `Display` impls, `Cow<'static, str>` for zero-alloc statics, full serde roundtrip, `#[non_exhaustive]` on all enums and structs, `#[must_use]` on all pure functions

[Unreleased]: https://github.com/MacCracken/itihas/compare/v1.5.0...HEAD
[1.5.0]: https://github.com/MacCracken/itihas/compare/v1.0.1...v1.5.0
[1.0.1]: https://github.com/MacCracken/itihas/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/MacCracken/itihas/releases/tag/v1.0.0
