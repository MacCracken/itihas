# Development Roadmap

> **Status**: 1.0 | **Current**: 1.0.1

## Completed

### AI Integration (v1.0.0)

- [x] Hoosh query types and data-driven answer resolution (`hoosh` feature)
- [x] MCP tool definitions stub (`mcp` feature, pending bote integration)

### External Integration (v1.0.1)

- [x] sankhya consuming itihas — feature-gated optional dep for era-aware epoch correlation

## Backlog

### AI Integration

- [ ] MCP tool invoke handlers (blocked on bote framework)
- [ ] Natural language era/event lookup via Hoosh LLM inference
- [ ] Daimon client for agent registration (when daimon is available)

## Future (demand-gated)

- Historical map data (geographic boundaries per era)
- Archaeological site metadata
- Historical trade route data
- Military campaign timelines
- Cultural diffusion tracking
- Historical population estimates
- Historical climate data correlation

## Outstanding (external dependencies)

- [x] sankhya consuming itihas — added as feature-gated optional dep
- [ ] avatara consuming itihas — avatara needs to add `itihas` as optional dep for historical simulation context
