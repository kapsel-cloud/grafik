# Prove seeded receipt scenes in Rust

- **Status:** Active
- **Direct owners:** [`../docs/SCOPE.md`](../docs/SCOPE.md),
  [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

Grafik generates complete renderer-neutral receipt and diagram scene plans in Rust from bounded
semantic data, explicit seeds, and hard budgets. A small WASM interface lets a browser render,
measure, and replay those plans. The standalone Grafik lab provides the development and review loop;
kapsel.cloud is not a build input and is not modified by this work.

## Acceptance

- One small Rust interface generates a serializable scene containing stable node identifiers,
  receipt structure, layout roles, diagram topology, visual roles, effect targets, and a controlled
  interaction script.
- Seeded selection covers curated grid layouts, button sizes, lists, tables, diagram forms, pattern
  profiles, phase ordering, and local glitches without ambient randomness.
- A second small Rust interface accepts browser-measured rectangles and ports for one generated scene
  and returns a complete, renderer-neutral timed trace.
- Hard budgets cap nodes, edges, phases, live effects, displacement, duration, and density before
  generation advances randomness.
- The browser owns DOM/SVG/CSS, actual geometry, accessibility, responsive reflow, and reduced-motion
  consumption. Rust emits no selectors, markup, literal colors, or DOM commands.
- The standalone lab presents a deterministic grid of candidates with seed navigation, replay,
  scene/trace inspection, and clickable scripted interactions.
- Same package version plus byte-equivalent request and measured geometry yields byte-equivalent scene
  and trace JSON.
- Grafik's default gate requires no sibling repository, live service, credentials, or network.

## Work order

1. Correct the canonical scene, pattern, simulation, rendering, and proof contracts.
2. Deepen the Rust implementation behind scene generation and measured-scene simulation interfaces.
3. Extend the standalone lab into the clickable grid review adapter.
4. Add native determinism/budget tests and dependency-free adapter tests.
5. Run `cargo make check` and record residual browser and visual risk.

## Evidence

- `cargo make check` (2026-07-19): passed with native scene determinism/budget tests, WASM build,
  attributable artifact validation, and dependency-free adapter tests.
- `node /tmp/grafik-wasm-smoke.mjs` (2026-07-19): generated a scene and measured load trace through
  the built WASM interface.
- `http://127.0.0.1:4174/lab.html?seed=424242&recipe=balanced`: standalone six-scene grid is served
  from Grafik while kapsel.cloud continues on its separate local port.

## Residual risk

The renderer-neutral scene-plan model was selected explicitly; Rust-authored markup was rejected.
Real-browser accessibility-tree behavior, 320-pixel reflow, visual quality across the pinned seed
corpus, and long-session timer cleanup still require manual proof. The pre-existing concurrent
`web/main.js` modification remains unresolved and was not edited as part of this task.
