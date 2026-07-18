import assert from "node:assert/strict";
import test from "node:test";

import { renderTrace, segmentAttributes } from "../svg-adapter.js";

const trace = {
  seed: 42,
  result_source: "recorded",
  final_disposition: "SUCCEEDED",
  events: [
    { kind: "outcome_started", at_ms: 0 },
    { kind: "connector_started", at_ms: 0, from: { x: 10, y: 20 }, to: { x: 30, y: 40 } },
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
    { kind: "success_pulsed", at_ms: 260, duration_ms: 360, intensity: 2 },
    { kind: "segment_retracted", index: 1, at_ms: 620, duration_ms: 90 },
    { kind: "segment_retracted", index: 0, at_ms: 710, duration_ms: 90 },
    { kind: "connector_finished", at_ms: 800 },
    { kind: "outcome_finished", at_ms: 800 },
  ],
};

const fakeLine = () => {
  const classes = new Set();
  return {
  attributes: new Map(),
  classes,
  style: { setProperty() {} },
  setAttribute(name, value) {
    this.attributes.set(name, value);
  },
  classList: {
    add(...names) {
      for (const name of names) classes.add(name);
    },
    remove(...names) {
      for (const name of names) classes.delete(name);
    },
  },
  remove() {
    this.removed = true;
  },
  };
};

const fakeGroup = () => ({
  children: [],
  append(line) {
    this.children.push(line);
  },
  replaceChildren() {
    this.children = [];
  },
});

const fakeOutcomeLayer = () => {
  const classes = new Set();
  const properties = new Map();
  return {
    children: [],
    classes,
    properties,
    append(child) {
      this.children.push(child);
    },
    replaceChildren() {
      this.children = [];
    },
    classList: {
      add(...names) {
        for (const name of names) classes.add(name);
      },
      remove(...names) {
        for (const name of names) classes.delete(name);
      },
    },
    style: {
      setProperty(name, value) {
        properties.set(name, value);
      },
    },
  };
};

test("segmentAttributes projects renderer-neutral coordinates", () => {
  const segment = trace.events.find((event) => event.kind === "segment_grew");
  assert.deepEqual(segmentAttributes(segment), {
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
  const outcomeLayer = fakeOutcomeLayer();
  let scheduled = false;
  let announcement = "";

  renderTrace(trace, {
    group,
    outcomeLayer,
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
  const outcomeLayer = fakeOutcomeLayer();
  const jobs = [];
  let announcement = "";
  renderTrace(trace, {
    group,
    outcomeLayer,
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
  const pulsed = group.children.every((line) => line.classes.has("is-success-pulsing"));
  const removalJobs = jobs.slice(initialJobs.length);
  for (const job of removalJobs) job.action();

  assert.deepEqual(
    initialJobs.map((job) => job.at),
    [0, 120, 260, 620, 710, 800],
  );
  assert.equal(group.children.length, 2);
  assert.equal(group.children.every((line) => line.removed), true);
  assert.equal(pulsed, true);
  assert.equal(announcement, "Recorded SUCCEEDED result complete.");
});

test("failed glitches only the decorative outcome layer", () => {
  const group = fakeGroup();
  const outcomeLayer = fakeOutcomeLayer();
  const jobs = [];
  let announcement = "";
  renderTrace(
    {
      seed: 9,
      result_source: "simulated",
      final_disposition: "FAILED",
      events: [
        { kind: "outcome_started", at_ms: 0 },
        {
          kind: "failure_glitched",
          at_ms: 0,
          duration_ms: 180,
          offset_x: -2,
          offset_y: 3,
          strips: 2,
        },
        { kind: "outcome_finished", at_ms: 180 },
      ],
    },
    {
      group,
      outcomeLayer,
      reducedMotion: false,
      schedule: (action, at) => {
        jobs.push({ action, at });
        return jobs.length;
      },
      announce: (message) => {
        announcement = message;
      },
    },
  );

  const initialJobs = jobs.slice();
  for (const job of initialJobs) job.action();
  assert.equal(outcomeLayer.classes.has("is-failure-glitching"), true);
  assert.equal(outcomeLayer.properties.get("--glitch-x"), "-2px");
  assert.equal(outcomeLayer.properties.get("--glitch-y"), "3px");
  assert.equal(outcomeLayer.properties.get("--glitch-strips"), "2");
  assert.equal(announcement, "Simulated FAILED result complete.");

  for (const job of jobs.slice(initialJobs.length)) job.action();
  assert.equal(outcomeLayer.classes.has("is-failure-glitching"), false);
});

test("unknown places bounded colored question marks behind the outcome", () => {
  const group = fakeGroup();
  const outcomeLayer = fakeOutcomeLayer();
  const jobs = [];
  const marks = [];
  const createMark = () => {
    const properties = new Map();
    const mark = {
      dataset: {},
      properties,
      style: {
        setProperty(name, value) {
          properties.set(name, value);
        },
      },
      classList: { add() {} },
      remove() {
        this.removed = true;
      },
    };
    marks.push(mark);
    return mark;
  };

  renderTrace(
    {
      seed: 12,
      result_source: "simulated",
      final_disposition: "UNKNOWN",
      events: [
        { kind: "outcome_started", at_ms: 0 },
        {
          kind: "question_mark_appeared",
          index: 0,
          at_ms: 0,
          lifetime_ms: 600,
          point: { x: 110, y: 220 },
          tone: "accent",
        },
        {
          kind: "question_mark_appeared",
          index: 1,
          at_ms: 500,
          lifetime_ms: 700,
          point: { x: 130, y: 240 },
          tone: "info",
        },
        { kind: "outcome_finished", at_ms: 1200 },
      ],
    },
    {
      group,
      outcomeLayer,
      outcomeOrigin: { x: 100, y: 200 },
      reducedMotion: false,
      createMark,
      schedule: (action, at) => {
        jobs.push({ action, at });
        return jobs.length;
      },
    },
  );

  const initialJobs = jobs.slice();
  for (const job of initialJobs) job.action();
  assert.equal(outcomeLayer.children.length, 2);
  assert.deepEqual(marks.map((mark) => mark.textContent), ["?", "?"]);
  assert.deepEqual(marks.map((mark) => mark.dataset.tone), ["accent", "info"]);
  assert.equal(marks[0].properties.get("--mark-x"), "10px");
  assert.equal(marks[0].properties.get("--mark-y"), "20px");

  for (const job of jobs.slice(initialJobs.length)) job.action();
  assert.equal(marks.every((mark) => mark.removed), true);
});
