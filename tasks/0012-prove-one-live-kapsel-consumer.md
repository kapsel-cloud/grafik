# Prove one live Kapsel consumer

- **Status:** In progress — fixture-consumer acceptance complete; live acceptance blocked
- **Fixture acceptance revisions:** Kapsel contract
  `3081b08ad4e9dee8d6db6007d84dfd911997c16e`, Grafik artifact
  `899a429bb6b32d23749ecc6d45c1c70067a3de7d`, and published `kapsel.cloud` fixture implementation
  `2c87f44a73898626b3217ecb7fc1d095fceb6128`
- **Matching website evidence owner:** the `kapsel.cloud` live run-and-receipt packet; only the
  website owner records the final Grafik acceptance revision
- **Live acceptance blocked by:** Kapsel's approved KAP-0053 exact sandbox deployment and a committed
  `kapsel.cloud` connection to that exact deployment
- **Direct owners:** [`../docs/SCOPE.md`](../docs/SCOPE.md),
  [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`../docs/RENDERING.md`](../docs/RENDERING.md), [`../docs/TESTING.md`](../docs/TESTING.md), and
  [`../docs/BUILD.md`](../docs/BUILD.md)
- **Consumer owner:**
  [`kapsel.cloud` live run-and-receipt packet](https://github.com/kapsel-cloud/kapsel.cloud/blob/master/tasks/0002-prove-the-live-run-and-receipt.md)

## Post-KAP-0053 result target

After KAP-0053 approves one exact sandbox deployment and the website commits and verifies its exact
connection, one `kapsel.cloud` consumer will map Kapsel's bounded public sandbox projection into
Grafik's renderer-neutral scene and trace vocabulary and will present the real run lifecycle without
giving Grafik product authority. The accepted fixture consumer does not satisfy this live target.

Grafik will remain independently buildable and useful with simulated inputs. The consumer will own
the Kapsel protocol client, product semantics, mapping, DOM/SVG/CSS, browser measurement,
accessibility, themes, reconnect behavior, and textual receipt. Grafik will own only deterministic
renderer-neutral scene and trace generation from bounded consumer-provided semantic input.

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

## Fixture-consumer acceptance evidence

Accepted on 2026-07-22 against public clean producer and fixture-implementation revisions. Matching
website evidence remains owned by the `kapsel.cloud` live run-and-receipt packet; its owner, not
Grafik, records the final Grafik acceptance revision. The fixture consumer at
`2c87f44a73898626b3217ecb7fc1d095fceb6128` vendors only the generated Grafik JS/WASM, license,
manifest, and checksums under its own repository. Its manifest attributes the clean Grafik source
revision `899a429bb6b32d23749ecc6d45c1c70067a3de7d`; the consumed JS and WASM SHA-256 values are
`54bffa6488debcf555f37a8a015b5bc926d8f5922f6ff630441bba872e44784a` and
`359dd0b2d053eaefac0552c2efab84ba37ea1d4a4c990211ff4cb9d55de63a30`. A clean local rebuild of the
pinned Grafik revision reproduced both consumed hashes after clearing the pre-existing WASM target
cache.

Inspection and focused tests show that `public/run/lib/grafik-adapter.js` is the only consumer file
that knows both the Kapsel projection and Grafik input. It maps a terminal fixture projection into
`animated_flow`, explicit seed `51`, generic bounded display content, `recorded` provenance, an
already-authoritative disposition, and browser-measured geometry. The adapter owns DOM/SVG/CSS,
measurement, reduced motion, and trace scheduling. The surrounding website retains strict schema
validation, fixture transport, ordered textual timeline, receipt bytes and download, cleanup text,
replay identity, copy, accessibility, and failure fallback. No renderer-neutral Grafik correction
was required.

A separate replay proof loaded the committed `healthy` KAP-0051 projection through that adapter and
called the vendored `grafik_scene` and `grafik_scene_trace` interfaces twice with the same request,
seed, explicit boundary geometry, and recorded trigger. Both complete scene JSON values were
byte-identical; both complete trace JSON values were byte-identical. The trace contained seven
ordered events, traversed all four declared edges, and preserved `recorded` plus `SUCCEEDED` in its
trigger. The serialized Grafik request contained none of `run_id`, `operation_id`, `api_version`,
`receiver_result`, or `cleanup_state`.

Validation from the exact consumer revision:

- `npm run test:run`: passed 11 tests, including strict fixture validation, ordered replay,
  consumer-owned generic mapping, artifact attribution/checksums, and absence of endpoint or sibling
  checkout configuration.
- `npm run test:run:browser`: passed the complete fixture browser matrix, including both terminal
  outcomes, session replay, reduced motion, forced Grafik failure, no JavaScript, keyboard use,
  320 CSS px, and no console/page errors.
- `npm run check`: passed formatting, HTML, links, all unit tests, and the deployment dry run with no
  bindings.
- Grafik `cargo make artifact`, `cargo make test-artifact`, and `cargo make check`: passed; the
  complete gate included 14 deterministic native tests and eight adapter tests.

This accepts only the committed fixture-consumer slice. It does not accept live transport,
reconnect against a deployed service, public receipt retrieval, deployment, or public traffic.
Those remain blocked until KAP-0053 approves one exact sandbox deployment and `kapsel.cloud` commits
and verifies the exact live connection.

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

Run the consumer repository's complete gate and focused fixture-browser matrix separately; reserve
live-browser evidence for the post-KAP-0053 connection.

## Residual risk

The accepted evidence uses committed contract fixtures and one Chromium browser matrix, not a live
sandbox. KAP-0053 remains queued behind KAP-0052, and no exact website live connection, deployment,
rollback, public receipt retrieval, or public-traffic evidence exists. Streaming and reconnect may
still expose a genuine incremental-scene gap; correct only the smallest repeated interface proved by
the eventual committed live consumer. One consumer does not prove broad compatibility, demand for
Grafik, or the value of a larger dashboard.
