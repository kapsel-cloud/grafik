# Contributing to Grafik

Grafik is contract-first experimental Rust/WASM software. Small, reviewable vertical tracers are
preferred over speculative abstractions.

## Before changing code

1. Read [`docs/SCOPE.md`](docs/SCOPE.md) for purpose and non-goals.
2. Use [`docs/INDEX.md`](docs/INDEX.md) to find the canonical owner of the behavior.
3. Read the active packet in [`tasks/`](tasks/) when one exists.
4. Update the canonical owner before implementation when the contract changes.

Do not add generic plugin systems, browser DOM access from Rust, ambient randomness, live
infrastructure access, credentials, private paths, or product receipt bytes.

## Proving a change

Follow [`docs/BUILD.md`](docs/BUILD.md) for prerequisites and real commands. Run the narrowest proof
while iterating, then the complete local gate:

```sh
cargo make check
```

Tests exercise public interfaces with explicit seeds, controlled time, bounded geometry, and exact
replay. A passing gate does not replace manual browser, accessibility, or visual evidence when the
rendered surface changes.

## Reporting work

Use the compact report defined by [`docs/REVIEW.md`](docs/REVIEW.md):

```text
Contract:
Surface:
Gate:
Risk:
```

By contributing, you agree that your contribution is licensed under the repository's Apache-2.0
license.
