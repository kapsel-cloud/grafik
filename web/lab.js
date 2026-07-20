import init, { grafik_scene, grafik_scene_trace } from "./pkg/grafik.js";
import { drawDiagram, measureScene, renderScene, renderSceneTrace } from "./scene-adapter.js";

const mount = document.querySelector("[data-flow-mount]");
const seedInput = document.querySelector("[data-seed]");
const status = document.querySelector("[data-status]");
const outcomeButtons = [...document.querySelectorAll("[data-outcome]")];
const disconnectControls = document.querySelector("[data-disconnect-controls]");
const replayButton = document.querySelector("[data-replay]");
const selectionTitle = document.querySelector("[data-selection-title]");
const sceneOutput = document.querySelector("[data-scene-json]");
const traceOutput = document.querySelector("[data-trace-json]");
const motion = matchMedia("(prefers-reduced-motion: reduce)");
const requestedSeed = Number(new URLSearchParams(location.search).get("seed"));
let selectedOutcome = "SUCCEEDED";
let disconnectedEdge = null;
let plan;
let view;
let stopTrace = () => {};
let resizeTimer;

if (Number.isSafeInteger(requestedSeed) && requestedSeed > 0) seedInput.value = String(requestedSeed);

const budgets = {
  max_nodes: 48,
  max_depth: 5,
  max_actions: 8,
  max_diagram_nodes: 16,
  max_diagram_edges: 24,
  max_effects: 12,
  max_phases_per_effect: 4,
  max_live_effects: 5,
  max_fragments: 8,
  max_displacement: 3,
  max_phase_ms: 2400,
  max_interaction_ms: 8000,
  max_json_bytes: 65536,
};

const flowContent = {
  eyebrow: "Standalone Grafik tracer · simulated input",
  title: "One bounded external-effect flow.",
  outcome: "SUCCEEDED",
  facts: [
    { label: "fixture", value: "deterministic and local" },
    { label: "authority", value: "renderer-neutral Rust trace" },
  ],
  evidence: [
    "Directed edges follow the declared topology.",
    "A disconnect stops traversal before the broken edge.",
    "Outcome labels remain explicit without decorative motion.",
  ],
  table_headers: ["stage", "meaning"],
  table_rows: [
    ["grant", "admission"],
    ["journal", "durable attempt"],
    ["provider seam", "external boundary"],
    ["observe", "bounded observation"],
    ["receipt", "explicit result"],
  ],
  diagram_labels: ["grant", "journal", "provider seam", "observe", "receipt"],
  actions: [{ key: "replay", label: "Replay flow" }],
  menu_items: ["Inspect plan", "Inspect trace"],
};

const controlledSeed = () => Number(seedInput.value);

const request = () => ({
  seed: controlledSeed(),
  recipe: "animated_flow",
  content: flowContent,
  budgets,
});

const flowTrigger = () => ({
  kind: "flow",
  result_source: "simulated",
  final_disposition: selectedOutcome,
  disconnected_edge: disconnectedEdge,
});

const updatePressedState = () => {
  for (const button of outcomeButtons) {
    button.setAttribute("aria-pressed", String(button.dataset.outcome === selectedOutcome));
  }
  for (const button of disconnectControls.querySelectorAll("button")) {
    const edge = button.dataset.disconnect || null;
    button.setAttribute("aria-pressed", String(edge === disconnectedEdge));
  }
  if (view?.resultToken) view.resultToken.textContent = selectedOutcome;
};

const runFlow = () => {
  if (!plan || !view) return;
  stopTrace();
  updatePressedState();
  drawDiagram(view, disconnectedEdge);
  const input = { plan, geometry: measureScene(view), trigger: flowTrigger() };
  try {
    const trace = JSON.parse(grafik_scene_trace(JSON.stringify(input)));
    traceOutput.textContent = JSON.stringify({ input, trace }, null, 2);
    stopTrace = renderSceneTrace(trace, view, {
      reducedMotion: motion.matches,
      announce: (message) => {
        status.textContent = message;
      },
    });
    status.textContent = disconnectedEdge
      ? `${selectedOutcome} selected; replay stops at ${disconnectedEdge}.`
      : `${selectedOutcome} selected; complete flow replay running.`;
  } catch (error) {
    status.textContent = `Flow interaction unavailable: ${String(error)}`;
  }
};

const buildDisconnectControls = () => {
  disconnectControls.replaceChildren();
  const none = document.createElement("button");
  none.type = "button";
  none.textContent = "No disconnect";
  none.dataset.disconnect = "";
  disconnectControls.append(none);
  for (const edge of plan.diagram.edges) {
    const source = plan.diagram.nodes.find((node) => node.id === edge.from)?.label;
    const target = plan.diagram.nodes.find((node) => node.id === edge.to)?.label;
    const button = document.createElement("button");
    button.type = "button";
    button.dataset.disconnect = edge.id;
    button.textContent = `${source} → ${target}`;
    disconnectControls.append(button);
  }
  for (const button of disconnectControls.querySelectorAll("button")) {
    button.addEventListener("click", () => {
      disconnectedEdge = button.dataset.disconnect || null;
      runFlow();
    });
  }
};

const generate = async () => {
  const valid = Number.isSafeInteger(controlledSeed()) && controlledSeed() > 0;
  seedInput.setCustomValidity(valid ? "" : "Seed must be a positive integer.");
  if (!valid) return seedInput.reportValidity();
  stopTrace();
  status.textContent = "Generating deterministic flow in Rust…";
  const url = new URL(location.href);
  url.searchParams.set("seed", String(controlledSeed()));
  history.replaceState(null, "", url);
  try {
    plan = JSON.parse(grafik_scene(JSON.stringify(request())));
    view = renderScene(plan, mount, { onAction: runFlow });
    buildDisconnectControls();
    updatePressedState();
    sceneOutput.textContent = JSON.stringify(plan, null, 2);
    selectionTitle.textContent = `Seed ${plan.seed} · ${plan.diagram.nodes.length} nodes · ${plan.diagram.edges.length} edges`;
    await document.fonts.ready;
    requestAnimationFrame(runFlow);
  } catch (error) {
    status.textContent = `Grafik scene generation failed: ${String(error)}`;
  }
};

for (const button of outcomeButtons) {
  button.addEventListener("click", () => {
    selectedOutcome = button.dataset.outcome;
    runFlow();
  });
}
replayButton.addEventListener("click", runFlow);
seedInput.addEventListener("change", generate);
motion.addEventListener("change", runFlow);
addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(runFlow, 160);
});

const start = async () => {
  try {
    await init();
    await generate();
  } catch (error) {
    status.textContent = `Grafik could not start: ${String(error)}`;
  }
};

start();
