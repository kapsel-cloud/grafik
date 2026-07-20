# Verify the pinned browser matrix

- **Status:** Deferred
- **Blocked by:** [`0008-build-the-functional-board-lab.md`](0008-build-the-functional-board-lab.md)
- **Direct owners:** [`../docs/RENDERING.md`](../docs/RENDERING.md),
  [`../docs/TESTING.md`](../docs/TESTING.md), [`../docs/BUILD.md`](../docs/BUILD.md)

## Result

Grafik has reproducible browser behavior tests and reviewed visual baselines for a small pinned board
matrix. Semantic assertions, geometry assertions, conformance snapshots, and screenshots provide
separate evidence; no single snapshot type is treated as proof of everything.

## Tool decision gate

Before adding a browser dependency, read its current official installation, browser-pinning,
screenshot, accessibility, trace, and CI documentation. Record the exact version and lockfile. The
selected runner must work against the local static server, install no global mutable state during a
test, and run without network after setup.

Update `docs/BUILD.md`, the task runner, and CI in the same change. Introduce real commands such as:

```sh
cargo make test-browser   # semantic behavior, keyboard, geometry, console
cargo make test-visual    # compare the reviewed pinned image matrix
```

Do not document these commands until they exist.

## Evidence layers

1. Native invariant/property tests prove board, graph, parameter, and trace semantics.
2. Conformance snapshots pin exact board, scene, and trace JSON.
3. Browser behavior tests prove DOM semantics, keyboard paths, measured geometry, resize, and motion
   preference handling.
4. Visual baselines prove the concrete adapter's appearance in one pinned environment.
5. Manual review still covers subjective art direction and assistive-technology combinations outside
   the automated matrix.

## Pinned matrix

Keep the baseline set intentionally small and named by purpose:

- desktop light, normal motion: complete five-component board;
- desktop dark, normal motion: same board and data;
- mobile 320 CSS px, light: canonical one-column reading order;
- mobile 320 CSS px, dark, reduced motion: readable final state with no scheduled effects;
- focused Signal, Glitch, Traverse, and Sweep midpoint frames;
- focused macro mapping before/after and variation recall;
- linear, branched, state, and network measured flow forms.

Every case pins board, data frame, parameters, seed, viewport, color scheme, motion preference, and
capture time. Effect screenshots capture a deterministic simulation-time frame rather than sleeping
for wall-clock timing.

## Visual baseline policy

- Baselines live in a versioned test directory and use stable descriptive names.
- The check path never rewrites them.
- Mismatches write received images, diffs, and metadata below `target/`.
- Acceptance is per image after inspecting the semantic/conformance diff and rendered diff together.
- Do not bulk-accept unexplained image changes.
- Pin browser version, viewport, device scale, locale, timezone, color scheme, motion, and fonts.
- A threshold must be justified by measured anti-aliasing behavior; never use a large threshold to
  hide layout movement.

## Acceptance

- Keyboard tests cover add, select, move, resize, connect, parameter change, macro change, variation
  recall, randomize, undo, and redo.
- Accessibility assertions cover native landmarks, lists, tables, labels, names, states, focus order,
  and decorative `aria-hidden` layers.
- Geometry assertions prove connectors end at measured ports with the documented overlap after load
  and resize.
- Reduced motion schedules no decorative timers and preserves readable final state.
- Every test fails on browser console errors, unhandled rejection, missing WASM, or horizontal
  overflow at 320 CSS px.
- Visual comparisons emit expected, received, diff, and case metadata.
- Re-running the complete matrix twice in the pinned environment produces no diff.
- CI archives received evidence on failure and never accepts baselines.
- The default gate includes stable browser behavior tests; the documented visual command runs the
  pinned image matrix.

## Narrow proof

```sh
cargo make test-browser
cargo make test-visual
cargo make check
```

## Residual risk

Pinned screenshots do not prove every browser, GPU, font rasterizer, assistive technology, seed, or
subjective design judgment. Keep the matrix small, deterministic, and paired with manual review.
