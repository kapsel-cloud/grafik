import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const fixtureUrl = new URL("../fixtures/kapsel-recorded-success.json", import.meta.url);

const loadFixture = async () => JSON.parse(await readFile(fixtureUrl, "utf8"));

test("recorded fixture exposes one bounded publishable result", async () => {
  const fixture = await loadFixture();

  assert.deepEqual(Object.keys(fixture).sort(), [
    "classification",
    "non_claims",
    "provenance",
    "result",
  ]);
  assert.deepEqual(fixture.classification, {
    recorded: true,
    simulated_presentation: true,
    production_ready: false,
  });
  assert.deepEqual(fixture.result, {
    result_source: "recorded",
    final_disposition: "SUCCEEDED",
  });
  assert.match(fixture.provenance.source_revision, /^[0-9a-f]{40}$/);
  assert.equal(fixture.provenance.command, "cargo make demo-kind");
  assert.equal(fixture.provenance.operation_id, "demo-healthy-op");
  assert.deepEqual(fixture.non_claims, [
    "no-exactly-once",
    "no-causation",
    "no-kubernetes-truth",
    "no-complete-capture",
    "no-witnessing",
    "not-production",
  ]);

  const serialized = JSON.stringify(fixture);
  for (const forbidden of [
    "/Users/",
    "kubeconfig",
    "journal",
    "receipt_file",
    "receipt_sha256",
    "signing",
    "trust",
  ]) {
    assert.equal(serialized.includes(forbidden), false, forbidden);
  }
});
