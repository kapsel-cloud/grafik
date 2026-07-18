# Grafik contributor router

Grafik is a public, contract-first Rust/WASM project. Name the contract you are changing before
editing implementation.

## Start here

1. Read `docs/SCOPE.md` for project purpose and non-goals.
2. Use `docs/INDEX.md` to find the direct owner of a claim.
3. Read the active task in `tasks/` when one exists.
4. Read accepted decisions in `docs/decisions/` for rationale, not current behavior.

## Correction protocol

When sources conflict, stop the conflicting edit. Compare `docs/SCOPE.md` with the direct owner
listed by `docs/INDEX.md`. Update the canonical owner before implementation. Do not average
incompatible designs or add a shadow summary.

## Engineering rules

- Keep one product package until a second real consumer proves another package seam.
- Rust/WASM owns simulation. Browser adapters own geometry, rendering, accessibility, and motion
  preferences. Rust must not manipulate the DOM.
- Use explicit seeds and controlled time. Product code must not use ambient randomness.
- The interface is the test surface. Apply the deletion test before extracting a module.
- One adapter is a hypothetical seam; two adapters make a seam real.
- Do not add generic `core`, `common`, `utils`, `engine`, or plugin packages.
- Product Rust forbids unsafe code, unwrap, expect, panic, todo, dbg, and print output.
- Commands in guides must exist. Run the narrowest proof first, then `cargo make check`.

## Work report

Use this compact result for meaningful work:

```text
Contract:
Surface:
Gate:
Risk:
```

Do not commit or push unless the user explicitly requests it.
