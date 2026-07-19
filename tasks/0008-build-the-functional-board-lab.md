# Build the functional board lab

- **Status:** Ready
- **Blocked by:** [`0007-map-macros-and-variations.md`](0007-map-macros-and-variations.md)
- **Direct owners:** [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md), [`../docs/BUILD.md`](../docs/BUILD.md)

## Result

The standalone Grafik lab becomes a functional board editor. A reviewer can add all five component
kinds, move and resize them on the grid, connect a flow, attach and reorder effects, tune direct
parameters, map a macro, save/recall a variation, randomize eligible values, and inspect every pure
state transition without a downstream checkout.

## Adapter rule

The browser owns pointer/keyboard gestures, semantic DOM/SVG, focus, measurement, themes, and motion
preferences. It does not own board truth. Every completed edit dispatches a serializable command to
Rust and rerenders from the returned board/scene. DOM position, CSS classes, and form controls are
never an independent persistence model.

## Editor surfaces

- Component palette: `value`, `collection`, `flow`, `timeline`, `control`
- Twelve-column board with visible cells, spans, selection, and deterministic push preview
- Inspector for data binding, ports, rack order, direct parameters, macros, and variations
- Connection mode with explicit compatible ports
- Command history with undo/redo by deterministic replay
- Board, latest command, data frame, parameter state, scene, geometry, and trace JSON inspectors
- Shareable local URL for pinned example inputs; no server persistence

## Correctness strategy

- Dependency-free adapter tests cover command construction, stable ID association, measurement,
  logical DOM order, and reduced-motion trace consumption.
- Native interface tests remain authoritative for edits and effects; browser tests do not duplicate
  Rust rules.
- Every editor action has a keyboard path and visible focus.
- Replay tests serialize the command log, rebuild from the empty board, and compare exact board JSON.
- Resize tests prove the same board is remeasured and resimulated, never regenerated.
- Manual proof follows the complete matrix in `docs/TESTING.md` until the pinned visual/browser task
  automates it.

## Acceptance

- All five component kinds can be added and deleted through Rust commands.
- Pointer and keyboard move/resize produce the same command and final board.
- Push-and-reflow preview matches the accepted Rust result or displays the returned stable error.
- Flow connections expose only compatible target ports.
- Effect devices can be attached, reordered, tuned, enabled, and removed.
- Direct controls and macro knobs expose label, minimum, maximum, current value, and readable units;
  native range inputs are preferred over custom slider semantics.
- Variation and randomize actions visibly identify changed and excluded parameters.
- Undo/redo rebuilds state from commands and survives a complete JSON round trip.
- No editor path uses ambient randomness, wall time, network, or consumer-repository files.
- At 320 CSS px the board becomes one column in canonical logical order with no horizontal overflow.
- `cargo make check` passes and the manual browser evidence is recorded.

## Narrow proof

```sh
cargo make test-web
cargo make wasm
cargo make lab
cargo make check
```

## Residual risk

Unit tests cannot prove real pointer capture, accessibility-tree behavior, browser layout, or visual
quality. Those risks are closed by the pinned browser and visual matrix in the next task.
