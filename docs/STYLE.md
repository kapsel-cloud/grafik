# Style

- **Status:** Active engineering guide
- **Kind:** Normative Rust and documentation owner
- **Authority:** Canonical for source discipline, naming, errors, and dependency policy
- **Owns:** How project code and owner documents are written
- **Does not own:** Product scope, architecture, commands, or test coverage

## Rust

- Edition 2021; minimum Rust 1.97; toolchain patch pinned by `rust-toolchain.toml`.
- `unsafe_code = "forbid"`; missing public docs and unreachable public items are denied.
- Clippy `all`, `pedantic`, `nursery`, and `cargo` run with warnings denied in the gate.
- Product code must not use `unwrap`, `expect`, `panic`, `unimplemented`, `todo`, `dbg`, stdout, or
  stderr printing. Tests may return `Result` and use assertions rather than bypassing these rules.
- Keep physical Rust source lines at or below 100 bytes.
- Prefer domain names: `Panel`, `EdgePort`, `Trace`, and `SpatialEvent`. Do not hide domain behavior
  in generic helpers.
- Public fallible functions document `# Errors`. Public examples are doctested.
- Return values instead of side effects. Accept seed, geometry, and time inputs rather than creating
  ambient dependencies.
- Validate finite numeric input before arithmetic. Use integer milliseconds for simulation time.

## Modules and interfaces

A module earns its place through depth: leverage for callers and locality for maintainers. The
interface includes invariants and error modes, not only Rust signatures. Apply the deletion test.
The interface is the test surface. Two adapters are required before introducing a general seam.

## Dependencies

Dependencies are design decisions. Pin direct dependency patch versions exactly, disable default
features when practical, and commit `Cargo.lock`. Prefer a small maintained library over bespoke
serialization, but keep deterministic randomness local and documented. Recheck registry versions
and official docs before upgrades.

## Documentation

Owner documents begin with Status, Kind, Authority, Owns, and Does not own. They state one canonical
truth and link instead of duplicating it. ADRs preserve rationale; tasks preserve unfinished work.
Use 100-column prose where practical, fenced language labels, relative links, and explicit
non-claims. Guides only advertise commands that exist.
