# Development Roadmap

> **Status**: 1.0 | **Current**: 1.0.1

## Completed

### AI Integration (v1.0.0)

- [x] Hoosh query types and data-driven answer resolution (`hoosh` feature)
- [x] MCP tool definitions stub (`mcp` feature, pending bote integration)

### External Integration (v1.0.1)

- [x] sankhya consuming itihas — feature-gated optional dep for era-aware epoch correlation

### MCP Tool Handlers (v1.0.2)

- [x] MCP tool invoke handlers — 5 handlers wired to bote `Dispatcher` (era, civilization, event, figure, timeline)
- [x] `register_handlers()` and `register_all()` convenience functions
- [x] MCP response format (`content` array with `text` type, `isError` flag)
- [x] `ToolAnnotations::read_only()` on all tools
- [x] Case-insensitive category/domain parsing

### Daimon Integration (v1.0.2)

- [x] `daimon` feature gate — `mcp::daimon` submodule with `bote/host` dependency
- [x] `register_tools()` — registers all 5 itihas tools on `McpHostRegistry`
- [x] `host_tool_descriptions()` — converts `ToolDef` to `McpToolDescription` format
- [x] `invoke()` — dispatches tool calls by name, returns typed `McpToolResult`
- [x] 7 integration tests (registry, invocation, error propagation, serde roundtrip)

### Hoosh LLM Integration (v1.0.2)

- [x] `hoosh-llm` feature gate — `hoosh::llm` submodule with hoosh crate + tokio dependency
- [x] `answer()` — async NL→structured query via LLM tool calling, data resolution, LLM fallback
- [x] 6 tool definitions (events_in_range, civilizations_at, causes_of, interactions_between, figure_lookup, era_lookup)
- [x] `resolve_era_lookup()` — direct era resolution for era_lookup tool calls
- [x] `parse_tool_call()` — converts LLM tool calls to `HistoryQuery` variants
- [x] 12 integration tests

## Backlog

### Archaeological Sites & Trade Routes (v1.0.2)

- [x] `site` module — 32 archaeological sites with location, period, type, civilization, and discovery metadata
- [x] `trade` module — 15 historical trade routes with endpoints, commodities, route type, and civilization context
- [x] `SiteNotFound` and `RouteNotFound` error variants
- [x] 6 new benchmarks for site and trade modules

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Military campaign timelines
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation

## Outstanding (external dependencies)

- [x] sankhya consuming itihas — added as feature-gated optional dep
- [ ] avatara consuming itihas — avatara needs to add `itihas` as optional dep for historical simulation context
