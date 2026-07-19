import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const html = await readFile(new URL("../lab.html", import.meta.url), "utf8");
const script = await readFile(new URL("../lab.js", import.meta.url), "utf8");
const adapter = await readFile(new URL("../scene-adapter.js", import.meta.url), "utf8");

test("standalone lab uses only local Grafik assets", () => {
  assert.match(html, /src="\.\/lab\.js"/u);
  assert.match(script, /grafik_scene, grafik_scene_trace/u);
  assert.match(script, /from "\.\/pkg\/grafik\.js"/u);
  assert.match(script, /from "\.\/scene-adapter\.js"/u);
  for (const source of [script, adapter]) {
    assert.doesNotMatch(source, /fetch\s*\(/u);
    assert.doesNotMatch(source, /Math\.random/u);
    assert.doesNotMatch(source, /kapsel\.cloud/u);
  }
});

test("standalone lab exposes deterministic grid review state", () => {
  assert.match(html, /data-gallery/u);
  assert.match(html, /data-seed/u);
  assert.match(html, /data-recipe/u);
  assert.match(html, /data-replay-selected/u);
  assert.match(html, /data-scene-json/u);
  assert.match(html, /data-trace-json/u);
  assert.match(script, /offset < 6/u);
  assert.match(script, /history\.replaceState/u);
  assert.match(script, /measureScene\(candidate\.view\)/u);
});

test("concrete adapter renders every receipt primitive with native elements", () => {
  for (const renderer of [
    "renderHeader",
    "renderFacts",
    "renderList",
    "renderTable",
    "renderDiagram",
    "renderResult",
    "renderActions",
    "renderMenu",
  ]) {
    assert.match(adapter, new RegExp(`const ${renderer}`, "u"));
  }
  assert.match(adapter, /element\("table"/u);
  assert.match(adapter, /element\("button"/u);
  assert.match(adapter, /element\("details"/u);
  assert.match(adapter, /dataset\.sceneNode/u);
  assert.match(adapter, /getBoundingClientRect/u);
});
