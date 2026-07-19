const SVG_NAMESPACE = "http://www.w3.org/2000/svg";

const element = (name, className, text) => {
  const node = document.createElement(name);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
};

const backing = () => {
  const node = element("span", "scene-effect-backing");
  node.setAttribute("aria-hidden", "true");
  return node;
};

const nodeShell = (node) => {
  const shell = element("section", `scene-node scene-node-${node.kind}`);
  shell.dataset.sceneNode = node.id;
  shell.dataset.gridRole = node.grid_role;
  shell.dataset.visualRole = node.visual_role;
  shell.append(backing());
  return shell;
};

const renderHeader = (shell, content) => {
  shell.append(element("p", "scene-eyebrow", content.eyebrow));
  shell.append(element("h2", "scene-title", content.title));
};

const renderFacts = (shell, content) => {
  const list = element("dl", "scene-facts");
  for (const item of content.items) {
    const row = element("div", "scene-fact");
    row.append(element("dt", "scene-fact-label", item.label));
    row.append(element("dd", "scene-fact-value", item.value));
    list.append(row);
  }
  shell.append(list);
};

const renderList = (shell, content) => {
  const list = element("ol", "scene-list");
  for (const item of content.items) list.append(element("li", "", item));
  shell.append(list);
};

const renderTable = (shell, content) => {
  const table = element("table", "scene-table");
  table.append(element("caption", "", "Receipt records"));
  const head = document.createElement("thead");
  const headRow = document.createElement("tr");
  for (const header of content.headers) {
    const cell = element("th", "", header);
    cell.scope = "col";
    headRow.append(cell);
  }
  head.append(headRow);
  table.append(head);
  const body = document.createElement("tbody");
  for (const row of content.rows) {
    const tableRow = document.createElement("tr");
    for (const value of row) tableRow.append(element("td", "", value));
    body.append(tableRow);
  }
  table.append(body);
  shell.append(table);
};

const renderDiagram = (shell, plan, nodes) => {
  shell.dataset.diagramForm = plan.diagram.form;
  shell.append(element("h3", "scene-section-title", "Flow"));
  const stage = element("div", "scene-diagram-stage");
  const svg = document.createElementNS(SVG_NAMESPACE, "svg");
  svg.classList.add("scene-diagram-edges");
  svg.setAttribute("aria-hidden", "true");
  stage.append(svg);
  const nodeList = element("div", "scene-diagram-nodes");
  for (const diagramNode of plan.diagram.nodes) {
    const node = element("div", "scene-diagram-node", diagramNode.label);
    node.dataset.sceneNode = diagramNode.id;
    node.dataset.terminal = String(diagramNode.terminal);
    node.prepend(backing());
    nodes.set(diagramNode.id, node);
    nodeList.append(node);
  }
  stage.append(nodeList);
  shell.append(stage);
  return { stage, svg };
};

const renderResult = (shell, content) => {
  shell.append(element("p", "scene-section-title", "Result"));
  shell.append(element("output", "scene-result-token", content.disposition));
};

const renderActions = (shell, content, nodes, onAction) => {
  const group = element("div", "scene-actions");
  group.setAttribute("role", "group");
  group.setAttribute("aria-label", "Receipt actions");
  for (const action of content.items) {
    const button = element("button", "scene-action", action.label);
    button.type = "button";
    button.dataset.actionId = action.id;
    button.dataset.buttonSize = action.size;
    button.dataset.sceneNode = action.id;
    button.prepend(backing());
    nodes.set(action.id, button);
    button.addEventListener("click", () => onAction(action.id));
    group.append(button);
  }
  shell.append(group);
};

const renderMenu = (shell, content) => {
  const details = element("details", "scene-menu");
  details.append(element("summary", "", "Options"));
  const list = document.createElement("ul");
  for (const item of content.items) list.append(element("li", "", item));
  details.append(list);
  shell.append(details);
};

