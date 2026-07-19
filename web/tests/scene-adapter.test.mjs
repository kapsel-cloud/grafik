import assert from "node:assert/strict";
import test from "node:test";

import { measuredEdgeEndpoints } from "../scene-adapter.js";

test("diagram edges derive from measured node bounds with a three-pixel overlap", () => {
  const origin = { left: 10, top: 20 };
  const source = { left: 30, top: 50, width: 100, height: 40 };
  const target = { left: 210, top: 130, width: 80, height: 60 };

  assert.deepEqual(measuredEdgeEndpoints(source, target, origin), {
    from: { x: 117, y: 50 },
    to: { x: 203, y: 140 },
  });
});

test("vertical diagram edges use measured top and bottom boundaries", () => {
  const origin = { left: 0, top: 0 };
  const source = { left: 40, top: 20, width: 80, height: 60 };
  const target = { left: 50, top: 180, width: 100, height: 40 };

  assert.deepEqual(measuredEdgeEndpoints(source, target, origin), {
    from: { x: 80, y: 77 },
    to: { x: 100, y: 183 },
  });
});
