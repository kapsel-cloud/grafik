# Pin conformance snapshots

- **Status:** Deferred
- **Blocked by:** [`0010-prove-one-real-consumer-boundary.md`](0010-prove-one-real-consumer-boundary.md)
- **Direct owners:** [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/TESTING.md`](../docs/TESTING.md), [`../docs/BUILD.md`](../docs/BUILD.md)

## Result

Grafik has a transparent, dependency-free conformance snapshot harness for exact public-interface
JSON. A mismatch produces received candidates outside the committed baseline tree and fails the
check. Reviewers accept or reject each changed file explicitly; tests and CI never rewrite baselines.

## Surface

```text
tests/conformance/v1/<case>/case.json
tests/conformance/v1/<case>/input.json
tests/conformance/v1/<case>/output.json
target/conformance-received/v1/<case>/...
```

`case.json` names the public interface and schema version. `input.json` and `output.json` are
canonical UTF-8 with one trailing newline. The manifest lists cases explicitly; the harness never
depends on directory iteration order.

This task introduces and documents these commands in `Makefile.toml` and `docs/BUILD.md`:

```sh
cargo make test-snapshots      # compare fresh exact outputs with committed baselines
cargo make snapshots-receive   # write candidates under target/conformance-received
```

There is intentionally no bulk accept command. Acceptance is a reviewed file copy after inspecting
the semantic diff and naming the changed contract.

## First pinned cases

- Existing balanced receipt scene plan at one fixed seed
- Existing measured load interaction trace with fixed geometry
- All four final dispositions through the outcome tracer
- One rare seeded uncertainty profile
- One maximum-budget receipt scene

Invalid input and error wording remain direct assertions, not snapshots.

## Implementation guidance

- Generate through the same public Rust interfaces used by callers.
- Compare exact serialized bytes, including field and event order.
- Use only structs, vectors, and sorted collections on snapshot surfaces; reject unordered maps.
- Generate each case twice and fail before comparison if the two outputs differ.
- On mismatch, write the complete received case under `target/` and report expected/received paths.
- Keep the committed cases human-readable and free of paths, tool versions, timestamps, or revisions.
- Add snapshot checks to `cargo make check`; never add the receive path to CI.

## Acceptance

- Repeated generation produces byte-identical received cases.
- Changing one serialized field fails `cargo make test-snapshots` and leaves committed files untouched.
- `cargo make snapshots-receive` writes only below `target/conformance-received/`.
- A test proves no snapshot case contains absolute paths or ambient timestamps.
- Direct invariant tests still cover validation, budgets, geometry, and event ordering.
- `cargo make check` runs the conformance comparison without network or sibling repositories.

## Narrow proof

```sh
cargo make test-snapshots
cargo make test
cargo make check
```

## Residual risk

Exact snapshots intentionally make schema and ordering changes noisy. Reviewers must reject updates
that merely hide unexplained drift; snapshot stability is not a promise of public compatibility
until the project makes that separate decision.
