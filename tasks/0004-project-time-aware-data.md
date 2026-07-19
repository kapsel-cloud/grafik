# Project time-aware data

- **Status:** Ready
- **Blocked by:** [`0003-edit-a-functional-board.md`](0003-edit-a-functional-board.md)
- **Direct owners:** [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

A second deep Rust interface projects a board plus one bounded external data frame into a complete
renderer-neutral scene. The tracer supports `value`, `collection`, and `timeline` components while
keeping observed timestamps distinct from deterministic animation time.

## Contract first

Define these concepts in their canonical owners before implementation:

```text
project(board, data_frame, parameter_state, observed_time) -> scene_plan | projection error
DataFrame { observed_at, bindings[] }
ValueBinding
CollectionBinding { list | table }
TimelineBinding { spans[], events[] }
```

The frame is caller-provided display data, not a live transport, query language, receipt parser, or
browser cache. Missing, stale, unknown, and invalid values remain distinct. Projection does not read
wall time and does not mutate the board.

## Correctness strategy

- Direct tests prove binding lookup, type checking, bounds, missing data, and timestamp semantics.
- Exact tests prove canonical logical order remains scene order regardless of grid placement.
- Invariant tests prove every rendered node references one board component and every visible datum
  comes from a declared bounded binding.
- Seed-corpus tests prove presentation variation never changes data values, labels, timestamps, or
  semantic status.
- Conformance snapshots pin one value, list, table, and span/event timeline projection.
- A metamorphic test changes only observed time and proves component topology and effect-free layout
  remain unchanged.

## Acceptance

- `value` projects scalar, status, observed time, and provenance roles without product reclassification.
- `collection` projects the same binding as a semantic list or table with explicit headers.
- `timeline` projects bounded spans and timestamped events with stable IDs and ordering rules.
- Unordered caller collections are normalized before serialization.
- Data-frame budgets cap strings, rows, columns, spans, events, and total serialized bytes.
- Byte-equivalent board, frame, parameters, and observed time produce byte-equivalent scene JSON.
- The WASM projection call is stateless and uses the native interface.
- Every new projection case has direct assertions plus reviewed conformance snapshots.
- Adapter tests prove native list/table/time markup and logical DOM order.
- `cargo make check` passes.

## Narrow proof

```sh
cargo make test-native
cargo make test-snapshots
cargo make test-web
cargo make check
```

## Residual risk

This task does not model flow ports, checkpoints, effect parameters, streaming updates, or browser
virtualization for long timelines.
