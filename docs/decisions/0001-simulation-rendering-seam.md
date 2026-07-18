# 0001: Keep simulation separate from browser rendering

- **Status:** Accepted
- **Date:** 2026-07-18
- **Current owners:** [`../ARCHITECTURE.md`](../ARCHITECTURE.md),
  [`../RENDERING.md`](../RENDERING.md)

## Context

Seeded evolution must be replayable outside a browser, while browser geometry, SVG, accessibility,
and motion preferences change with the presentation environment. Direct DOM manipulation from
WASM would mix both concerns and make deterministic proof depend on browser state.

## Decision

Rust/WASM owns simulation and emits renderer-neutral events. Browser adapters own DOM measurement,
rendering, themes, accessibility, event handling, and reduced motion. Rust does not import DOM
interfaces.

## Consequences

Native tests can prove the simulation interface with controlled inputs. The browser must translate
geometry and consume traces. Serialization is an explicit shallow adapter cost.

## Rejected

- Rust-owned DOM rendering: weakens determinism and portability.
- JavaScript-owned randomness: prevents one authoritative replay algorithm.
