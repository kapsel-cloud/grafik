# Architecture

- **Status:** Active bootstrap contract
- **Kind:** Normative architecture owner
- **Authority:** Canonical for module ownership, seams, adapters, and dependency direction
- **Owns:** Product module shape and the Rust/WASM/browser split
- **Does not own:** Spatial budgets, event details, visual styling, or command syntax

## Composition

Grafik is one product package with one deep scene module and a shallow WASM adapter:

```text
bounded content + recipe + seed + budgets
                -> generate_scene -> complete renderer-neutral scene plan
                                         |
                         browser renders and measures stable node IDs
                                         |
scene plan + measured geometry + interaction
                -> simulate_scene -> complete renderer-neutral timed trace
                                         |
                              browser adapter renders trace
```

`grafik::generate_scene` is the receipt-view interface and primary test surface. Behind it live
bounded content validation, curated layout and diagram selection, stable tree construction, style
roles, button sizing, seeded pattern composition, hard-budget enforcement, and interaction scripts.
Callers receive one complete serializable plan and do not configure those internal choices
individually.

`grafik::simulate_scene` is the measured-effects interface. It accepts one generated plan,
browser-measured rectangles and ports keyed by stable node ID, and one declared interaction. Behind
it live geometry validation, route construction, phase timing, local-effect placement, and complete
trace generation. The current `grafik::simulate` result tracer remains while this vertical tracer is
proved; it must be absorbed rather than becoming a parallel long-term implementation.

The `wasm` Rust module is a shallow serialization adapter compiled only for `wasm32`. It exposes one
JSON call per Rust interface, translates into the same inputs used by native tests, and returns JSON.
It must not import browser DOM types or retain hidden mutable scene state.

The browser owns two concrete concerns without introducing a generic rendering interface:

- `web/main.js` measures DOM rectangles, selects edge ports, and calls generated WASM bindings.
- `web/svg-adapter.js` projects spatial events into SVG line segments and schedules visual traversal.

A second rendering implementation would make a renderer seam real. Until then, the lab's HTML/SVG
adapter stays concrete. `web/lab.html` is the independent development host for these public
interfaces; it is not a second product showcase. Its grid may render many scene plans, but all use
one concrete adapter.

## Dependency direction

- Domain types and simulation know nothing about WASM, JSON, SVG, CSS, DOM, clocks, product record
  bytes, or live transport. Callers provide bounded display content and adapters translate product
  results into Grafik's semantic vocabulary.
- The WASM adapter depends inward on the simulation interface.
- Browser adapters depend on the generated WASM interface, scene schema, and spatial event schema.
- Rendering preferences never flow into simulation; reduced motion changes trace consumption, not
  trace generation.
- Future simulated and real clients may translate into one semantic vocabulary, but neither may
  fork spatial logic or grant browser infrastructure authority.

## Repository boundary

Grafik's development loop is contained in this repository. Native tests, WASM generation, and the
standalone lab must not read from, write to, serve files from, or require a consumer checkout. The
lab uses simulated bounded inputs and generated files under Grafik's own `web/pkg/` directory.

A downstream product may consume an explicitly selected Grafik revision or attributable artifact,
but it owns the integration change in its own repository. Do not use cross-repository symlinks,
relative path dependencies, or scripts that copy into a sibling checkout: those make parallel work
silently alter another worktree. Product-data mapping, DOM/SVG/CSS, brand tokens, accessibility, and
responsive geometry remain consumer-adapter concerns; Grafik owns only the renderer-neutral tree and
layout roles it receives and generates.

## Package discipline

One package is sufficient because native tests and WASM are two consumers of one Rust interface,
not separate deployment units. Apply the deletion test before extracting files or packages. Do not
create `core`, `common`, `utils`, `engine`, or a plugin interface without independent consumers.

## Error direction

Invalid geometry is rejected before randomness advances. Rust errors are stable, readable values;
the WASM adapter converts them to JavaScript errors. Rendering errors remain in the browser.
