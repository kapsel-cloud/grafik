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
scene in the standalone grid lab. Variation comes from scene plans, not separate rendering
implementations.

After fonts and layout settle, the browser reports rectangles and selected ports keyed by every
stable node ID required by the plan. Diagram connector and packet endpoints use those measured
anchors with a small visual overlap under node boundaries; the adapter must not maintain an
independent coordinate model. Resize causes remeasurement and resimulation of the same plan, never
scene regeneration.

The adapter maps load and click events to declared stable interaction IDs. It must update native
pressed, selected, focus, and expanded state before applying decorative events. It does not invent
randomness, reorder phases, retarget effects, or let decorative overlays receive pointer events.

## Outcome-tracer geometry adapter

After fonts and layout are ready, the browser reads hero and receipt rectangles in the diagram
stage's local coordinate space. It selects the bottom-center hero port and top-center receipt port
and sends both rectangles, ports, and the outcome-text region to WASM. It reruns the tracer after a
debounced resize. Geometry measurement is browser work; Rust never queries or mutates the DOM.

## Simulated result adapter

The standalone preview constructs one explicit simulated `SUCCEEDED` result and passes it with
measured geometry into WASM. It loads no product record, performs no live operation, and defines no
stable cross-product format. Selecting another disposition keeps the same seed and changes only the
bounded simulated input.

Consumer adapters may supply intentionally published recorded results through the same semantic
interface, but product records, provenance formats, transport, and classification remain outside the
standalone Grafik repository. Controls update readable outcome and source text before running the
decorative profile.

## Browser outcome adapter

One absolutely positioned, `aria-hidden` SVG covers the diagram stage. The adapter converts each
spatial segment into one SVG line. Growth appends lines in event order; retraction removes them from
the leaf. The SVG uses `pointer-events: none` and never enters panel stacking or reading order.

The adapter consumes the public JSON event vocabulary directly. It maps successful weights and the
pulse to connector emphasis, applies failure displacement only to a decorative layer behind the
outcome text, and projects question marks into an `aria-hidden` layer behind the same region. Palette
roles become theme-aware CSS colors in the browser. It may schedule with browser time, but must not
reinterpret seeded choices or add ambient randomness.

## Reduced motion

When `prefers-reduced-motion: reduce` matches, the adapter consumes the complete trace immediately
and applies its final state. No timer, traversal, pulsing, glitch, or background drift runs. Content,
panel borders, status text, and the no-JavaScript fallback remain readable without the connector.

## Accessibility and resilience

- The scene tree defines logical reading order; visual grid placement must not change DOM order.
- Generated lists and tables use native elements, headers, and captions from bounded content.
- Generated actions and menus use native controls with stable accessible names and visible focus.
- Motion is decorative and hidden from assistive technology.
- Native buttons expose the three receiver outcomes and state selection with `aria-pressed`.
- A live status announces the selected result profile or a readable failure.
- Meaning and reading order remain in HTML; color and motion carry no unique information.
- Text reflows without horizontal scrolling at 320 CSS px.
- With JavaScript disabled, both explanatory panels and the explicit simulated/non-production label
  remain present.
- Browser errors are shown as text rather than leaving a perpetual loading state.
