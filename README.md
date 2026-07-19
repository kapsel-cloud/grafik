# grafik

**Living diagrams for executable product explanations.**

Grafik is an experimental open-source Rust/WASM explanation system. A living diagram is a seeded
simulation whose semantic events deform a spatial field around readable panels. The first tracer
grows and retracts one connector between two measured panel edge ports.

## Status

Pre-release tracer only. The Rust interfaces generate deterministic renderer-neutral scene plans and
complete timed traces; the included browser adapters are narrow proofs, not stable interfaces or a
production-ready animation library. Breaking changes are expected between alpha releases.

## Use the alpha

Pin the exact pre-release while the interface is experimental:

```toml
[dependencies]
grafik = "=0.1.0-alpha.1"
```

The native Rust interface is the contract and test surface. Browser consumers compile the same crate
for `wasm32-unknown-unknown` and generate bindings locally; crates.io does not ship prebuilt browser
JavaScript or WASM.

## Develop independently

Prerequisites and exact commands are owned by [`docs/BUILD.md`](docs/BUILD.md).

```sh
cargo make check
cargo make lab
```

Open `http://127.0.0.1:4174/lab.html`. The standalone lab builds and serves files from this checkout,
uses simulated inputs, and exposes complete scene and trace JSON without requiring a live service.

## Read next

- Purpose and first tracer: [`docs/SCOPE.md`](docs/SCOPE.md)
- Question-to-owner routing: [`docs/INDEX.md`](docs/INDEX.md)
- Simulation seam: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- Spatial vocabulary and limits: [`docs/PATTERNS.md`](docs/PATTERNS.md)
- Determinism and trace contract: [`docs/SIMULATION.md`](docs/SIMULATION.md)
- Contribution workflow: [`CONTRIBUTING.md`](CONTRIBUTING.md)

## Package identity

The crate is named `grafik`. Version `0.1.0-alpha.1` is the first experimental source release; it
makes no stable compatibility or production-readiness promise. Published versions are built from the
Rust source and contract documentation in the release archive.

Licensed under Apache-2.0. See [`LICENSE`](LICENSE).
