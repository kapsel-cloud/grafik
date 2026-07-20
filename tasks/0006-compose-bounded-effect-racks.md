# Compose bounded effect racks

- **Status:** Deferred
- **Blocked by:** [`0005-connect-flow-checkpoints.md`](0005-connect-flow-checkpoints.md)
- **Direct owners:** [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/PATTERNS.md`](../docs/PATTERNS.md),
  [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

A component can own an ordered rack of a few deep decorative effect devices. Each device exposes
stable bounded parameters while hiding renderer-specific implementation. Rust resolves the serial
rack into one complete deterministic trace after browser geometry is measured.

## First devices

- `signal`: amount, attack, hold, release, repeats, palette role
- `glitch`: amount, displacement, slices, fragments, rate, decay
- `traverse`: duration, width, direction, packet count, trail
- `sweep`: amount, angle role, width, duration, repetitions

Each device declares supported targets from `backing`, `border`, `connector`, and `ornament`.
Readable labels and data are never legal targets. Parallel racks, plugins, CSS properties, SVG
commands, and user-provided executable effects remain out of scope.

## Parameter contract

Every parameter has a stable ID, label, closed value type, minimum/maximum or enum domain, default,
unit/display role, randomizable flag, and hard budget. Parameter state is separate from its schema.
Values are validated and normalized before trace generation. Automation and macro mappings are not
part of this task.

## Correctness strategy

- Exhaustive tests cover each device at minimum, default, and maximum legal parameters.
- Direct tests reject unknown parameter IDs, wrong types, NaN/infinity, range overflow, unsupported
  targets, duplicate devices, and rack-budget overflow.
- Serial-order tests prove swapping two devices changes the trace intentionally while replaying the
  same ordered rack does not.
- Property tests scan bounded parameter combinations and seeds for event density, displacement,
  fragments, duration, and live-effect limits.
- Conformance snapshots pin all four devices and one multi-device rack against fixed geometry.
- Adapter tests assert effects touch only `aria-hidden`, pointer-transparent decorative layers.

## Acceptance

- Board commands attach, detach, and reorder devices with stable IDs.
- Rack deletion and component deletion remove dependent parameter state deterministically.
- The same scene, geometry, cue, rack order, parameter state, and seed yields identical trace JSON.
- Every trace event references an existing legal target and remains inside hard budgets.
- `signal`, `glitch`, `traverse`, and `sweep` each produce visibly and structurally distinct events.
- Reduced motion consumes the same trace without scheduling decorative phases.
- Snapshot diffs expose parameter, event, target, phase, and ordering changes.
- Wider invariant tests remain independent of snapshots.
- `cargo make check` passes.

## Narrow proof

```sh
cargo make test-native
cargo make test-snapshots
cargo make test-web
cargo make check
```

## Residual risk

The closed devices may need parameter refinement after visual review. Change parameter identity or
domain only with an explicit schema and conformance-snapshot review.
