const SVG_NAMESPACE = "http://www.w3.org/2000/svg";

export const segmentAttributes = (event) => ({
  x1: String(event.from.x),
  y1: String(event.from.y),
  x2: String(event.to.x),
  y2: String(event.to.y),
  pathLength: "1",
  "data-progress-weight": String(event.weight),
});

const browserLine = () => document.createElementNS(SVG_NAMESPACE, "line");
const browserMark = () => document.createElement("span");

export const renderTrace = (
  trace,
  {
    group,
    outcomeLayer,
    outcomeOrigin = { x: 0, y: 0 },
    reducedMotion,
    announce = () => {},
    createLine = browserLine,
    createMark = browserMark,
    schedule = globalThis.setTimeout,
    cancel = globalThis.clearTimeout,
  },
) => {
  group.replaceChildren();
  outcomeLayer.replaceChildren();
  outcomeLayer.classList.remove("is-failure-glitching");
  const source = trace.result_source === "recorded" ? "Recorded" : "Simulated";
  const result = `${source} ${trace.final_disposition} result`;
  if (reducedMotion) {
    announce(`${result} applied. Motion reduced.`);
    return () => {};
  }

  const segments = new Map();
  const timers = [];
  for (const event of trace.events) {
    if (event.kind === "segment_grew") {
      timers.push(
        schedule(() => {
          const line = createLine();
          for (const [name, value] of Object.entries(segmentAttributes(event))) {
            line.setAttribute(name, value);
          }
          line.classList.add("connector-segment");
          line.style.setProperty("--traversal-ms", `${event.duration_ms}ms`);
          group.append(line);
          segments.set(event.index, line);
        }, event.at_ms),
      );
    }
    if (event.kind === "success_pulsed") {
      timers.push(
        schedule(() => {
          const lines = [...segments.values()];
          const step = event.duration_ms / Math.max(lines.length, 1);
          for (const [index, line] of lines.entries()) {
            line.setAttribute("data-pulse-intensity", String(event.intensity));
            line.style.setProperty("--pulse-delay-ms", `${Math.round(index * step)}ms`);
            line.style.setProperty("--pulse-ms", `${Math.round(step)}ms`);
            line.classList.add("is-success-pulsing");
          }
          timers.push(
            schedule(() => {
              for (const line of lines) line.classList.remove("is-success-pulsing");
            }, event.duration_ms),
          );
        }, event.at_ms),
      );
    }
    if (event.kind === "failure_glitched") {
      timers.push(
        schedule(() => {
          outcomeLayer.style.setProperty("--glitch-x", `${event.offset_x}px`);
          outcomeLayer.style.setProperty("--glitch-y", `${event.offset_y}px`);
          outcomeLayer.style.setProperty("--glitch-strips", String(event.strips));
          outcomeLayer.style.setProperty("--glitch-ms", `${event.duration_ms}ms`);
          outcomeLayer.classList.add("is-failure-glitching");
          timers.push(
            schedule(
              () => outcomeLayer.classList.remove("is-failure-glitching"),
              event.duration_ms,
            ),
          );
        }, event.at_ms),
      );
    }
    if (event.kind === "question_mark_appeared") {
      timers.push(
        schedule(() => {
          const mark = createMark();
          mark.textContent = "?";
          mark.dataset.tone = event.tone;
          mark.classList.add("question-mark");
          mark.style.setProperty("--mark-x", `${event.point.x - outcomeOrigin.x}px`);
          mark.style.setProperty("--mark-y", `${event.point.y - outcomeOrigin.y}px`);
          mark.style.setProperty("--mark-ms", `${event.lifetime_ms}ms`);
          outcomeLayer.append(mark);
          timers.push(schedule(() => mark.remove(), event.lifetime_ms));
        }, event.at_ms),
      );
    }
    if (event.kind === "segment_retracted") {
      timers.push(
        schedule(() => {
          const line = segments.get(event.index);
          if (!line) return;
          line.style.setProperty("--traversal-ms", `${event.duration_ms}ms`);
          line.classList.add("is-retracting");
          timers.push(
            schedule(() => {
              line.remove();
              segments.delete(event.index);
            }, event.duration_ms),
          );
        }, event.at_ms),
      );
    }
    if (event.kind === "outcome_finished") {
      timers.push(schedule(() => announce(`${result} complete.`), event.at_ms));
    }
  }

  return () => {
    for (const timer of timers) cancel(timer);
    group.replaceChildren();
    outcomeLayer.replaceChildren();
    outcomeLayer.classList.remove("is-failure-glitching");
  };
};
