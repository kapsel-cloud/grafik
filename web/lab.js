import init, { grafik_scene, grafik_scene_trace } from "./pkg/grafik.js";
import {
  drawDiagram,
  measureScene,
  renderScene,
  renderSceneTrace,
} from "./scene-adapter.js";

const gallery = document.querySelector("[data-gallery]");
const seedInput = document.querySelector("[data-seed]");
const recipeInput = document.querySelector("[data-recipe]");
const windowButtons = [...document.querySelectorAll("[data-window-step]")];
const status = document.querySelector("[data-status]");
const selectionTitle = document.querySelector("[data-selection-title]");
const sceneOutput = document.querySelector("[data-scene-json]");
const traceOutput = document.querySelector("[data-trace-json]");
const replayButton = document.querySelector("[data-replay-selected]");
const motion = matchMedia("(prefers-reduced-motion: reduce)");
const parameters = new URLSearchParams(location.search);
const recipes = new Set(["calm", "balanced", "vivid"]);
const requestedSeed = Number(parameters.get("seed"));
const requestedRecipe = parameters.get("recipe");
let candidates = [];
let selected;
let resizeTimer;

if (Number.isSafeInteger(requestedSeed) && requestedSeed > 0) {
  seedInput.value = String(requestedSeed);
}
if (recipes.has(requestedRecipe)) recipeInput.value = requestedRecipe;

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

const receiptContent = {
  eyebrow: "Recorded operation · local simulated presentation",
  title: "Evidence moved through a bounded receiver flow.",
  outcome: "SUCCEEDED",
  facts: [
    { label: "target", value: "example/receiver" },
    { label: "digest", value: "sha256:0123456789abcdef" },
    { label: "policy", value: "bounded observation" },
  ],
  evidence: [
    "Request accepted into the local journal.",
    "Provider delivery facts were observed.",
    "Receiver result remained semantically distinct.",
  ],
  table_headers: ["stage", "state", "attempt"],
  table_rows: [
    ["journal", "written", "01"],
    ["provider", "delivered", "01"],
    ["receiver", "succeeded", "01"],
  ],
  diagram_labels: ["request", "journal", "provider", "receiver", "result"],
  actions: [
    { key: "pulse", label: "Pulse result" },
    { key: "inspect", label: "Inspect flow" },
  ],
  menu_items: ["Copy candidate URL", "Pin for comparison", "Show scene JSON"],
};

const controlledSeed = () => Number(seedInput.value);

const validateSeed = () => {
  const seed = controlledSeed();
  const valid = Number.isSafeInteger(seed) && seed > 0;
  seedInput.setCustomValidity(valid ? "" : "Seed must be a positive integer.");
  if (!valid) seedInput.reportValidity();
  return valid;
};

const updateUrl = () => {
  const url = new URL(location.href);
  url.searchParams.set("seed", String(controlledSeed()));
  url.searchParams.set("recipe", recipeInput.value);
  history.replaceState(null, "", url);
};

const sceneRequest = (seed) => ({
  seed,
  recipe: recipeInput.value,
  content: receiptContent,
  budgets,
});

const selectCandidate = (candidate) => {
  selected?.card.classList.remove("is-selected");
  selected = candidate;
  candidate.card.classList.add("is-selected");
  selectionTitle.textContent = `Seed ${candidate.plan.seed} · ${candidate.plan.layout} · ${candidate.plan.diagram.form}`;
  sceneOutput.textContent = JSON.stringify(candidate.plan, null, 2);
  traceOutput.textContent = candidate.trace
    ? JSON.stringify(candidate.trace, null, 2)
    : "No interaction yet.";
  replayButton.disabled = false;
};

const runInteraction = (candidate, trigger) => {
  candidate.stopTrace();
  drawDiagram(candidate.view);
  const input = {
    plan: candidate.plan,
    geometry: measureScene(candidate.view),
    trigger,
  };
  try {
    const trace = JSON.parse(grafik_scene_trace(JSON.stringify(input)));
    candidate.trace = trace;
    candidate.stopTrace = renderSceneTrace(trace, candidate.view, {
      reducedMotion: motion.matches,
      announce: (message) => {
        candidate.status.textContent = message;
        if (selected === candidate) status.textContent = message;
      },
    });
    candidate.status.textContent = `${trigger.kind} interaction running`;
    if (selected === candidate) traceOutput.textContent = JSON.stringify(trace, null, 2);
  } catch (error) {
    candidate.status.textContent = `Interaction unavailable: ${String(error)}`;
  }
};

