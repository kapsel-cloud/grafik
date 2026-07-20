# Browser rendering

- **Status:** Active tracer contract
- **Kind:** Normative browser adapter owner
- **Authority:** Canonical for geometry, SVG rendering, accessibility, and reduced motion
- **Owns:** DOM measurement, port selection, trace consumption, visual projection, and motion policy
- **Does not own:** Seeded choices, route evolution, event ordering, or semantic outcome meaning

## Receipt-scene adapter

The browser receives one complete scene plan and renders its stable node tree as semantic HTML in
logical order. It maps grid roles to responsive CSS, visual roles to theme tokens, button-size roles
to accessible native controls, and diagram topology to DOM/SVG structure. Rust supplies no markup,
selectors, fonts, literal colors, breakpoints, or focus behavior.

The browser adapter must render these primitives: bounded text, facts, lists, tables, result blocks,
native actions, native menus, and the four diagram forms. A single concrete adapter serves every
scene in the standalone lab. Variation comes from scene plans, not separate rendering
implementations.

After fonts and layout settle, the browser reports rectangles and selected ports keyed by every
stable node ID required by the plan. Diagram connector and packet endpoints use those measured
anchors with a small visual overlap under node boundaries; the adapter must not maintain an
independent coordinate model. Resize causes remeasurement and resimulation of the same plan, never
scene regeneration.

The adapter maps load and click events to declared stable interaction IDs. It must update native
pressed, selected, focus, and expanded state before applying decorative events. It does not invent
randomness, reorder phases, retarget effects, or let decorative overlays receive pointer events.

## Animated-flow adapter

The standalone lab renders the curated five-node flow in generated logical order and gives every
base connector a visible arrow marker. It measures each node rectangle, selects source and target
ports on the facing boundaries, and returns those measurements to `simulate_scene`; it does not
derive traversal coordinates independently.

Native pressed controls select `SUCCEEDED`, `FAILED`, or `UNKNOWN`, select no disconnect or one
declared edge, and replay the same generated plan. Outcome selection updates the visible result text
before simulation. Disconnect selection preserves the base topology but renders a visible gap in the
selected connector, removes its completion arrow, and identifies the broken endpoints without
placing that decoration in reading order.

The trace overlay projects ordered `edge_traversed` events, one break-local spark, a semantic approval
role as an `aria-hidden` thumbs-up, and failure displacement only on the terminal's decorative
backing. `UNKNOWN` adds no terminal cue. When disconnected, no terminal cue is shown even though the
selected outcome text remains visible. Every decorative layer is `aria-hidden`, pointer-inert, and
must not obscure labels or controls.

The plan and trace inspectors expose the complete generated plan, measured state, selected outcome,
disconnect, and returned events. Replay uses the same plan, seed, geometry, and controls; resize
remeasures and resimulates rather than regenerating.

The standalone flow uses explicit `simulated` provenance and performs no live operation. Consumer
adapters may supply intentionally published `recorded` results through the same semantic interface,
but product records, provenance formats, transport, and classification remain outside Grafik.
Controls update readable outcome and source text before running the decorative profile.

## Reduced motion

When `prefers-reduced-motion: reduce` matches, the adapter consumes the complete trace immediately
and applies its final readable state. No timer, traversal, spark, pulsing, glitch, or background drift
runs. A static decorative approval mark may remain, but content never depends on it. Content, base
connectors, explicit breaks, panel borders, status text, and the no-JavaScript fallback remain
readable.

## Accessibility and resilience

- The scene tree defines logical reading order; visual grid placement must not change DOM order.
- Generated lists and tables use native elements, headers, and captions from bounded content.
- Generated actions and menus use native controls with stable accessible names and visible focus.
- Motion is decorative and hidden from assistive technology.
- Native buttons expose the three receiver outcomes and disconnect selection with `aria-pressed`.
- Keyboard users can select an outcome, select or clear one disconnect, and replay the flow.
- A live status announces the selected result profile or a readable failure.
- Meaning and reading order remain in HTML; color and motion carry no unique information.
- Text reflows without horizontal scrolling at 320 CSS px.
- With JavaScript disabled, both explanatory panels and the explicit simulated/non-production label
  remain present.
- Browser errors are shown as text rather than leaving a perpetual loading state.
