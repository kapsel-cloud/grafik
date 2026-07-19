# Changelog

All notable changes to Grafik will be documented here. The project does not yet make stable
compatibility promises.

## Unreleased

## 0.1.0-alpha.1 - 2026-07-19

First experimental source release. Interfaces and serialized schemas may change between alpha
versions.

### Added

- Contract-first Rust/WASM architecture with browser-owned rendering and measured geometry.
- Deterministic outcome tracer preserving `NOT_ATTEMPTED`, `SUCCEEDED`, `FAILED`, and `UNKNOWN`.
- Seeded renderer-neutral receipt scene plans, diagram topology, effect targets, and interaction
  traces.
- Bounded pulse, glitch, invert, fragment, edge-traversal, packet, and scanline patterns with
  controlled simulation time.
- Native replay, geometry, budget, serialization, and seed-corpus tests.
- Standalone Grafik development lab isolated from downstream product repositories.
