# Simulation

- **Status:** Active experimental contract
- **Kind:** Normative simulation and trace owner
- **Authority:** Canonical for seeds, controlled time, replay, and spatial event vocabulary
- **Owns:** Simulation input, deterministic behavior, event ordering, and trace meaning
- **Does not own:** DOM geometry acquisition, SVG projection, wall-clock scheduling, or themes

## Receipt scene generation

`generate_scene(request)` accepts bounded display content, a nonzero `u64` seed, one recipe, and hard
budgets. Display content contains text and semantic structure but no markup, selectors, coordinates,
literal colors, or product receipt bytes. Generation returns one complete `ScenePlan` containing:

- a version, seed, and selected curated profile;
- one stable scene-node tree in logical reading order;
- grid, visual, and button-size roles;
- diagram nodes and topology;
- named effect targets and bounded pattern plans; and
- deterministic load and action interaction scripts.

Stable node and action IDs are assigned from deterministic tree order, not caller memory addresses or
map iteration. Every random choice comes from one documented seeded stream. Validation and budget
checks finish before that stream advances.

`simulate_scene(input)` accepts a complete generated plan, one measured rectangle and selected ports
for each required stable node ID, and either `load` or one action ID declared by the plan. It returns
a complete renderer-neutral timed trace. Missing, duplicate, stale, non-finite, or impossible
geometry fails before any event is returned. Connector and packet coordinates are derived only from
declared diagram topology and measured ports.

The WASM adapter exposes these as `grafik_scene(request_json)` and
`grafik_scene_trace(input_json)`. It is stateless: the browser passes the plan it received back with
measured geometry. This makes exact replay, inspection, and version mismatch visible rather than
relying on hidden WASM state.

## Outcome tracer interface

The existing Rust interface accepts a `SimulationInput` containing a nonzero `u64` seed, result source, final
disposition, hero and receipt panel rectangles, selected edge ports, and an outcome-text region.
Coordinates are finite CSS-pixel values in one browser coordinate space. The source is either
`simulated`, which performs
no infrastructure work, or `recorded`, which identifies an intentionally published earlier result.

`simulate(input)` either returns a complete `Trace` or a validation error. The trace contains the
seed and ordered spatial events. It performs no I/O and reads no ambient clock or randomness.

## Seed and randomness

The implementation owns a small, documented deterministic generator. Scene generation and timed
simulation use separate streams derived from the request seed so adding a timing choice cannot
silently change the generated layout. Every weighted choice is derived from one of those streams.
Seed zero is rejected rather than silently substituted. Algorithm, scene schema, and event schema
are experimental; exact plans and traces may change before a compatibility decision.

## Controlled time

Simulation time is integer milliseconds accumulated from deterministic event durations. Browser
wall time only controls how quickly events are displayed and cannot alter route or event order.

## Receipt-scene events

A scene trace starts with `interaction_started` and ends with `interaction_finished`. Events between
those boundaries come only from the selected interaction script and include stable target IDs. The
initial vocabulary is `phase_started`, `node_activated`, `edge_traversed`, `packet_traversed`,
`backing_glitched`, `backing_inverted`, `fragments_emitted`, and `scanline_swept`.
Events carry integer time, bounded numeric parameters, measured coordinates when required, and
semantic palette roles. They carry no selectors, markup, SVG commands, CSS classes, or literal
colors.

A button press references one stable action ID and may affect only targets declared by that action's
script. Replaying an interaction does not regenerate the scene. Reduced motion consumes the same
trace as an immediate final readable state and suppresses decorative traversal.

## Outcome-tracer spatial events

Every outcome trace begins with `outcome_started` and ends with `outcome_finished`. Events between them are
selected only by final disposition:

- `SUCCEEDED`: `connector_started`, one or more weighted `segment_grew` events, one
  `success_pulsed` event, matching leaf-first `segment_retracted` events, then
  `connector_finished`.
- `FAILED`: one `failure_glitched` event carrying bounded duration, displacement, and strip count.
- `UNKNOWN`: bounded `question_mark_appeared` events carrying absolute coordinates, lifetime, and
  one of three palette roles.
- `NOT_ATTEMPTED`: no receiver-cue event.

Events are renderer-neutral coordinates and timing descriptions. They contain no selectors, SVG
commands, CSS classes, literal colors, or DOM objects.

## Replay guarantee

For the same package version and byte-equivalent valid request, repeated scene generation produces an
identical `ScenePlan` and identical JSON representation. For the same plan, interaction, and
byte-equivalent measured geometry, repeated simulation produces an identical trace and JSON.
Tests compare complete plans and traces. Different seeds are allowed to converge when curated
choices or geometry remove optional variation; determinism does not promise visual uniqueness.

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
