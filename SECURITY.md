# Security Policy

## Scope

Itihas is a pure historical data library providing civilization, era, event, calendar, and figure metadata for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| String processing | Unicode edge cases | Standard Rust string handling; no raw pointer ops |
| Serde deserialization | Crafted JSON | Enum validation via serde derive |
| Year lookups | Integer overflow on extreme years | Bounded by i32 range; no arithmetic overflow |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal core deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No `unwrap()` or `panic!()` in library code — all errors via `Result`
- All public types are `Send + Sync` (compile-time verified)
- No network I/O in core library
- Minimal dependency surface (core depends only on serde, thiserror, tracing)
