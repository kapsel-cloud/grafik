# Simulation

- **Status:** Active experimental contract
- **Kind:** Normative simulation and trace owner
- **Authority:** Canonical for seeds, controlled time, replay, and spatial event vocabulary
- **Owns:** Simulation input, deterministic behavior, event ordering, and trace meaning
- **Does not own:** DOM geometry acquisition, SVG projection, wall-clock scheduling, or themes

## Interface

The Rust interface accepts a `SimulationInput` containing a nonzero `u64` seed, result source, final
disposition, hero and receipt panel rectangles, and selected edge ports. Coordinates are finite
CSS-pixel values in one browser coordinate space. The source is either `simulated`, which performs
no infrastructure work, or `recorded`, which identifies an intentionally published earlier result.

`simulate(input)` either returns a complete `Trace` or a validation error. The trace contains the
seed and ordered spatial events. It performs no I/O and reads no ambient clock or randomness.

## Seed and randomness

The implementation owns a small, documented deterministic generator. Every weighted choice is
derived from the input seed. Seed zero is rejected rather than silently substituted. Algorithm and
event schema are experimental; exact traces may change before a compatibility decision.

## Controlled time

Simulation time is integer milliseconds accumulated from deterministic event durations. Browser
wall time only controls how quickly events are displayed and cannot alter route or event order.

## Spatial events

Every trace emits `connector_started` with selected ports and `connector_finished` with final
simulation time. For `SUCCEEDED`, the events between them are:

1. One or more `segment_grew` events with segment index, seeded progress weight, endpoints, start
   time, and duration.
2. Matching `segment_retracted` events in descending index order with start time and duration.

Other dispositions do not reuse successful progress and emit no events between connector start and
finish.

Events are renderer-neutral coordinates and timing descriptions. They contain no selectors, SVG
commands, CSS classes, colors, or DOM objects.

## Replay guarantee

For the same package version and byte-equivalent valid input, repeated calls produce an identical
`Trace` and identical JSON representation. Tests compare complete traces and independently assert
leaf-first retraction. Different seeds are allowed to converge when geometry removes optional
choices; determinism does not promise visual uniqueness.

## Semantic result seam

The final disposition is exactly one of:

- `NOT_ATTEMPTED`: local rejection before an attempt; it is not receiver `FAILED` or `UNKNOWN`;
- `SUCCEEDED`: receiver facts established the adapter's defined successful outcome;
- `FAILED`: receiver facts established the adapter's defined failed outcome; or
- `UNKNOWN`: bounded observation established neither success nor failure.

Grafik preserves this value and its `simulated` or `recorded` source in the trace without parsing
receipts or reclassifying the result. For the KAP-0038 adapter, `SUCCEEDED` does not establish
causation, workload correctness, complete cluster health, or universal capture; `FAILED` does not
establish permanence or cause; and `UNKNOWN` does not mean the request failed, was not received, or
was harmless. A timeout remains `UNKNOWN`.

The seam is experimental and intentionally smaller than a cross-product protocol. It accepts no
receipt bytes, trust decision, operation authority, credentials, private identifiers, or live
transport. `recorded` identifies provenance, not infrastructure truth, receipt verification, or a
production claim.
