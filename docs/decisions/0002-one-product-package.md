# 0002: Start with one product package

- **Status:** Accepted
- **Date:** 2026-07-18
- **Current owner:** [`../ARCHITECTURE.md`](../ARCHITECTURE.md)

## Context

The bootstrap has one simulation implementation consumed by native tests and a WASM adapter. A
workspace of speculative `core`, browser, protocol, or plugin packages would expose more interfaces
without independent deployment or reuse needs.

## Decision

The repository root is one Rust package and workspace. Internal Rust modules remain private unless
the product interface requires them. A second package requires a real independent consumer or
dependency constraint and must pass the deletion test.

## Consequences

Simulation knowledge stays local and navigation stays shallow. A later extraction may require a
migration, which is cheaper than maintaining hypothetical seams now.

## Rejected

- A generic animation core: Grafik is an explanation system, not a framework.
- Separate protocol and WASM crates: no independent consumer currently earns either interface.
