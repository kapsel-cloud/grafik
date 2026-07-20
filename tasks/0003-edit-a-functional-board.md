# Edit a functional board

- **Status:** Deferred
- **Blocked by:** [`0002-pin-conformance-snapshots.md`](0002-pin-conformance-snapshots.md)
- **Direct owners:** [`../docs/SCOPE.md`](../docs/SCOPE.md),
  [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

One deep Rust board module applies explicit editor commands to pure serializable state. The first
vertical tracer adds, moves, resizes, reorders, and deletes `value` components on a canonical
twelve-column integer grid using deterministic push-and-reflow collision handling.

## Contract first

Before implementation, update the canonical owners to define:

- `Board`, `BoardVersion`, `ComponentId`, canonical logical order, and grid placement;
- `apply(board, command) -> board | edit error` as the public test surface;
- caller-supplied stable IDs and duplicate-ID behavior;
- valid columns, rows, spans, minimum sizes, and board budgets; and
- the exact push-and-reflow and logical-reorder rules.

Grid position never defines logical order. A command that changes reading/evaluation order uses an
explicit reorder operation. Mobile projection is not part of this task.

## First command set

```text
add_value
move_component
resize_component
reorder_component
delete_component
```

Commands carry every stable ID and coordinate needed for replay. `apply` performs no I/O, reads no
clock or randomness, and returns a new complete board. Failed commands return the original state
unchanged.

## Correctness strategy

- Table-driven tests cover every command and error.
- Invariant tests assert unique IDs, in-bounds spans, no overlap, canonical order, and node budgets
  after every successful command.
- Command-sequence tests apply the same sequence twice and compare complete board JSON.
- A bounded generated command corpus checks invariants after every prefix, not only final state.
- Conformance snapshots pin an empty board, a composed board, a push cascade, and delete/reorder.
- Mutation tests are not required, but each invariant receives one negative regression case proving
  the test fails for a malformed board.

## Acceptance

- Add rejects duplicate IDs and impossible placements before state changes.
- Move and resize use one documented deterministic push direction and tie-break order.
- Push cascades terminate within hard row/component budgets or return a stable error.
- Delete removes exactly one component and its placement without renumbering other IDs.
- Reorder changes canonical logical order without changing placement.
- Byte-equivalent board plus command produces byte-equivalent board JSON and error values.
- The WASM adapter exposes one stateless JSON edit call using the native interface.
- Board and command snapshots pass through `cargo make test-snapshots`.
- `cargo make check` passes without browser, network, or sibling repository input.

## Narrow proof

```sh
cargo make test-native
cargo make test-snapshots
cargo make wasm
cargo make check
```

## Residual risk

This tracer proves grid editing with one component kind only. It does not yet prove data binding,
typed flow connections, undo UI, effects, macros, or visual drag behavior.
