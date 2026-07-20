# Technical task route

- **Status:** Active route
- **Kind:** Work and evidence tracker
- **Authority:** Canonical for unfinished public technical work
- **Owns:** Remaining work, acceptance evidence, blockers, and handoff notes
- **Does not own:** Product behavior or architectural decisions

Create focused Markdown task packets only when work must survive a session. Name them
`NNNN-imperative-result.md`. Each packet links its direct owner, states observable acceptance,
records commands run, and names residual risk. Completed packets may remain as bounded acceptance
provenance, but they are not part of the active route; archive or delete obsolete planning packets
rather than turning this directory into a second roadmap.

Private company strategy, positioning, adoption, pricing, partnerships, and interviews do not belong
here or elsewhere in this public repository.

## Current route

[`0011-prove-the-animated-flow-tracer.md`](0011-prove-the-animated-flow-tracer.md) is the active
standalone Grafik packet. It proves one deterministic simulated logical flow with directed animated
connectors, an explicit disconnect, and decorative outcome reinforcement without a live product
integration.

[`0012-prove-one-live-kapsel-consumer.md`](0012-prove-one-live-kapsel-consumer.md) is queued behind
the standalone tracer and the committed Kapsel sandbox consumer. It proves one real downstream
boundary without moving Kapsel semantics, networking, or authority into Grafik.

[`0010-prove-one-real-consumer-boundary.md`](0010-prove-one-real-consumer-boundary.md) is invalidated:
its earlier uncommitted completion claim is not supported by the current consumer repository or its
history. Do not count it as evidence. Packets 0002 through 0009 remain deferred and do not become an
active roadmap.

Correct the active packet's direct canonical owner before changing implementation, run its narrowest
proof, and finish with `cargo make check`.

## Deferred packets

1. [`0002-pin-conformance-snapshots.md`](0002-pin-conformance-snapshots.md)
2. [`0003-edit-a-functional-board.md`](0003-edit-a-functional-board.md)
3. [`0004-project-time-aware-data.md`](0004-project-time-aware-data.md)
4. [`0005-connect-flow-checkpoints.md`](0005-connect-flow-checkpoints.md)
5. [`0006-compose-bounded-effect-racks.md`](0006-compose-bounded-effect-racks.md)
6. [`0007-map-macros-and-variations.md`](0007-map-macros-and-variations.md)
7. [`0008-build-the-functional-board-lab.md`](0008-build-the-functional-board-lab.md)
8. [`0009-verify-the-pinned-browser-matrix.md`](0009-verify-the-pinned-browser-matrix.md)

## Definition of done for each packet

1. The direct canonical owner describes the accepted behavior before implementation changes.
2. One deep public Rust interface owns validation, stable errors, deterministic ordering, and
   budgets.
3. Direct tests prove examples and failures; invariant/property tests scan wider state and seed
   space.
4. Reviewed conformance snapshots pin complete representative public-interface JSON.
5. Concrete adapter tests prove translation only; browser/visual evidence is added when layout
   changes.
6. Snapshot updates identify the changed contract and are never accepted merely to make the gate
   pass.
7. The packet records commands run, acceptance evidence, and residual risk; a green gate alone is
   not sufficient evidence.
