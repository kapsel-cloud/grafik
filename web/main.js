import init, { grafik_trace } from "./pkg/grafik.js";
import { renderTrace } from "./svg-adapter.js";

const diagram = document.querySelector("[data-diagram]");
const hero = document.querySelector("[data-panel=hero]");
const receipt = document.querySelector("[data-panel=receipt]");
const group = document.querySelector("[data-connector]");
const status = document.querySelector("[data-status]");
const motion = matchMedia("(prefers-reduced-motion: reduce)");
let stopTrace = () => {};
let resizeTimer;
let recordedResult;

const relativeRect = (element, origin) => {
  const rect = element.getBoundingClientRect();
  return {
    x: rect.left - origin.left,
    y: rect.top - origin.top,
    width: rect.width,
    height: rect.height,
  };
};

const run = () => {
  if (!recordedResult) return;
  stopTrace();
  const origin = diagram.getBoundingClientRect();
  const heroRect = relativeRect(hero, origin);
  const receiptRect = relativeRect(receipt, origin);
  const input = {
    seed: 424242,
    result_source: recordedResult.result_source,
    final_disposition: recordedResult.final_disposition,
    hero: heroRect,
    receipt: receiptRect,
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
      reducedMotion: motion.matches,
      announce: (message) => {
        status.textContent = message;
      },
    });
  } catch (error) {
    status.textContent = `Connector unavailable: ${String(error)}`;
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
    recordedResult = result;
    run();
  } catch (error) {
    status.textContent = `Grafik could not start: ${String(error)}`;
  }
};

addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(run, 120);
});
motion.addEventListener("change", run);
start();
