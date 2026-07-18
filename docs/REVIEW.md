# Review

- **Status:** Active review guide
- **Kind:** Normative review owner
- **Authority:** Canonical for review order, commit subjects, and result shape
- **Owns:** How changes are inspected and reported
- **Does not own:** Contract content, command syntax, or task priority

## Commit subjects

Use `<domain>: <imperative result>`, for example `simulation: emit replayable connector trace`.
Do not commit or push unless the user explicitly requests it.

## Review loop

1. Name the changed contract and its direct owner from `INDEX.md`.
2. Inspect the narrowest meaningful diff and run its narrow proof.
3. Check dependency direction and ensure Rust has no DOM knowledge.
4. Check deterministic inputs, event ordering, geometry exclusion, and bounded budgets.
5. Check owner documents for contradiction, duplicated truth, and commands that do not exist.
6. Run `cargo make check`.
7. Manually inspect the browser when rendering changed; include reduced-motion and no-JavaScript
   behavior.
8. State what remains unproved even when no finding remains.

Findings lead the review and include file/line evidence, impact, and the smallest corrective action.
Do not claim visual, browser, release, or production proof from native tests alone.

## Result shape

```text
Contract:
Owner:
Surface:
Gate:
Good:
Findings:
Risk:
Next action:
```

A clean review still says which surfaces were checked and names residual risk.
