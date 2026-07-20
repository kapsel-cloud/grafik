import assert from "node:assert/strict";
import test from "node:test";

import {
  drawDiagram,
  measuredEdgeEndpoints,
  renderSceneTrace,
} from "../scene-adapter.js";

class FakeElement {
  constructor(name) {
    this.name = name;
    this.attributes = new Map();
    this.children = [];
    this.dataset = {};
    this.properties = new Map();
    this.classes = new Set();
    this.classList = {
      add: (...names) => names.forEach((value) => this.classes.add(value)),
      remove: (...names) => names.forEach((value) => this.classes.delete(value)),
    };
    this.style = { setProperty: (key, value) => this.properties.set(key, value) };
  }

  set className(value) {
    this.classes = new Set(value.split(/\s+/u).filter(Boolean));
  }

  get className() {
    return [...this.classes].join(" ");
  }

  setAttribute(name, value) {
    this.attributes.set(name, value);
  }

  append(...children) {
    this.children.push(...children);
    for (const child of children) child.parent = this;
  }

  replaceChildren(...children) {
    this.children = [];
    this.append(...children);
  }

  remove() {
    this.parent.children = this.parent.children.filter((child) => child !== this);
  }

  querySelector(selector) {
    return this.querySelectorAll(selector)[0];
  }

  querySelectorAll(selector) {
    const className = selector.startsWith(".") ? selector.slice(1) : undefined;
    const matches = [];
    const visit = (node) => {
      if (className && node.classes.has(className)) matches.push(node);
      for (const child of node.children) visit(child);
    };
    visit(this);
    return matches;
  }
}

globalThis.document = {
  createElement: (name) => new FakeElement(name),
  createElementNS: (_namespace, name) => new FakeElement(name),
};

const bounds = (left, top, width, height) => ({ left, top, width, height });

test("diagram rendering overlaps base edges while measurements use exact boundaries", () => {
  const origin = { left: 10, top: 20 };
  const from = bounds(30, 50, 100, 40);
  const to = bounds(210, 130, 80, 60);

  assert.deepEqual(measuredEdgeEndpoints(from, to, origin), {
    from: { x: 117, y: 50 },
    to: { x: 203, y: 140 },
  });
  assert.deepEqual(measuredEdgeEndpoints(from, to, origin, 0), {
    from: { x: 120, y: 50 },
    to: { x: 200, y: 140 },
  });
});

test("responsive vertical edges select bottom and top boundary ports", () => {
  const origin = { left: 0, top: 0 };
  const from = bounds(40, 20, 80, 60);
  const to = bounds(50, 180, 100, 40);

  assert.deepEqual(measuredEdgeEndpoints(from, to, origin, 0), {
    from: { x: 80, y: 80 },
    to: { x: 100, y: 180 },
  });
});

const diagramView = () => {
  const svg = new FakeElement("svg");
  const stage = new FakeElement("div");
  stage.getBoundingClientRect = () => bounds(0, 0, 400, 160);
  const source = new FakeElement("div");
  source.getBoundingClientRect = () => bounds(20, 50, 80, 40);
  const target = new FakeElement("div");
  target.getBoundingClientRect = () => bounds(260, 50, 80, 40);
  return {
    plan: {
      seed: 42,
      diagram: { edges: [{ id: "edge-0", from: "node-0", to: "node-1" }] },
    },
    nodes: new Map([
      ["node-0", source],
      ["node-1", target],
    ]),
    diagramView: { stage, svg },
  };
};

test("base topology renders an arrow or an explicit selected break", () => {
  const view = diagramView();
  drawDiagram(view);
  const directed = view.diagramView.svg.children[1];
  assert.match(directed.attributes.get("marker-end"), /^url\(#scene-arrow-/u);
  assert.equal(directed.attributes.get("pathLength"), "1");

  drawDiagram(view, "edge-0");
  const broken = view.diagramView.svg.children[1];
  assert.equal(broken.classes.has("is-disconnected"), true);
  assert.equal(broken.attributes.has("marker-end"), false);
});

const traceView = () => {
  const root = new FakeElement("article");
  root.getBoundingClientRect = () => bounds(0, 0, 600, 300);
  const overlay = new FakeElement("svg");
  const terminal = new FakeElement("div");
  const backing = new FakeElement("span");
  backing.classList.add("scene-effect-backing");
  terminal.append(backing);
  root.append(terminal, overlay);
  return {
    root,
    overlay,
    nodes: new Map([["terminal", terminal]]),
    terminal,
    backing,
  };
};

const flowTrace = {
  events: [
    { kind: "interaction_started", at_ms: 0 },
    {
      kind: "edge_traversed",
      edge_id: "edge-0",
      from: { x: 10, y: 20 },
      to: { x: 80, y: 20 },
      at_ms: 0,
      duration_ms: 240,
    },
    {
      kind: "flow_disconnected",
      edge_id: "edge-1",
      point: { x: 120, y: 20 },
      at_ms: 240,
      duration_ms: 120,
      sparks: 3,
    },
    {
      kind: "backing_glitched",
      target_id: "terminal",
      at_ms: 360,
      duration_ms: 140,
      offset_x: -2,
      offset_y: 3,
    },
    {
      kind: "success_reinforced",
      target_id: "terminal",
      point: { x: 200, y: 20 },
      at_ms: 500,
    },
    { kind: "interaction_finished", at_ms: 500 },
  ],
};

test("flow trace behavior projects traversal, sparks, success, and terminal backing", () => {
  const view = traceView();
  const jobs = [];
  const stop = renderSceneTrace(flowTrace, view, {
    schedule(action, at) {
      jobs.push({ action, at });
      return jobs.length;
    },
    cancel() {},
  });
  for (const job of jobs.slice().sort((left, right) => left.at - right.at)) job.action();

  assert.equal(view.overlay.children.some((node) => node.classes.has("scene-edge-traversal")), true);
  const sparks = view.overlay.children.find((node) => node.classes.has("scene-disconnect-sparks"));
  assert.equal(sparks.children.length, 3);
  assert.equal(view.terminal.querySelectorAll(".scene-success-reinforcement").length, 1);
  assert.equal(view.backing.classes.has("is-glitching"), true);
  assert.equal(view.backing.properties.get("--glitch-x"), "-2px");
  stop();
  assert.equal(view.overlay.children.length, 0);
});

test("reduced motion schedules no jobs and keeps only static approval", () => {
  const view = traceView();
  let scheduled = false;
  renderSceneTrace(flowTrace, view, {
    reducedMotion: true,
    schedule() {
      scheduled = true;
    },
  });

  assert.equal(scheduled, false);
  assert.equal(view.overlay.children.length, 0);
  const approvals = view.terminal.querySelectorAll(".scene-success-reinforcement");
  assert.equal(approvals.length, 1);
  assert.equal(approvals[0].attributes.get("aria-hidden"), "true");
  assert.equal(view.backing.classes.has("is-glitching"), false);
});
