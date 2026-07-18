# Documentation index

- **Status:** Active router
- **Kind:** Normative authority index
- **Authority:** Canonical for question-to-owner routing and contradiction order
- **Owns:** Where each project question is answered
- **Does not own:** The answers themselves

| Question | Owner |
| --- | --- |
| What is Grafik and what is out of scope? | [`SCOPE.md`](SCOPE.md) |
| Which module owns simulation and which direction may dependencies flow? | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| What are panels, ports, fields, patterns, and spatial budgets? | [`PATTERNS.md`](PATTERNS.md) |
| What are seeds, time, events, replay, and determinism? | [`SIMULATION.md`](SIMULATION.md) |
| What does the browser measure and render? | [`RENDERING.md`](RENDERING.md) |
| Which commands and prerequisites are real? | [`BUILD.md`](BUILD.md) |
| What evidence is required? | [`TESTING.md`](TESTING.md) |
| How is Rust and documentation written? | [`STYLE.md`](STYLE.md) |
| How is work reviewed and reported? | [`REVIEW.md`](REVIEW.md) |
| Why was a durable choice made? | [`decisions/README.md`](decisions/README.md) |
| Where does unfinished technical work live? | [`../tasks/README.md`](../tasks/README.md) |

## Authority order

Resolve contradictions in this order:

1. `SCOPE.md` plus the active contract for the changed surface
2. The direct owner named above
3. Conforming implementation and tests
4. An active task document
5. Accepted ADRs, which preserve rationale but do not redefine current behavior

Correct the owner rather than adding a duplicate summary.
