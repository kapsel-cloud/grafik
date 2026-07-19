# Technical task route

- **Status:** Active route
- **Kind:** Work and evidence tracker
- **Authority:** Canonical for unfinished public technical work
- **Owns:** Remaining work, acceptance evidence, blockers, and handoff notes
- **Does not own:** Product behavior or architectural decisions

Create focused Markdown task packets only when work must survive a session. Name them
`NNNN-imperative-result.md`. Each packet links its direct owner, states observable acceptance,
records commands run, and names residual risk. Delete or archive completed packets rather than
turning this directory into a second roadmap.

Private company strategy, positioning, adoption, pricing, partnerships, and interviews do not
belong here or elsewhere in this public repository.

## Active dependency chain

Follow this sequence; each packet owns its observable acceptance and residual risk:

1. [`0002-pin-conformance-snapshots.md`](0002-pin-conformance-snapshots.md)
2. [`0003-edit-a-functional-board.md`](0003-edit-a-functional-board.md)
3. [`0004-project-time-aware-data.md`](0004-project-time-aware-data.md)
4. [`0005-connect-flow-checkpoints.md`](0005-connect-flow-checkpoints.md)
5. [`0006-compose-bounded-effect-racks.md`](0006-compose-bounded-effect-racks.md)
6. [`0007-map-macros-and-variations.md`](0007-map-macros-and-variations.md)
7. [`0008-build-the-functional-board-lab.md`](0008-build-the-functional-board-lab.md)
8. [`0009-verify-the-pinned-browser-matrix.md`](0009-verify-the-pinned-browser-matrix.md)

Do not begin a packet before its blocker passes. Correct the direct canonical owner before changing
implementation, then run the narrowest proof named by the packet and finish with `cargo make check`.

## Definition of done for each packet

1. The direct canonical owner describes the accepted behavior before implementation changes.
2. One deep public Rust interface owns validation, stable errors, deterministic ordering, and budgets.
3. Direct tests prove examples and failures; invariant/property tests scan wider state and seed space.
4. Reviewed conformance snapshots pin complete representative public-interface JSON.
5. Concrete adapter tests prove translation only; browser/visual evidence is added when layout changes.
6. Snapshot updates identify the changed contract and are never accepted merely to make the gate pass.
7. The packet records commands run, acceptance evidence, and residual risk; a green gate alone is not
   sufficient evidence.