const renderNode = (node, plan, nodes, onAction) => {
  const shell = nodeShell(node);
  nodes.set(node.id, shell);
  let diagramView;
  switch (node.content.kind) {
    case "header":
      renderHeader(shell, node.content);
      break;
    case "facts":
      renderFacts(shell, node.content);
      break;
    case "list":
      renderList(shell, node.content);
      break;
    case "table":
      renderTable(shell, node.content);
      break;
    case "result":
      renderResult(shell, node.content);
      break;
    case "actions":
      renderActions(shell, node.content, nodes, onAction);
      break;
    case "menu":
      renderMenu(shell, node.content);
      break;
    default:
      break;
  }
  if (node.kind === "diagram") diagramView = renderDiagram(shell, plan, nodes);
  for (const child of node.children) {
    const rendered = renderNode(child, plan, nodes, onAction);
    shell.append(rendered.shell);
    diagramView ??= rendered.diagramView;
  }
  return { shell, diagramView };
};

const endpoint = (rect, origin, toward, overlap = 3) => {
  const center = {
    x: rect.left - origin.left + rect.width / 2,
    y: rect.top - origin.top + rect.height / 2,
  };
  const dx = toward.x - center.x;
  const dy = toward.y - center.y;
  if (Math.abs(dx) >= Math.abs(dy)) {
    return {
      x: center.x + Math.sign(dx) * (rect.width / 2 - overlap),
      y: center.y,
    };
  }
  return {
    x: center.x,
    y: center.y + Math.sign(dy) * (rect.height / 2 - overlap),
  };
};

export const measuredEdgeEndpoints = (source, target, origin) => {
  const sourceCenter = {
    x: source.left - origin.left + source.width / 2,
    y: source.top - origin.top + source.height / 2,
  };
  const targetCenter = {
    x: target.left - origin.left + target.width / 2,
    y: target.top - origin.top + target.height / 2,
  };
  return {
    from: endpoint(source, origin, targetCenter),
    to: endpoint(target, origin, sourceCenter),
  };
};

export const drawDiagram = (view) => {
  if (!view.diagramView) return;
  const { stage, svg } = view.diagramView;
  const origin = stage.getBoundingClientRect();
  svg.setAttribute("viewBox", `0 0 ${origin.width} ${origin.height}`);
  svg.replaceChildren();
  for (const edge of view.plan.diagram.edges) {
    const source = view.nodes.get(edge.from)?.getBoundingClientRect();
    const target = view.nodes.get(edge.to)?.getBoundingClientRect();
    if (!source || !target) continue;
    const { from, to } = measuredEdgeEndpoints(source, target, origin);
    const line = document.createElementNS(SVG_NAMESPACE, "line");
    line.dataset.diagramEdge = edge.id;
    line.setAttribute("x1", String(from.x));
    line.setAttribute("y1", String(from.y));
    line.setAttribute("x2", String(to.x));
    line.setAttribute("y2", String(to.y));
    svg.append(line);
  }
};

export const renderScene = (plan, mount, { onAction = () => {} } = {}) => {
  const nodes = new Map();
  const frame = element("article", "receipt-scene");
  frame.dataset.sceneNode = plan.root.id;
  frame.dataset.layout = plan.layout;
  frame.dataset.recipe = plan.recipe;
  frame.append(backing());
  nodes.set(plan.root.id, frame);
  let diagramView;
  for (const child of plan.root.children) {
    const rendered = renderNode(child, plan, nodes, onAction);
    frame.append(rendered.shell);
    diagramView ??= rendered.diagramView;
  }
  const overlay = document.createElementNS(SVG_NAMESPACE, "svg");
  overlay.classList.add("scene-trace-overlay");
  overlay.setAttribute("aria-hidden", "true");
  frame.append(overlay);
  mount.replaceChildren(frame);
  const view = { plan, root: frame, nodes, diagramView, overlay };
  drawDiagram(view);
  return view;
};

const localRect = (rect, origin) => ({
  x: rect.left - origin.left,
  y: rect.top - origin.top,
  width: rect.width,
  height: rect.height,
});

export const measureScene = (view) => {
  const origin = view.root.getBoundingClientRect();
  return [...view.nodes.entries()].map(([id, node]) => {
    const rect = localRect(node.getBoundingClientRect(), origin);
    return {
      id,
      rect,
      incoming: { x: rect.x, y: rect.y + rect.height / 2 },
      outgoing: { x: rect.x + rect.width, y: rect.y + rect.height / 2 },
    };
  });
};

const effectNode = (view, id) => view.nodes.get(id)?.querySelector(".scene-effect-backing");

