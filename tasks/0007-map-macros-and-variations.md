# Map macros and variations

- **Status:** Deferred
- **Blocked by:** [`0006-compose-bounded-effect-racks.md`](0006-compose-bounded-effect-racks.md)
- **Direct owners:** [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

The `control` component exposes direct effect parameters and a small Ableton-style macro bank. One
macro can drive several parameters through independent bounded mappings. Named variations save and
recall parameter values without changing board topology. Explicit seeded randomization affects only
opted-in parameters.

## V1 decisions

- A macro input is normalized from zero through one.
- Each mapping names one target parameter, output start/end, and a closed curve kind.
- Output start greater than output end represents inversion explicitly.
- Start with `linear` mapping only; add curves only after a real control needs one.
- Direct parameters remain inspectable even when published through macros.
- Variations are partial stable parameter-ID/value maps plus explicit exclusions.
- Automation lanes are deferred until direct control, macro mapping, variation recall, and seeded
  randomization are proven.

## Contract first

Define `Macro`, `MacroId`, `MacroMapping`, `Variation`, parameter precedence, exclusion behavior, and
randomization semantics. Specify whether direct edits update macro positions or only resolved target
values; do not leave bidirectional mapping ambiguous. The recommended v1 rule is one-way macro to
parameter resolution with direct edits stored independently and the latest explicit editor command
winning in replay order.

## Correctness strategy

- Direct tests cover one-to-one, one-to-many, inversion, clamping, exclusions, and deleted targets.
- A precedence table tests defaults, direct values, macro commands, variation recall, and randomize.
- Replay tests apply an explicit command log twice and compare parameter state and scene/trace JSON.
- Topology hashes before and after variation recall prove variations cannot mutate structure.
- Seed-corpus tests prove randomization changes eligible values, leaves excluded values unchanged,
  and always respects domains.
- Conformance snapshots pin one two-target macro, one inverted mapping, one variation, and one seeded
  randomization result.
- Accessibility tests use native range inputs where possible and prove keyboard/value semantics for
  any custom knob presentation.

## Acceptance

- Board commands add/delete/reorder macros and mappings with stable IDs.
- Missing, duplicate, incompatible, and cyclic macro targets are rejected.
- One macro maps to at least two parameters with independent output ranges.
- Variation save/recall is exact, deterministic, and topology-preserving.
- Seeded randomization accepts an explicit seed and changes only `randomizable` parameters.
- Randomization and variation exclusions are serialized and replayable.
- The `control` component renders direct controls, macros, variation actions, and current values with
  native accessible names and visible focus.
- Conformance snapshots use the testing term; product state consistently uses `variation`.
- `cargo make check` passes.

## Narrow proof

```sh
cargo make test-native
cargo make test-snapshots
cargo make test-web
cargo make check
```

## Residual risk

Automation curves and time-indexed parameter lanes remain future work. Adding them requires a clear
precedence contract with direct edits, macros, and variation recall rather than another ad hoc value
source.
