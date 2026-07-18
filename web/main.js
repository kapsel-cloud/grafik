import init, { grafik_trace } from "./pkg/grafik.js";
import { renderTrace } from "./svg-adapter.js";

const diagram = document.querySelector("[data-diagram]");
const hero = document.querySelector("[data-panel=hero]");
const receipt = document.querySelector("[data-panel=receipt]");
const group = document.querySelector("[data-connector]");
const outcomeTarget = document.querySelector("[data-outcome-target]");
const outcomeLayer = document.querySelector("[data-outcome-layer]");
const resultText = document.querySelector("[data-result-text]");
const resultSource = document.querySelector("[data-result-source]");
const controls = document.querySelector("[data-outcome-controls]");
const outcomeButtons = [...document.querySelectorAll("[data-outcome]")];
const seedInput = document.querySelector("[data-seed]");
const status = document.querySelector("[data-status]");
const motion = matchMedia("(prefers-reduced-motion: reduce)");
let stopTrace = () => {};
let resizeTimer;
let selectedResult;

const relativeRect = (element, origin) => {
  const rect = element.getBoundingClientRect();
  return {
    x: rect.left - origin.left,
    y: rect.top - origin.top,
    width: rect.width,
    height: rect.height,
  };
};

const controlledSeed = () => {
  const seed = Number(seedInput.value);
  return Number.isSafeInteger(seed) && seed > 0 ? seed : 424242;
};

const showResult = () => {
  resultText.textContent = selectedResult.final_disposition;
  const recorded = selectedResult.result_source === "recorded";
  resultSource.textContent = recorded
    ? "Kapsel v0.1.0 evaluator recording"
    : "simulated preview · no infrastructure work";
  for (const button of outcomeButtons) {
    button.setAttribute(
      "aria-pressed",
      String(button.dataset.outcome === selectedResult.final_disposition),
    );
  }
};

const run = () => {
  if (!selectedResult) return;
  stopTrace();
  showResult();
  status.textContent = `Rendering ${selectedResult.final_disposition} profile…`;
  const origin = diagram.getBoundingClientRect();
  const heroRect = relativeRect(hero, origin);
  const receiptRect = relativeRect(receipt, origin);
  const outcomeRect = relativeRect(outcomeTarget, origin);
  const input = {
    seed: controlledSeed(),
    result_source: selectedResult.result_source,
    final_disposition: selectedResult.final_disposition,
    hero: heroRect,
    receipt: receiptRect,
    outcome_region: outcomeRect,
    hero_port: {
      point: { x: heroRect.x + heroRect.width / 2, y: heroRect.y + heroRect.height },
      edge: "bottom",
    },
    receipt_port: {
      point: { x: receiptRect.x + receiptRect.width / 2, y: receiptRect.y },
      edge: "top",
    },
  };

  try {
    const trace = JSON.parse(grafik_trace(JSON.stringify(input)));
    stopTrace = renderTrace(trace, {
      group,
      outcomeLayer,
      outcomeOrigin: { x: outcomeRect.x, y: outcomeRect.y },
      reducedMotion: motion.matches,
      announce: (message) => {
        status.textContent = message;
      },
    });
  } catch (error) {
    status.textContent = `Outcome profile unavailable: ${String(error)}`;
  }
};

const loadRecordedResult = async () => {
  const response = await fetch("./fixtures/kapsel-recorded-success.json");
  if (!response.ok) throw new Error(`recorded result returned HTTP ${response.status}`);
  const fixture = await response.json();
  return fixture.result;
};

const start = async () => {
  try {
    const [, , result] = await Promise.all([init(), document.fonts.ready, loadRecordedResult()]);
    selectedResult = result;
    controls.hidden = false;
    run();
  } catch (error) {
    status.textContent = `Grafik could not start: ${String(error)}`;
  }
};

for (const button of outcomeButtons) {
  button.addEventListener("click", () => {
    selectedResult = {
      result_source: "simulated",
      final_disposition: button.dataset.outcome,
    };
    run();
  });
}

seedInput.addEventListener("change", () => {
  const valid = Number.isSafeInteger(Number(seedInput.value)) && Number(seedInput.value) > 0;
  seedInput.setCustomValidity(valid ? "" : "Seed must be a positive integer.");
  if (valid) run();
  else seedInput.reportValidity();
});

addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(run, 120);
});
motion.addEventListener("change", run);
start();