const candidateCard = (plan) => {
  const card = document.createElement("article");
  card.className = "scene-candidate";
  const heading = document.createElement("header");
  heading.className = "candidate-heading";
  const identity = document.createElement("div");
  const title = document.createElement("h2");
  title.textContent = `Seed ${plan.seed}`;
  const metadata = document.createElement("p");
  metadata.textContent = `${plan.layout} · ${plan.diagram.form} · ${plan.effects.length} effects`;
  identity.append(title, metadata);
  const controls = document.createElement("div");
  controls.className = "candidate-controls";
  const select = document.createElement("button");
  select.type = "button";
  select.textContent = "Inspect";
  const pin = document.createElement("button");
  pin.type = "button";
  pin.textContent = "Pin";
  pin.setAttribute("aria-pressed", "false");
  controls.append(select, pin);
  heading.append(identity, controls);
  const mount = document.createElement("div");
  mount.className = "candidate-scene";
  const candidateStatus = document.createElement("p");
  candidateStatus.className = "candidate-status";
  candidateStatus.textContent = "Measuring local scene…";
  card.append(heading, mount, candidateStatus);
  gallery.append(card);

  let candidate;
  const view = renderScene(plan, mount, {
    onAction: (actionId) => {
      selectCandidate(candidate);
      runInteraction(candidate, { kind: "action", action_id: actionId });
    },
  });
  candidate = {
    plan,
    card,
    view,
    status: candidateStatus,
    trace: undefined,
    stopTrace: () => {},
  };
  select.addEventListener("click", () => selectCandidate(candidate));
  pin.addEventListener("click", () => {
    const pinned = pin.getAttribute("aria-pressed") !== "true";
    pin.setAttribute("aria-pressed", String(pinned));
    pin.textContent = pinned ? "Pinned" : "Pin";
    card.classList.toggle("is-pinned", pinned);
  });
  return candidate;
};

const generateWindow = async () => {
  if (!validateSeed()) return;
  updateUrl();
  status.textContent = "Generating six deterministic scenes in Rust…";
  for (const candidate of candidates) candidate.stopTrace();
  candidates = [];
  selected = undefined;
  replayButton.disabled = true;
  gallery.replaceChildren();

  try {
    const firstSeed = controlledSeed();
    for (let offset = 0; offset < 6; offset += 1) {
      const plan = JSON.parse(grafik_scene(JSON.stringify(sceneRequest(firstSeed + offset))));
      candidates.push(candidateCard(plan));
    }
    await document.fonts.ready;
    requestAnimationFrame(() => {
      for (const candidate of candidates) {
        drawDiagram(candidate.view);
        runInteraction(candidate, { kind: "load" });
      }
      if (candidates[0]) selectCandidate(candidates[0]);
      status.textContent = `Showing seeds ${firstSeed}–${firstSeed + 5}.`;
    });
  } catch (error) {
    status.textContent = `Grafik scene generation failed: ${String(error)}`;
  }
};

for (const button of windowButtons) {
  button.addEventListener("click", () => {
    seedInput.value = String(Math.max(1, controlledSeed() + Number(button.dataset.windowStep)));
    generateWindow();
  });
}
seedInput.addEventListener("change", generateWindow);
recipeInput.addEventListener("change", generateWindow);
replayButton.addEventListener("click", () => {
  if (selected) runInteraction(selected, { kind: "load" });
});
motion.addEventListener("change", () => {
  for (const candidate of candidates) runInteraction(candidate, { kind: "load" });
});
addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(() => {
    for (const candidate of candidates) {
      drawDiagram(candidate.view);
      runInteraction(candidate, { kind: "load" });
    }
  }, 160);
});

const start = async () => {
  try {
    await init();
    await generateWindow();
  } catch (error) {
    status.textContent = `Grafik could not start: ${String(error)}`;
  }
};

start();
