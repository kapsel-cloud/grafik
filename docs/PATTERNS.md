# Spatial patterns

- **Status:** Active vocabulary and bounded profile contract
- **Kind:** Normative spatial-field contract
- **Authority:** Canonical for spatial terms, route invariants, and bounded pattern profiles
- **Owns:** Field vocabulary, exclusion rules, and density/lifetime/velocity/chaos limits
- **Does not own:** Serialization, browser scheduling, themes, or product outcome meaning

## Vocabulary

- **Living diagram:** a seeded simulation whose semantic events deform a spatial field around
  explanatory panels.
- **Spatial field:** the shared coordinate system for sparse structure, connectors, symbols, and
  local glitches.
- **Panel:** a readable rectangle and default exclusion zone.
- **Edge port:** an attachment point on a panel edge.
- **Pattern:** a bounded evolution with explicit density, lifetime, velocity, direction, and chaos.
- **Outcome cue:** a bounded profile mapping semantic outcome to patterns without changing meaning.
- **Scene plan:** one complete renderer-neutral content, layout, diagram, style-role, and interaction
  description.
- **Scene node:** a stable identified item in logical reading order. Its kind is `frame`, `header`,
  `facts`, `list`, `table`, `diagram`, `result`, `actions`, or `menu`.
- **Grid role:** a curated placement role interpreted responsively by the browser; never raw CSS.
- **Diagram topology:** stable nodes and parent/child or directed edges from which measured connectors
  are derived.
- **Style role:** semantic presentation intent such as `quiet`, `standard`, `emphasis`, `outcome`, or
  `interactive`; never a literal color, font, or selector.
- **Effect target:** a stable scene node or diagram edge that may receive decorative events.
- **Interaction script:** deterministic events associated with scene load or one stable action ID.
- **Phase:** one controlled interval in an interaction script. Phases are ordered and may overlap only
  when their combined live effects remain within budget.

## Receipt scene profiles

One receipt scene uses one curated layout profile: `instrument`, `split`, `asymmetric`, `result_led`,
`diagram_led`, `terminal`, or `editorial`. The plan preserves one logical tree independent of visual
placement. Grid roles may use up to twelve conceptual columns; the browser maps every profile to one
column at 320 CSS px without changing reading order.

Receipt content primitives are bounded text, facts, lists, tables, results, actions, and menus.
Button size is one of `compact`, `regular`, or `prominent`; the browser maps the role to accessible
control geometry. Diagram form is one of `linear`, `tree`, `state`, or `network`. Every diagram edge
is derived from declared topology and stable node IDs. The browser measures rendered node rectangles
and ports before Rust emits connector or packet coordinates.

Seeded pattern composition privately selects from `pulse`, `glitch`, `invert`, `fragment`,
`edge_traverse`, `packet`, and `scanline`. Callers choose a curated recipe and maximum intensity, not
individual random knobs. Patterns target decorative backing or overlay layers. Readable labels,
tables, lists, and result tokens never move, fragment, invert, or enter decorative reading order.

Receipt-scene hard budgets:

| Budget | Limit |
| --- | --- |
| Scene nodes / tree depth | At most 48 / 5 |
| Conceptual grid | At most 12 columns and 16 rows |
| Actions / menu items | At most 8 / 8 |
| Diagram nodes / edges | At most 16 / 24 |
| Effect plans / phases per plan | At most 12 / 4 |
| Simultaneously live effects | At most 5 |
| Local fragments | At most 8 per effect |
| Decorative displacement | At most 3 CSS px |
| One phase / complete interaction | At most 2,400 ms / 8,000 ms |
| Seeded scene JSON | At most 64 KiB |

Invalid input or a budget that cannot contain the requested content fails before randomness
advances. Rare choices consume the same hard budgets as normal choices and cannot increase the
limits.

## Tracer connector

The browser selects the hero's bottom edge port and receipt's top edge port. The simulation requires
non-overlapping panels with vertical clearance. It creates an orthogonal route in that clearance:
one outward segment, a seeded lateral choice when useful, one cross-gap segment, and one inward
segment. Zero-length segments are omitted.

Panel interiors are closed exclusion zones. A segment may touch its selected port but must not enter
either interior. Growth proceeds from the hero; retraction removes segments in reverse order from
the receipt leaf.

Tracer budgets:

| Budget | Limit |
| --- | --- |
| Connector density | At most 5 live segments |
| Segment growth | 120–260 ms per segment |
| Segment retraction | 90–180 ms per segment |
| Route clearance | At least 8 CSS px from panel interiors in the gap |
| Direction set | Orthogonal only |
| Chaos | One seeded lateral route choice; no branching |
| Lifetime | Every grown segment is retracted in the same trace |

## Succeeded progress profile

`SUCCEEDED` uses the tracer connector as weighted progress. Each grown segment receives one seeded
weight from 1 through 3; weight 2 is most common, weight 1 is next, and weight 3 is rare. After growth,
one seeded pulse traverses the connector before leaf-first retraction. Pulse duration is 360–720 ms
and intensity is 1–3. Growth keeps the 120–260 ms segment budget, retraction keeps 90–180 ms per
segment, density stays at five live segments or fewer, and direction remains orthogonal with no
branching.

## Failed local-glitch profile

`FAILED` emits one seeded glitch behind the outcome text. Duration is 140–260 ms, horizontal and
vertical displacement are each at most 3 CSS px, and strip count is 1–3. Only the decorative backing
layer moves; the readable text, panel, and surrounding layout remain fixed. The browser maps the
event to the concrete outcome element and states `FAILED` in HTML independently.

## Unknown question-mark profile

`UNKNOWN` emits decorative `?` marks behind the outcome region for a two-second burst. The seeded
rate is 1–3 marks per second, normal density is at most three live marks, and lifetime is 450–900 ms.
A rare one-in-32 seeded choice may add one cluster of up to two marks while preserving a hard maximum
of five live marks. Coordinates are bounded to the outcome region, and each mark carries one of
three semantic palette roles rather than a literal color. Marks never enter reading order or carry
meaning unavailable in the `UNKNOWN` text.

Other dispositions do not reuse these receiver cues. `NOT_ATTEMPTED` remains a pre-attempt
disposition and receives no animated receiver profile.

## Future bounded profile

Ambient drift remains independent sparse growth with coordinate changes every 7–11 seconds.

No profile may move labels, displace the full screen, obscure panel text, or make meaning depend on
motion or color. Reduced motion suppresses every pulse, glitch, and question mark.
