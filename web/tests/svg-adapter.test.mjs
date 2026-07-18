import assert from "node:assert/strict";
import test from "node:test";

import { renderTrace, segmentAttributes } from "../svg-adapter.js";

const trace = {
  seed: 42,
  result_source: "recorded",
  final_disposition: "SUCCEEDED",
  events: [
    {
      kind: "segment_grew",
      index: 0,
      weight: 2,
      at_ms: 0,
      duration_ms: 120,
      from: { x: 10, y: 20 },
      to: { x: 10, y: 40 },
    },
    {
      kind: "segment_grew",
      index: 1,
      weight: 1,
      at_ms: 120,
      duration_ms: 140,
      from: { x: 10, y: 40 },
      to: { x: 30, y: 40 },
    },
    { kind: "segment_retracted", index: 1, at_ms: 260, duration_ms: 90 },
    { kind: "segment_retracted", index: 0, at_ms: 350, duration_ms: 90 },
    { kind: "connector_finished", at_ms: 440 },
  ],
};

const fakeLine = () => ({
  attributes: new Map(),
  classes: new Set(),
  style: { setProperty() {} },
  setAttribute(name, value) {
    this.attributes.set(name, value);
  },
  classList: {
    add() {},
  },
  remove() {
    this.removed = true;
  },
});

const fakeGroup = () => ({
  children: [],
  append(line) {
    this.children.push(line);
  },
  replaceChildren() {
    this.children = [];
  },
});

test("segmentAttributes projects renderer-neutral coordinates", () => {
  assert.deepEqual(segmentAttributes(trace.events[0]), {
    x1: "10",
    y1: "20",
    x2: "10",
    y2: "40",
    pathLength: "1",
    "data-progress-weight": "2",
  });
});

test("reduced motion applies the final state without scheduling", () => {
  const group = fakeGroup();
  let scheduled = false;
  let announcement = "";

  renderTrace(trace, {
    group,
    reducedMotion: true,
    schedule: () => {
      scheduled = true;
    },
    announce: (message) => {
      announcement = message;
    },
  });

  assert.equal(scheduled, false);
  assert.deepEqual(group.children, []);
  assert.equal(announcement, "Recorded SUCCEEDED result applied. Motion reduced.");
});

test("animated traversal schedules growth and leaf-first removal", () => {
  const group = fakeGroup();
  const jobs = [];
  let announcement = "";
  renderTrace(trace, {
    group,
    reducedMotion: false,
    createLine: fakeLine,
    schedule: (action, at) => {
      jobs.push({ action, at });
      return jobs.length;
    },
    announce: (message) => {
      announcement = message;
    },
  });

  const initialJobs = jobs.slice().sort((left, right) => left.at - right.at);
  for (const job of initialJobs) job.action();
  const removalJobs = jobs.slice(initialJobs.length);
  for (const job of removalJobs) job.action();

  assert.deepEqual(
    initialJobs.map((job) => job.at),
    [0, 120, 260, 350, 440],
  );
  assert.equal(group.children.length, 2);
  assert.equal(group.children.every((line) => line.removed), true);
  assert.equal(announcement, "Recorded SUCCEEDED result complete.");
});