const scheduleClass = (jobs, schedule, node, className, event) => {
  if (!node) return;
  jobs.push(
    schedule(() => {
      node.style.setProperty("--effect-ms", `${event.duration_ms}ms`);
      node.classList.add(className);
      jobs.push(schedule(() => node.classList.remove(className), event.duration_ms));
    }, event.at_ms),
  );
};

const traversal = (view, event, packet) => {
  const line = document.createElementNS(SVG_NAMESPACE, packet ? "circle" : "line");
  if (packet) {
    line.setAttribute("r", "4");
    line.style.setProperty("--from-x", `${event.from.x}px`);
    line.style.setProperty("--from-y", `${event.from.y}px`);
    line.style.setProperty("--to-x", `${event.to.x}px`);
    line.style.setProperty("--to-y", `${event.to.y}px`);
    line.classList.add("scene-packet");
  } else {
    line.setAttribute("x1", String(event.from.x));
    line.setAttribute("y1", String(event.from.y));
    line.setAttribute("x2", String(event.to.x));
    line.setAttribute("y2", String(event.to.y));
    line.setAttribute("pathLength", "1");
    line.classList.add("scene-edge-traversal");
  }
  line.style.setProperty("--effect-ms", `${event.duration_ms}ms`);
  view.overlay.append(line);
  return line;
};

export const renderSceneTrace = (
  trace,
  view,
  {
    reducedMotion = false,
    announce = () => {},
    schedule = globalThis.setTimeout,
    cancel = globalThis.clearTimeout,
  } = {},
) => {
  view.overlay.replaceChildren();
  const bounds = view.root.getBoundingClientRect();
  view.overlay.setAttribute("viewBox", `0 0 ${bounds.width} ${bounds.height}`);
  if (reducedMotion) {
    announce("Scene interaction applied. Motion reduced.");
    return () => {};
  }
  const jobs = [];
  for (const event of trace.events) {
    const node = "target_id" in event ? effectNode(view, event.target_id) : undefined;
    if (event.kind === "node_activated") {
      scheduleClass(jobs, schedule, node, "is-pulsing", event);
    } else if (event.kind === "backing_glitched") {
      if (node) {
        node.style.setProperty("--glitch-x", `${event.offset_x}px`);
        node.style.setProperty("--glitch-y", `${event.offset_y}px`);
      }
      scheduleClass(jobs, schedule, node, "is-glitching", event);
    } else if (event.kind === "backing_inverted") {
      scheduleClass(jobs, schedule, node, "is-inverting", event);
    } else if (event.kind === "scanline_swept") {
      scheduleClass(jobs, schedule, node, "is-scanning", event);
    } else if (event.kind === "fragments_emitted") {
      jobs.push(
        schedule(() => {
          for (let index = 0; index < event.count; index += 1) {
            const fragment = document.createElementNS(SVG_NAMESPACE, "rect");
            fragment.setAttribute("x", String(event.point.x + (index % 3) * 5 - 5));
            fragment.setAttribute("y", String(event.point.y + Math.floor(index / 3) * 5 - 5));
            fragment.setAttribute("width", "4");
            fragment.setAttribute("height", "4");
            fragment.style.setProperty("--effect-ms", `${event.duration_ms}ms`);
            fragment.classList.add("scene-fragment");
            view.overlay.append(fragment);
            jobs.push(schedule(() => fragment.remove(), event.duration_ms));
          }
        }, event.at_ms),
      );
    } else if (event.kind === "edge_traversed" || event.kind === "packet_traversed") {
      jobs.push(
        schedule(() => {
          const mark = traversal(view, event, event.kind === "packet_traversed");
          jobs.push(schedule(() => mark.remove(), event.duration_ms));
        }, event.at_ms),
      );
    } else if (event.kind === "interaction_finished") {
      jobs.push(schedule(() => announce("Scene interaction complete."), event.at_ms));
    }
  }
  return () => {
    for (const job of jobs) cancel(job);
    view.overlay.replaceChildren();
    for (const node of view.nodes.values()) {
      const layer = node.querySelector(".scene-effect-backing");
      layer?.classList.remove("is-pulsing", "is-glitching", "is-inverting", "is-scanning");
    }
  };
};
