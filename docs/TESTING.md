# Testing

- **Status:** Active proof strategy
- **Kind:** Normative testing owner
- **Authority:** Canonical for evidence, determinism, and proof layering
- **Owns:** What tests must prove and where proof belongs
- **Does not own:** Command syntax, implementation seams, or review result formatting

## Proof layers

Run the narrowest proof first:

1. Native interface tests prove geometry validation, deterministic replay, event ordering, route
   exclusion, and bounded timing through `grafik::simulate`.
2. WASM build proof verifies the shallow adapter compiles and generates browser bindings.
3. Dependency-free Node tests prove the recorded fixture is bounded and publishable, and the
   concrete SVG adapter projects growth/retraction and applies reduced-motion traces without
   scheduling.
4. Manual browser proof checks real geometry, SVG placement, resize behavior, no-JavaScript reading,
   reduced motion, 320 CSS-pixel reflow, and console errors.
5. `cargo make check` is the complete local default gate.

Tests exercise public interfaces. Moving a test outward must not widen the production interface.

## Determinism

Default tests must not depend on wall clock, ambient randomness, network access, locale, filesystem
order, or live services. Use explicit geometry, nonzero seeds, integer simulation time, sorted output,
and exact trace comparison. A replay failure must print the seed and input through the assertion.

Required native behaviors:

- all four final dispositions and their simulated or recorded source remain distinct in the public
  trace and JSON;
- identical input and seed produce identical complete traces and JSON;
- invalid or overlapping geometry fails before producing events;
- grown segments remain outside panel interiors except at selected ports;
- `SUCCEEDED` alone emits weighted progress with weights in the documented range;
- retraction indexes are the reverse of growth indexes;
- event density and durations remain within `PATTERNS.md` budgets.

## Browser evidence

Automated adapter tests do not prove browser layout, accessibility tree behavior, or visual quality.
Before a visual change is accepted, manually check keyboard reading order, reduced motion, no
JavaScript, 320-pixel width, text-spacing overrides, light/dark preference, and browser console.

## Meaningful-change record

```text
Contract:
Surface:
Gate:
Risk:
```

A passing gate does not erase unproved browser or visual risk; state it.
