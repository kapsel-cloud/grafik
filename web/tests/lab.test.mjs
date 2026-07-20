import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const html = await readFile(new URL("../lab.html", import.meta.url), "utf8");
const index = await readFile(new URL("../index.html", import.meta.url), "utf8");
const script = await readFile(new URL("../lab.js", import.meta.url), "utf8");
const adapter = await readFile(new URL("../scene-adapter.js", import.meta.url), "utf8");

test("standalone flow lab uses only local Grafik scene interfaces", () => {
  for (const entry of [html, index]) assert.match(entry, /src="\.\/lab\.js"/u);
  assert.match(script, /grafik_scene, grafik_scene_trace/u);
  assert.match(script, /from "\.\/pkg\/grafik\.js"/u);
  assert.doesNotMatch(script, /grafik_trace/u);
  for (const source of [script, adapter]) {
    assert.doesNotMatch(source, /fetch\s*\(/u);
    assert.doesNotMatch(source, /Math\.random/u);
  }
});

test("lab pins the curated animated-flow fixture and complete replay state", () => {
  assert.match(script, /recipe: "animated_flow"/u);
  assert.match(script, /\["grant", "journal", "provider seam", "observe", "receipt"\]/u);
  assert.match(script, /result_source: "simulated"/u);
  assert.match(script, /final_disposition: selectedOutcome/u);
  assert.match(script, /disconnected_edge: disconnectedEdge/u);
  assert.match(script, /measureScene\(view\)/u);
  assert.match(html, /data-scene-json/u);
  assert.match(html, /data-trace-json/u);
});

test("native controls and no-JavaScript fallback preserve explicit meaning", () => {
  for (const outcome of ["SUCCEEDED", "FAILED", "UNKNOWN"]) {
    assert.match(html, new RegExp(`data-outcome="${outcome}"`, "u"));
  }
  assert.match(html, /data-disconnect-controls/u);
  assert.match(html, /data-replay/u);
  assert.match(html, /role="group"/u);
  assert.match(html, /aria-pressed/u);
  assert.match(html, /grant → journal → provider seam → observe → receipt/u);
  assert.match(html, /<noscript>/u);
});
