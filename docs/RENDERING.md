# Browser rendering

- **Status:** Active tracer contract
- **Kind:** Normative browser adapter owner
- **Authority:** Canonical for geometry, SVG rendering, accessibility, and reduced motion
- **Owns:** DOM measurement, port selection, trace consumption, visual projection, and motion policy
- **Does not own:** Seeded choices, route evolution, event ordering, or semantic outcome meaning

## Geometry adapter

After fonts and layout are ready, the browser reads hero and receipt rectangles in the diagram
stage's local coordinate space. It selects the bottom-center hero port and top-center receipt port
and sends both rectangles, ports, and the outcome-text region to WASM. It reruns the tracer after a
debounced resize. Geometry measurement is browser work; Rust never queries or mutates the DOM.

## Recorded result adapter

The browser loads `fixtures/kapsel-recorded-success.json`, takes only its `result_source` and
`final_disposition`, and passes them with measured geometry into WASM. The fixture is a sanitized
record associated with the published Kapsel `v0.1.0` evaluator. It is not a receipt, trust decision,
live operation, or stable cross-product format.

The fixture identifies a recorded run, simulated presentation, and non-production status. Its only
provenance is the permanent GitHub evaluator guide for Kapsel `v0.1.0`. It contains no credentials,
grants, seeds, kubeconfig, journal, receipt bytes or digest, trust material, private paths, cluster
identities, operation identifiers, source revisions, commands, or timing.

The standalone preview starts with this recorded `SUCCEEDED` result. Selecting `FAILED` or `UNKNOWN`
creates an explicitly simulated result with the same seed and performs no infrastructure work.
Controls update readable outcome and provenance text before running the decorative profile.

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

- Motion is decorative and hidden from assistive technology.
- Native buttons expose the three receiver outcomes and state selection with `aria-pressed`.
- A live status announces the selected result profile or a readable failure.
- Meaning and reading order remain in HTML; color and motion carry no unique information.
- Text reflows without horizontal scrolling at 320 CSS px.
- With JavaScript disabled, both explanatory panels and the explicit simulated/non-production label
  remain present.
- Browser errors are shown as text rather than leaving a perpetual loading state.
