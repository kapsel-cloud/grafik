# Architecture

- **Status:** Active bootstrap contract
- **Kind:** Normative architecture owner
- **Authority:** Canonical for module ownership, seams, adapters, and dependency direction
- **Owns:** Product module shape and the Rust/WASM/browser split
- **Does not own:** Spatial budgets, event details, visual styling, or command syntax

## Composition

Grafik starts as one product package with one deep simulation module:

```text
browser geometry -> WASM adapter -> simulation interface -> spatial event trace
                                                        |
                                      browser SVG adapter renders trace
```

`grafik::simulate` is the product interface and test surface. Behind it live geometry validation,
seeded weighted choices, orthogonal route construction, growth timing, and leaf-first retraction.
Callers provide all geometry and randomness inputs; the module returns data and performs no I/O.

The `wasm` Rust module is a shallow serialization adapter compiled only for `wasm32`. It translates
browser numbers into the same simulation input used by native tests and returns JSON. It must not
import browser DOM types.

The browser owns two concrete concerns without introducing a generic rendering interface:

- `web/main.js` measures DOM rectangles, selects edge ports, and calls generated WASM bindings.
- `web/svg-adapter.js` projects spatial events into SVG line segments and schedules visual traversal.

A second rendering implementation would make a renderer seam real. Until then, the SVG adapter
stays concrete and shallow.

## Dependency direction

- Domain types and simulation know nothing about WASM, JSON, SVG, CSS, DOM, clocks, or Kapsel.
- The WASM adapter depends inward on the simulation interface.
- Browser adapters depend on the generated WASM interface and spatial event schema.
- Rendering preferences never flow into simulation; reduced motion changes trace consumption, not
  trace generation.
- Future simulated and real clients may translate into one semantic vocabulary, but neither may
  fork spatial logic or grant browser infrastructure authority.

## Package discipline

One package is sufficient because native tests and WASM are two consumers of one Rust interface,
not separate deployment units. Apply the deletion test before extracting files or packages. Do not
create `core`, `common`, `utils`, `engine`, or a plugin interface without independent consumers.

## Error direction

Invalid geometry is rejected before randomness advances. Rust errors are stable, readable values;
the WASM adapter converts them to JavaScript errors. Rendering errors remain in the browser.
