# Simulation

- **Status:** Active tracer contract
- **Kind:** Normative simulation and trace owner
- **Authority:** Canonical for seeds, controlled time, replay, and spatial event vocabulary
- **Owns:** Simulation input, deterministic behavior, event ordering, and trace meaning
- **Does not own:** DOM geometry acquisition, SVG projection, wall-clock scheduling, or themes

## Interface

The Rust interface accepts a `SimulationInput` containing a nonzero `u64` seed, hero and receipt
panel rectangles, and selected edge ports. Coordinates are finite CSS-pixel values in one browser
coordinate space.

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

The tracer emits:

1. `connector_started` with selected ports.
2. One or more `segment_grew` events with segment index, endpoints, start time, and duration.
3. Matching `segment_retracted` events in descending index order with start time and duration.
4. `connector_finished` with the final simulation time.

Events are renderer-neutral coordinates and timing descriptions. They contain no selectors, SVG
commands, CSS classes, colors, or DOM objects.

## Replay guarantee

For the same package version and byte-equivalent valid input, repeated calls produce an identical
`Trace` and identical JSON representation. Tests compare complete traces and independently assert
leaf-first retraction. Different seeds are allowed to converge when geometry removes optional
choices; determinism does not promise visual uniqueness.

## Semantic event seam

The tracer has no Kapsel receipt outcomes. Future simulated and real-client adapters must emit the
same semantic vocabulary before entering spatial evolution. A simulated run must be identified as
simulated and perform no infrastructure work.
