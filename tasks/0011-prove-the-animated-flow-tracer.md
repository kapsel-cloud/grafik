# Prove the animated flow tracer

- **Status:** Ready
- **Blocked by:** None
- **Direct owners:** [`../docs/SCOPE.md`](../docs/SCOPE.md),
  [`../docs/PATTERNS.md`](../docs/PATTERNS.md), [`../docs/SIMULATION.md`](../docs/SIMULATION.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md), [`../docs/TESTING.md`](../docs/TESTING.md)

## Result

The standalone Grafik lab presents one deterministic simulated external-effect flow as a readable
node-and-edge diagram. Directed connectors visibly traverse the declared topology. A disconnected
edge emits a bounded spark at its break, a successful terminal emits a decorative thumbs-up, and a
failed terminal applies a bounded glitch behind the explicit `FAILED` label. `UNKNOWN` remains a
separate explicit outcome.

This tracer answers whether Grafik can explain a logical path elegantly. It does not integrate with
a live Kapsel process, parse a Kapsel receipt, or turn the lab into an operations dashboard.

## First fixture

```text
grant -> journal -> provider seam -> observe -> receipt
```

The fixture is deterministic, simulated, and owned by Grafik. It may use the published Kapsel
lifecycle vocabulary as display content, but no Kapsel checkout, package, schema, credential, or
runtime service becomes an input.

## Interaction

- Outcome controls select `SUCCEEDED`, `FAILED`, or `UNKNOWN` without changing topology.
- Replay animates arrow traversal in declared edge order.
- One disconnect control breaks a declared edge and renders the break explicitly rather than
  implying that the downstream path completed.
- Decorative effects never replace the formal outcome label or move readable nodes.
- Existing plan and trace inspectors expose the complete simulated inputs and generated events.
- Reduced motion applies the complete readable final state without traversal, sparks, pulsing, or
  glitch timers.

## Acceptance

- Every connector has a visible direction and references existing source and target node IDs.
- Arrow motion follows measured browser geometry and never crosses node interiors except at ports.
- A disconnect stops traversal at the selected edge and emits bounded decorative sparks at the
  break.
- `SUCCEEDED` retains its text label and may show an `aria-hidden` thumbs-up at the terminal.
- `FAILED` retains its text label and applies glitch only to a decorative backing layer.
- `UNKNOWN` retains its text label and receives no success or failure cue.
- Keyboard controls can select outcomes, disconnect an edge, and replay the same fixture.
- The same fixture, seed, geometry, and controls produce the same complete trace.
- The lab remains readable at 320 CSS px, without JavaScript, and under reduced motion.
- Grafik's default gate remains independent of Kapsel and any live service.

## Deferred surfaces

No batch runner, log store, filtering system, code-integration catalog, live receipt transport, or
Kapsel `0.1.0` adapter enters this packet. Those require separate evidence after the flow tracer
passes visual and semantic review.

## Narrow proof

```sh
cargo make test
cargo make test-web
cargo make check
```

## Residual risk

A polished simulated tracer proves explanation and rendering behavior only. It does not prove that
operators need a batch explorer, that logs share a stable schema, or that live Kapsel integration is
safe or useful.
