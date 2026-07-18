# Build and run

- **Status:** Active command guide
- **Kind:** Executable build authority
- **Authority:** Canonical for prerequisites and commands that exist in this repository
- **Owns:** Local command syntax, tool pins, and default-gate composition
- **Does not own:** Test meaning, style rationale, architecture, or release policy

## Prerequisites

- Rust `1.97.1`, selected by `rust-toolchain.toml`, with `clippy`, `rustfmt`, and
  `wasm32-unknown-unknown`
- `cargo-make` 0.37 or later
- `wasm-bindgen-cli` exactly `0.2.126`, matching the Rust dependency
- Node.js 22 or later for dependency-free browser-adapter tests
- Python 3.11 or later for the static preview server

Install the repository-specific Rust additions once:

```sh
rustup target add wasm32-unknown-unknown --toolchain 1.97.1
cargo install wasm-bindgen-cli --version 0.2.126 --locked
```

## Default gate

```sh
./scripts/ci-local.sh
# equivalent aliases
cargo make check
cargo make ci
```

The gate checks Rust formatting and 100-byte source width, whitespace, Clippy, rustdoc, native tests,
WASM compilation/bindings, and browser-adapter tests. It is deterministic and requires no live
service.

## Narrow commands

```sh
cargo make build       # native debug build with Cargo.lock
cargo make test        # native library and documentation tests
cargo make lint        # Clippy with warnings denied
cargo make docs        # warning-free rustdoc
cargo make wasm        # produce web/pkg/grafik.js and grafik_bg.wasm
cargo make test-web    # dependency-free Node tests for the SVG adapter
cargo make fmt         # rewrite Rust formatting
cargo make fmt-check   # check Rust formatting and source width
```

Use the narrowest command while iterating, then run the default gate.

## Browser tracer

Build bindings before serving:

```sh
cargo make wasm
cargo make serve
```

Open `http://127.0.0.1:4173`. `serve` runs
`python3 -m http.server 4173 --bind 127.0.0.1 --directory web` and remains in the foreground until
interrupted. No generated binding is committed; rerun `cargo make wasm`
after Rust changes.

## Authoritative inputs

`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `Makefile.toml`, and `scripts/ci-local.sh` are the
executable inputs. Update this guide in the same change whenever their public commands change.
