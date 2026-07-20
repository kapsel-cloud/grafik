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
2. WASM build proof verifies the shallow adapter compiles and generates browser bindings.
3. Dependency-free Node tests prove the standalone lab uses only local Grafik assets and the concrete
   adapter maps every scene primitive, measured anchor, interaction, and reduced-motion trace without
   ambient randomness.
4. Manual browser proof checks flow controls, real geometry, connector placement, resize behavior,
   no-JavaScript reading, reduced motion, 320 CSS-pixel reflow, and console errors.
5. `cargo make check` is the complete local default gate.

Tests exercise public interfaces. Moving a test outward must not widen the production interface.

## Determinism

Default tests must not depend on wall clock, ambient randomness, network access, locale, filesystem
order, or live services. Use explicit geometry, nonzero seeds, integer simulation time, sorted output,
and exact trace comparison. A replay failure must print the seed and input through the assertion.

## Snapshot vocabulary and policy

Use distinct names for distinct state:

- A **conformance snapshot** is a reviewed, committed, exact serialized input/output example for one
  public Rust or WASM interface.
- A **variation** is a user-authored set of effect parameter values. It is product data, not test
  evidence.
- A **checkpoint** is a time-aware observation in a displayed flow. It is domain data, not a saved
  editor state or test artifact.
- A **visual baseline** is a screenshot produced in a pinned browser environment. It proves concrete
  rendering only and cannot replace semantic or invariant tests.

Conformance snapshots complement, never replace, direct assertions. Use direct tests for validation,
budgets, graph invariants, error behavior, and properties across seed corpora. Use snapshots for
complete stable examples whose accidental field, ordering, identifier, or timing drift must be
reviewed as a contract change.

A conformance snapshot contains an explicit case name, schema version, byte-equivalent input, and
byte-equivalent output. Inputs include every seed, time, geometry rectangle, interaction, and budget;
they contain no ambient values. Snapshot serializers use stable struct order and sorted collections;
production snapshot surfaces must not expose unordered maps.

Snapshot updates are a review operation, not a test side effect. The check path never rewrites a
committed baseline. An update path writes received candidates outside the committed baseline tree,
then requires an explicit per-file accept or reject step. Do not bulk-accept unexplained changes.
Every accepted change states which contract changed and why. CI runs snapshot checks but never the
accept path.

Pin a small representative corpus rather than every seed: one minimal case, each closed component or
effect kind, each semantic result, one maximum-budget case, one reduced-motion trace, and named rare
seeds. Property tests continue to scan wider seed ranges for invariant and budget failures.

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

Required animated-flow behaviors:

- the curated fixture contains exactly `grant`, `journal`, `provider seam`, `observe`, and `receipt`
  in one linear topology with directed edges in that order;
- every complete connected replay traverses each declared edge exactly once in declared order using
  measured boundary ports, and no segment enters any node interior except at its endpoint ports;
- disconnecting each edge in turn preserves topology, traverses only preceding edges, emits exactly
  one bounded break and spark cue, and emits no terminal cue;
- changing among `SUCCEEDED`, `FAILED`, and `UNKNOWN` preserves topology and explicit result text;
- a connected success emits one terminal approval role, a connected failure glitches only the
  terminal decorative backing, and unknown emits neither cue;
- byte-equivalent plan, geometry, interaction, outcome, and disconnect produce identical complete
  trace JSON;
- invalid outcomes, unknown disconnect IDs, missing or non-finite ports, and interior-crossing routes
  fail before events are returned; and
- adapter tests prove visible arrow markers, native pressed controls, explicit break projection,
  pointer-inert `aria-hidden` decorations, reduced-motion timer suppression, and no ambient
  randomness.

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
