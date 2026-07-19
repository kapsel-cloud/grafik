# Testing

- **Status:** Active proof strategy
- **Kind:** Normative testing owner
- **Authority:** Canonical for evidence, determinism, and proof layering
- **Owns:** What tests must prove and where proof belongs
- **Does not own:** Command syntax, implementation seams, or review result formatting

## Proof layers

Run the narrowest proof first:

1. Native interface tests prove deterministic scene generation, budget enforcement, stable tree and
   topology IDs, measured-geometry validation, interaction ordering, route exclusion, and bounded
   timing through `grafik::generate_scene` and `grafik::simulate_scene`.
2. Existing native outcome-tracer tests remain until that implementation is absorbed.
3. WASM build proof verifies the shallow adapter compiles and generates browser bindings.
4. Dependency-free Node tests prove the standalone lab uses only local Grafik assets and the concrete
   adapter maps every scene primitive, measured anchor, interaction, and reduced-motion trace without
   ambient randomness.
5. Manual browser proof checks the candidate grid, clickable interactions, real geometry, connector
   placement, resize behavior, no-JavaScript reading, reduced motion, 320 CSS-pixel reflow, and console
   errors.
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
- `SUCCEEDED` emits weighted progress, one bounded pulse, and leaf-first retraction;
- `FAILED` emits one bounded decorative glitch and no successful progress;
- `UNKNOWN` emits a two-second seeded burst at 1–3 marks per second, with normal and rare density
  limits independently asserted;
- `NOT_ATTEMPTED` emits no receiver cue;
- every profile's density, lifetime, displacement, and duration remain within `PATTERNS.md` budgets.

Required receipt-scene behaviors:

- byte-equivalent request and seed produce identical complete scene plans and JSON;
- separate random streams keep layout stable when only effect timing implementation changes;
- invalid content or impossible budgets fail before seeded choices advance;
- all scene trees remain within node/depth limits and use unique stable IDs;
- lists, tables, actions, menus, and every diagram form survive serialization;
- every diagram edge references existing topology nodes;
- every effect references an existing target and every action references a bounded script;
- button sizes, layouts, diagrams, pattern profiles, and phases vary across a pinned seed corpus;
- byte-equivalent plan, geometry, and interaction produce identical complete traces and JSON;
- missing, duplicate, stale, or non-finite measured geometry is rejected;
- every live-effect, density, lifetime, displacement, fragment, phase, and total-duration budget holds;
- no event can target or move readable text; and
- the complete default gate reads or writes no sibling checkout.

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
