# Grafik by Kapsel

**Living diagrams for executable product explanations.**

Grafik is an experimental open-source Rust/WASM explanation system. A living diagram is a seeded
simulation whose semantic events deform a spatial field around readable panels. The first tracer
grows and retracts one connector between two measured panel edge ports.

## Status

Bootstrap tracer only. The simulation trace is deterministic and renderer-neutral; the included
browser SVG adapter is a narrow proof, not a stable interface or production-ready animation
library.

Grafik does not connect to Kubernetes or a Kapsel gateway, does not hold credentials, and does not
manipulate the DOM from WASM. It preserves one simulated or recorded final disposition without
parsing Kapsel receipts or claiming infrastructure truth.

## Run the proof

Prerequisites and exact commands are owned by [`docs/BUILD.md`](docs/BUILD.md).

```sh
cargo make check
cargo make wasm
cargo make serve
```

Open `http://127.0.0.1:4173`. The browser loads one sanitized recorded KAP-0038 result, measures the
hero and receipt panels, asks WASM for a seeded trace, and renders it through SVG. This remains a
simulated, non-production presentation. Reduced-motion users receive the final readable state
without animated traversal.

## Read next

- Purpose and first tracer: [`docs/SCOPE.md`](docs/SCOPE.md)
- Question-to-owner routing: [`docs/INDEX.md`](docs/INDEX.md)
- Simulation seam: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- Spatial vocabulary and limits: [`docs/PATTERNS.md`](docs/PATTERNS.md)
- Determinism and trace contract: [`docs/SIMULATION.md`](docs/SIMULATION.md)

## Package identity

A crates.io search on 2026-07-18 returned no exact `grafik` or `kapsel-grafik` package. The local
package is named `grafik` but is marked `publish = false`; registry availability is not a promise
and must be rechecked before any publication decision.

Licensed under Apache-2.0. See [`LICENSE`](LICENSE).
