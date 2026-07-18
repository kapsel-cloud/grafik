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

export const renderTrace = (
  trace,
  {
    group,
    reducedMotion,
    announce = () => {},
    createLine = browserLine,
    schedule = globalThis.setTimeout,
    cancel = globalThis.clearTimeout,
  },
) => {
  group.replaceChildren();
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
    if (event.kind === "connector_finished") {
      timers.push(schedule(() => announce(`${result} complete.`), event.at_ms));
    }
  }

  return () => {
    for (const timer of timers) cancel(timer);
    group.replaceChildren();
  };
};
