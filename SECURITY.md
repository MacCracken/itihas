# Security Policy

## Scope

Itihas is a pure historical data library providing civilization, era, event, campaign, site, trade route, calendar, and figure metadata for Cyrius. The core library performs no I/O beyond stdout for test output.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| String processing | Buffer overflows | Cyrius Str type handles length; no raw pointer string ops in library code |
| Heap allocation | Allocation exhaustion | Fixed data set (~297 entities), bounded memory use |
| Year lookups | Integer overflow on extreme years | Bounded by i64 range; no arithmetic overflow in lookups |
| Dependencies | Supply chain | Vendored stdlib only; no external package deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (Cyrius) | Yes |
| 1.x (Rust, rust-old/) | Reference only, not maintained |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- No `unsafe` equivalent — all memory via `alloc`/`store64`/`load64`
- No network I/O in core library
- Minimal dependency surface (vendored Cyrius stdlib only)
- All data statically defined at compile time
