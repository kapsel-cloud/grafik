# Connect flow checkpoints

- **Status:** Deferred
- **Blocked by:** [`0004-project-time-aware-data.md`](0004-project-time-aware-data.md)
- **Direct owners:** [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

Boards can add a `flow` component, connect stable typed ports, and project real-world checkpoints and
links into measured diagrams. Checkpoints carry observed-time data; they are not editor snapshots or
parameter variations.

## V1 decisions

- Start with three closed port value families: `record`, `event`, and `trigger`.
- Connections are directed output-port to input-port and have stable IDs.
- V1 board topology is acyclic. Cycles require a later explicit delay/feedback component and a new
  scheduling contract; visual loops alone do not justify runtime cycles.
- Graph evaluation order comes from validated dependencies and stable tie-breaks, never grid position.
- Every rendered connector derives from declared topology and browser-measured ports.

## Contract first

Define `PortSpec`, `PortId`, `Connection`, `Checkpoint`, `FlowRun`, checkpoint state, link semantics,
and graph budgets before implementation. Specify whether each input accepts one or many connections
and whether each checkpoint transition is merely observed or inferred. Grafik must preserve supplied
states and timestamps rather than inventing causation.

## Correctness strategy

- Direct tests reject missing ports, reversed directions, incompatible families, duplicate edges,
  forbidden cardinality, self-edges, cycles, and budget overflow.
- Topological-order tests use stable IDs as the documented tie-break and compare complete order.
- Property tests generate bounded directed acyclic graphs and assert every edge references existing
  ports, every node appears once, and evaluation terminates.
- Geometry tests prove measured endpoints overlap node boundaries by the documented amount and never
  use a second coordinate model.
- Conformance snapshots pin linear, branched, linked, and unknown checkpoint flows.
- A negative regression test mutates one edge endpoint and proves both invariant and snapshot checks
  fail for different reasons.

## Acceptance

- Board commands add/delete ports and connect/disconnect edges without leaving dangling references.
- Deleting a component removes its incident connections in one documented deterministic order.
- Flow projection preserves checkpoint IDs, observed timestamps, state, attributes, and non-parent
  links exactly.
- `record`, `event`, and `trigger` compatibility is closed and exhaustively tested.
- Cycle detection fails before projection or seeded choices.
- Browser measurement reports stable node ports; Rust emits connector/packet coordinates.
- At least linear, tree, state, and network projections derive edges from one topology.
- Exact board, scene, and measured-trace snapshots pass.
- Reduced motion applies the readable final flow state without traversal.
- `cargo make check` passes.

## Narrow proof

```sh
cargo make test-native
cargo make test-snapshots
cargo make test-web
cargo make check
```

## Residual risk

The first port families may be insufficient for a second real product adapter. Do not add a generic
port type, arbitrary evaluator, or cycle semantics until concrete data proves the need.
