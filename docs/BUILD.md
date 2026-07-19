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
WASM compilation/bindings, browser-artifact attribution and checksums, and browser-adapter tests. It
is deterministic and requires no live service.

## Narrow commands

```sh
cargo make build       # native debug build with Cargo.lock
cargo make test        # native library and documentation tests
cargo make lint        # Clippy with warnings denied
cargo make docs        # warning-free rustdoc
cargo make wasm        # produce web/pkg/grafik.js and grafik_bg.wasm
cargo make lab         # build WASM and serve the standalone Grafik lab
cargo make artifact    # produce the attributable browser proof in dist
cargo make test-artifact # validate artifact metadata and SHA-256 checksums
cargo make test-web    # dependency-free Node tests for the SVG adapter
cargo make fmt         # rewrite Rust formatting
cargo make fmt-check   # check Rust formatting and source width
```

Use the narrowest command while iterating, then run the default gate.

## Browser artifact

`cargo make artifact` replaces `dist/` with browser JavaScript, WASM, static assets, and an
`artifact.json` manifest. The manifest records the source revision, source-tree state, exact tool
version output, and an explicit experimental/non-production classification. `SHA256SUMS` covers all
other files in sorted path order. Generated artifacts are ignored by Git.

The command is deterministic for the same source tree and pinned toolchain. This is a build contract,
not a reproducible-build claim; make that claim only after comparing two clean builds in isolated
environments.

## Standalone development lab

```sh
cargo make lab
```

Open `http://127.0.0.1:4174/lab.html`. The command builds the current WASM bindings and serves only
Grafik's `web/` directory. Port 4174 leaves common application-preview ports free so a consumer can
run in parallel. The lab uses simulated inputs and does not require, read, or modify another
checkout. Use its seed and recipe URL parameters for exact local replay.

Do not point this command at another repository, add a cross-repository symlink, or automate copying
into a sibling checkout. A downstream integration should select a Grafik revision or attributable
artifact explicitly and make its own reviewed repository change.

## Browser tracer

Build bindings before serving the simulated outcome tracer:

```sh
cargo make wasm
cargo make serve
```

Open `http://127.0.0.1:4174`. `serve` runs
`python3 -m http.server 4174 --bind 127.0.0.1 --directory web` and remains in the foreground until
interrupted. No generated binding is committed; rerun `cargo make wasm` after Rust changes.

## Authoritative inputs

`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `Makefile.toml`, and `scripts/ci-local.sh` are the
executable inputs. Update this guide in the same change whenever their public commands change.
