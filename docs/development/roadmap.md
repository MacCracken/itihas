# Development Roadmap

> **Current**: 2.6.2 | **Compiler**: cyrius 6.7.18 | **Tests**: 442 + 65 MCP | **Coverage**: 87% (reference) | **Lint**: clean | **Doc gate**: clean

This file tracks **open work only**, pinned to the minor line that owns it.
Completed releases live in [CHANGELOG.md](../../CHANGELOG.md) and are not
duplicated here.

Each item is its own work-loop cycle (cleanliness + benchmark gates, per
CLAUDE.md) unless it is marked batchable. A Large item is cut into the patch
releases listed under it, one per cycle.

| Line | Theme | Gate to start |
|------|-------|---------------|
| [2.7.x](#27x--structural-arc) | Native structs for the nine remaining data modules | — (next) |
| [2.8.x](#28x--query-engine-and-footprint) | Shared query helpers, static-data footprint | 2.7.x complete |
| [2.9.x](#29x--data-and-presentation) | New records, `*_to_str` formatters | 2.7.x complete |
| [2.10.x](#210x--portability) | `hoosh_post` on Windows and agnos | demand |
| [2.x.x](#2xx--backlog-unpinned) | Unpinned backlog | demand |

---

## 2.7.x — Structural arc

**Native `#derive(accessors)` structs for the nine data modules still on
hand-maintained offset enums.** `era` moved in 2.5.1. The rest read and write
fields with `load64(p + OFFSET)` / `store64` against offset enums that nothing
checks: two fields sharing an offset would alias silently, where a struct makes
it a compile error. Strictly **one module per patch release**, each with layout
assertions that pin every field to its offset in both directions, and its own
interleaved A/B.

| Release | Module | Why this order |
|---------|--------|----------------|
| 2.7.0 | `civilization` | Hottest after `era`: `civs_active_at`, `civs_by_region` and the founding-year view |
| 2.7.1 | `event` | 105 rows, the year-sorted view and three range queries |
| 2.7.2 | `campaign` | Range query plus the commander/belligerent token matchers |
| 2.7.3 | `site` | Two range predicates (occupation, discovery) |
| 2.7.4 | `trade` | Commodity and civilization token matchers |
| 2.7.5 | `figure` | Domain filter only |
| 2.7.6 | `interaction` | Read by `influence_score`'s in-place scan |
| 2.7.7 | `causality` | Read by `causal_chain`'s frontier walk |
| 2.7.8 | `calendar` | 8 rows, no hot path |

**Rule from the `era` migration, binding on every row above.** Delegating
`x_field(p)` to the generated `X_field(p)` adds a call, and Cyrius does not
inline. Routing `era`'s hot queries through the wrappers cost +12.3% on
`eras_containing_500bce`, and calling the generated accessor directly put it
back to flat. Keep the `x_*` names as the public API, and have hot paths call
`X_field` directly.

## 2.8.x — Query engine and footprint

Starts after 2.7.x, so the shared helpers can be written once against the
generated accessors instead of raw offsets.

| Item | Effort | Detail |
|------|--------|--------|
| **Generic filter/lookup helpers** | Large | About 31 near-identical filter bodies across the ten data modules (by-name, by-region, by-year, by-token). One filter shape per patch, behind the benchmark gate. The 2.4.2 token-membership semantics (`str_list_has`, `str_list_has_name`) are the contract to preserve. |
| **Static-data footprint** | Large | The data modules embed ~300 KB of inline description strings in `.rodata`. Moving them to runtime-built storage touches every data module. Demand-gated within the line: start when a consumer reports binary size as a problem. |

## 2.9.x — Data and presentation

| Item | Effort | Detail |
|------|--------|--------|
| **Carthage and Khwarezmian civilization records** | Medium | Lets the Punic Wars interaction and the Mongol–Khwarezm references point at real records instead of free text. Moves `civ_count` 53 → 55 and the 338-entity total, which cascades into tests, `src/lib.cyr`, `src/main.cyr`, README and the architecture overview. Each record needs its provenance in `docs/sources/civilizations.md`. |
| **`*_to_str` display formatters** | Medium | The Rust original had `Display` on every type. One formatter per record type, writing into a caller-supplied `str_builder` (the `_into` pattern from `serial.cyr`), with the ongoing-era sentinel rendered as "present" the way `resolve_era_lookup` already does. |
| **`null` for "no discovery year" in JSON** | Low | `site_to_json` emits a never-lost site's `SITE_DISC_NONE` as `"discovered": 0`, a year that does not exist. `null` is the honest value, but it changes the JSON a consumer parses, so it ships with the formatters rather than in a patch. Found in the 2.6.2 sweep. |
| **Data questions left open in 2.6.2** | Low | Rows where the sweep found sources in conflict, so nothing was changed: Leptis Magna's start (-1000 in some references, the late 7th century BCE in others); Ranald MacDonald's civilization ("United States", though he was born in British North America); the Egyptian civil-calendar epoch (-2781, the Sothic reckoning, where the cited Dershowitz and Reingold use Nabonassar, -747); "Roman Empire" and "Assyrian Empire" records that start at their cities' founding rather than the empire's. Each needs a decision and a line in `docs/sources/`. |

Civilization-name fields stay **free text, not foreign keys**: 72 of 223
references name a polity or culture with no record of its own, and that is
correct data (see [usage.md](../guides/usage.md#civilization-names-are-free-text)).
A reference whose *dates* contradict the record it names is still a bug.

## 2.10.x — Portability

| Item | Effort | Detail |
|------|--------|--------|
| **`hoosh_post` on Windows and agnos** | Medium | `src/hoosh.cyr` opens its socket with raw `syscall(SYS_SOCKET, …)` / `syscall(SYS_CONNECT, …)` and moves bytes with `SYS_WRITE` / `SYS_READ`. Those numbers exist on Linux and macOS only, and from the 6.6.20 pin on `cyrius distlib` warns that both are undefined on x86_64-windows and x86_64-agnos. A Windows socket is a Winsock `SOCKET`, so the port routes connect, write, read and close through `lib/net.cyr`'s per-OS `sock_*` primitives rather than renaming constants. The HTTP client stays self-contained in itihas. |

## 2.x.x — Backlog (unpinned)

Pinned to a line when a consumer needs it.

| Item | Effort | Detail |
|------|--------|--------|
| **bayan's Unicode tables** | Upstream | Since cyrius 6.7.13, `lib/bayan.cyr` includes `lib/unicode/*` (case-fold, categories, normalization) and `lib/math.cyr` for every consumer. DCE removes unreachable functions, not data. Measured in 2.6.1 on the DCE build: `.rodata` 66,814 → 408,802 B, and the binary 251,800 → 535,392 B, though itihas calls only `bayan_json_parse` and `bayan_json_get_by_cstr`. The fix belongs in bayan, as a JSON-only profile or tables only the Unicode-aware functions reach. A hand-rolled JSON reader here would trade an AGNOS dependency for local code, against "own the stack". |
| **The `ITIHAS_ERR_*` codes** | Low | `src/error.cyr` defines the error enum, and nothing in itihas returns one: lookups return 0 and filters an empty vec. Either adopt the codes (an out-parameter on the lookups) or retire the enum with a deprecation cycle. A decision, not a bug; found in the 2.6.2 sweep. |
| **argonaut consolidation** | Medium | argonaut is ported upstream (424 tests) but not integrated. itihas has its own `src/serial.cyr`, so this is an optional consolidation, not a gap. |
| Historical map data (boundaries per era) | Large | — |
| Cultural diffusion tracking | Large | — |
| Historical population estimates | Medium | — |
| Historical climate correlation | Medium | — |
