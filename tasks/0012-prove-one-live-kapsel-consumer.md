# Prove one live Kapsel consumer

- **Status:** Queued
- **Blocked by:** [`0011-prove-the-animated-flow-tracer.md`](0011-prove-the-animated-flow-tracer.md),
  Kapsel's published sandbox event/receipt contract, and the `kapsel.cloud` live run-and-receipt
  packet
- **Direct owners:** [`../docs/SCOPE.md`](../docs/SCOPE.md),
  [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md), [`../docs/TESTING.md`](../docs/TESTING.md), and
  [`../docs/BUILD.md`](../docs/BUILD.md)
- **Consumer owner:**
  [`kapsel.cloud` live run-and-receipt packet](https://github.com/kapsel-cloud/kapsel.cloud/blob/master/tasks/0002-prove-the-live-run-and-receipt.md)

## Result

One committed `kapsel.cloud` consumer maps Kapsel's bounded public sandbox projection into Grafik's
renderer-neutral scene and trace vocabulary and presents the real run lifecycle without giving
Grafik product authority.

Grafik remains independently buildable and useful with simulated inputs. The consumer owns the
Kapsel protocol client, product semantics, mapping, DOM/SVG/CSS, browser measurement, accessibility,
themes, reconnect behavior, and textual receipt. Grafik owns only deterministic renderer-neutral
scene and trace generation from bounded consumer-provided semantic input.

## Boundary

```text
Kapsel public event projection and receipt
  -> kapsel.cloud versioned client and semantic adapter
  -> Grafik generic scene / trace input
  -> Grafik renderer-neutral output
  -> kapsel.cloud DOM, SVG, text, accessibility, and fallback
```

No Kapsel event, receipt, API, URL, operation, result, credential, or infrastructure type enters
Grafik's public Rust model. No Grafik code opens a network connection or classifies a Kapsel result.

## Work

1. Finish and accept the standalone animated flow tracer before examining the live consumer.
2. Pin one exact Grafik source revision and produce an attributable browser artifact with checksum.
3. Integrate the artifact through a committed `kapsel.cloud` change; neither repository's default
   build may read or write the sibling checkout.
4. Attempt the live mapping through existing scene and trace interfaces first. Open one finite Grafik
   correction only when the concrete consumer proves an interface gap that cannot remain
   consumer-owned.
5. Map admission, durable attempt, provider seam, process termination/recovery, receiver observation,
   classification, publication, and cleanup into generic nodes, edges, progress, disconnect, and
   outcome cues without changing Kapsel meaning.
6. Preserve the textual live timeline and receipt when Grafik fails, JavaScript is disabled, reduced
   motion is requested, the stream reconnects, an unknown event version arrives, or geometry changes.
7. Prove deterministic replay of one captured public event projection separately from the live
   stream. The capture must contain no credentials, private identifiers, uncontrolled logs, or
   unnecessary visitor data.
8. Record exact committed revisions, artifact checksums, commands, browser evidence, integration
   friction, and one explicit outcome in both owning repositories.

## Acceptance

- Grafik contains no Kapsel-specific parser, schema, status enum, receipt type, endpoint, styling,
  copy, or network client.
- The consumer adapter is the only place that knows both the Kapsel projection and Grafik input.
- Live progress is never animation-derived; reload and reconnect reconstruct it from Kapsel's ordered
  public projection.
- `SUCCEEDED`, `FAILED`, and `UNKNOWN` remain visible textual Kapsel outcomes. Decorative cues cannot
  replace, infer, or contradict them.
- Cleanup remains visually and semantically distinct from operation classification.
- Reduced motion schedules no traversal, spark, pulse, or glitch while preserving the complete
  readable final state.
- Missing or failed Grafik assets leave the live textual timeline, receipt, local self-serve action,
  and security path usable.
- The same captured public projection, adapter version, seed, and geometry produce the same complete
  renderer-neutral trace.
- Grafik and consumer gates pass independently from exact clean revisions; the consumer verifies the
  pinned artifact and attribution.

## Non-goals

No generic log explorer, query system, batch view, operator console, Kapsel SDK, live network client
inside Grafik, receipt verification authority, stable cross-product protocol, source-level sibling
dependency, or second consumer abstraction.

## Narrow proof

```sh
cargo make artifact
cargo make test-artifact
cargo make check
```

Run the consumer repository's complete gate and focused live-browser matrix separately.

## Residual risk

One real consumer can prove a useful technical boundary but not broad compatibility, demand for
Grafik, or the value of a larger dashboard. Streaming and reconnect behavior may expose a genuine
incremental-scene gap; correct only the smallest repeated interface needed by the committed consumer.
