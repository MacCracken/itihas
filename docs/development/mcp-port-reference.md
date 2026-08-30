# MCP / daimon port reference

The specification for the `mcp` module and its `daimon` integration, ported
from the Rust original in 2.5.0.

**Status: IMPLEMENTED in 2.5.0** as `src/mcp.cyr`. This file is kept as the
specification the implementation was written against, and as the record of what
the Rust original did.

It exists because `rust-old/` was removed once every other module had shipped.
`mcp.rs` was the only file with reference value left, so its surface, tool
schemas and handler mappings are captured here rather than kept as 714 lines of
un-buildable Rust. The original remains in git history if the full text is ever
needed:

```
git log --all -- rust-old/src/mcp.rs
git show <commit>:rust-old/src/mcp.rs
```

## How the shipped module differs from this spec

Three deliberate departures, each recorded because a reader comparing the two
will notice them:

- **Everything is `itihas_`-prefixed.** Cyrius has one flat function namespace,
  so a bare `tool_definitions()` in a library linked beside others is a
  collision waiting to happen. Building this module proved the point: it
  surfaced two real clashes in itihas's own code — `query_new` against bote's
  `libro`, and `RT_SIZE` against `lib/async_win.cyr` with a *different value* —
  both renamed in 2.5.0.
- **`ToolResult` is not a tagged union.** Handlers return a JSON cstr, matching
  bote's own handler convention; failures are `{"ok":false,"error":"..."}`.
- **`src/mcp.cyr` is not in the default bundle.** It is absent from
  `src/lib.cyr` and `[lib].modules`, so `dist/itihas.cyr` references no bote
  symbol. bote sits behind the optional `mcp` feature, because measuring showed
  that defaulting it on linked 2.1 MB of bote into a binary that never calls it.

## Public surface

Six functions across two modules.

| Module | Function | Purpose |
|--------|----------|---------|
| `mcp` | `tool_definitions()` | The five tool definitions below, all read-only |
| `mcp` | `register_handlers(dispatcher)` | Bind each tool name to its handler |
| `mcp` | `register_all(dispatcher)` | `tool_definitions()` + `register_handlers()` in one call |
| `mcp::daimon` | `register_tools(registry)` | Register the tools on an `McpHostRegistry` |
| `mcp::daimon` | `host_tool_descriptions()` | The same defs in daimon's description format |
| `mcp::daimon` | `invoke(name, arguments)` | Dispatch by tool name; `None` for an unknown name |

`ToolResult` was a two-variant enum — success carrying a JSON value, or an error
carrying a message. In Cyrius that maps to the existing packed-pointer
convention rather than a tagged union.

## Tool definitions

All five are **read-only** lookups over static data, and every parameter is
optional — a call with no arguments returns the full collection. Year fields are
signed, negative meaning BCE, matching the rest of itihas.

| Tool | Parameters | Backing queries |
|------|-----------|-----------------|
| `itihas_era` | `name`, `year`, `region` | `era_by_name`, `eras_containing`, `eras_by_region`, `all_eras` |
| `itihas_civilization` | `name`, `year`, `region` | `civ_by_name`, `civs_active_at`, `civs_by_region`, `all_civilizations` |
| `itihas_event` | `name`, `start_year`, `end_year`, `category` | `event_by_name`, `events_between`, `events_by_category`, `all_events` |
| `itihas_figure` | `name`, `domain` | `figure_by_name`, `figures_by_domain`, `all_figures` |
| `itihas_timeline` | `start_year`, `end_year` | `events_between`, `eras_containing`, `civs_active_at` |

Descriptions as originally written:

- `itihas_era` — "Look up historical eras by name, year, scope, or region"
- `itihas_civilization` — "Look up civilizations by name, region, or active year"
- `itihas_event` — "Look up historical events by name, year range, or category"
- `itihas_figure` — "Look up historical figures by name or domain"
- `itihas_timeline` — "Get a timeline of events, eras, and civilizations for a year range"

## Handler dispatch

Each handler selects a query by which parameters are present, most specific
first (`name` before `year` before `region`), falling back to the full
collection. Results serialize through the existing `*_to_json` functions in
`src/serial.cyr`, so no new serialization is required.

`itihas_timeline` is the only composite: it unions events, eras and
civilizations for the requested range into one object.

## Notes for the Cyrius port

- `src/hoosh.cyr` already implements the same *shape* for a different transport
  — `hoosh_tool_defs_json()` emits six tool definitions and `parse_tool_call()`
  dispatches by name. That is the closest working model; the MCP tool names and
  parameter sets differ, but the dispatch structure carries over directly.
- The Rust version used `#[must_use]` on the pure functions. In Cyrius that is
  the bare `#must_use` attribute — see CLAUDE.md.
- `tracing::info!` / `tracing::debug!` calls in `register_tools` map to the
  `sakshi` wrappers in `src/logging.cyr`.
