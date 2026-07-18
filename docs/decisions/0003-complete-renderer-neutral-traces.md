# 0003: Return complete renderer-neutral traces

- **Status:** Accepted
- **Date:** 2026-07-18
- **Current owners:** [`../SIMULATION.md`](../SIMULATION.md),
  [`../RENDERING.md`](../RENDERING.md)

## Context

The tracer needs deterministic replay, animated browser traversal, and an immediate reduced-motion
state. A callback-driven WASM loop would bind simulation progress to browser scheduling and make
replay evidence harder to compare.

## Decision

One simulation call returns the complete ordered spatial trace with integer simulation timestamps.
Events contain geometry and timing but no rendering selectors or commands. Browser adapters choose
whether to schedule traversal or consume the final state immediately.

## Consequences

Complete traces are easy to compare and serialize. They use memory proportional to event count;
explicit density and lifetime budgets bound that cost. Long-running future diagrams may need a
separate streaming decision rather than silently changing this interface.

## Rejected

- Browser callbacks from Rust: couples event generation to wall time.
- Final geometry only: cannot describe growth and leaf-first retraction.
