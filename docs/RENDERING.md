# Browser rendering

- **Status:** Active tracer contract
- **Kind:** Normative browser adapter owner
- **Authority:** Canonical for geometry, SVG rendering, accessibility, and reduced motion
- **Owns:** DOM measurement, port selection, trace consumption, visual projection, and motion policy
- **Does not own:** Seeded choices, route evolution, event ordering, or semantic outcome meaning

## Geometry adapter

After fonts and layout are ready, the browser reads hero and receipt rectangles in the diagram
stage's local coordinate space. It selects the bottom-center hero port and top-center receipt port
and sends both rectangles and ports to WASM. It reruns the tracer after a debounced resize. Geometry measurement is browser work;
Rust never queries or mutates the DOM.

## Recorded result adapter

The browser loads `fixtures/kapsel-recorded-success.json`, takes only its `result_source` and
`final_disposition`, and passes them with measured geometry into WASM. The fixture is a sanitized
record associated with the published Kapsel `v0.1.0` evaluator. It is not a receipt, trust decision,
live operation, or stable cross-product format.

The fixture identifies a recorded run, simulated presentation, and non-production status. Its only
provenance is the permanent GitHub evaluator guide for Kapsel `v0.1.0`. It contains no credentials,
grants, seeds, kubeconfig, journal, receipt bytes or digest, trust material, private paths, cluster
identities, operation identifiers, source revisions, commands, or timing.

## SVG adapter

One absolutely positioned, `aria-hidden` SVG covers the diagram stage. The adapter converts each
spatial segment into one SVG line. Growth appends lines in event order; retraction removes them from
the leaf. The SVG uses `pointer-events: none` and never enters panel stacking or reading order.

The adapter consumes the public JSON event vocabulary directly. For successful progress, it maps
seeded weights 1 through 3 to subtle line emphasis without adding events or changing result meaning.
It may schedule with browser time, but must not reinterpret seeded choices.

## Reduced motion

When `prefers-reduced-motion: reduce` matches, the adapter consumes the complete trace immediately
and applies its final state. No timer, traversal, pulsing, glitch, or background drift runs. Content,
panel borders, status text, and the no-JavaScript fallback remain readable without the connector.

## Accessibility and resilience

- Motion is decorative and hidden from assistive technology.
- A live status announces whether the simulated connector completed or could not start.
- Meaning and reading order remain in HTML; color and motion carry no unique information.
- Text reflows without horizontal scrolling at 320 CSS px.
- With JavaScript disabled, both explanatory panels and the explicit simulated/non-production label
  remain present.
- Browser errors are shown as text rather than leaving a perpetual loading state.
