import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const mainUrl = new URL("../main.js", import.meta.url);

test("simulated tracer imports the generated WASM interface it calls", async () => {
  const source = await readFile(mainUrl, "utf8");

  assert.match(source, /import init, \{ grafik_trace \} from "\.\/pkg\/grafik\.js";/u);
  assert.match(source, /await Promise\.all\(\[init\(\)/u);
  assert.match(source, /grafik_trace\(JSON\.stringify\(input\)\)/u);
  assert.match(source, /result_source: "simulated"/u);
  assert.doesNotMatch(source, /fetch\s*\(/u);
  assert.doesNotMatch(source, /Math\.random/u);
});
