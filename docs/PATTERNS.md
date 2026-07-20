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
- **Flow state:** one selected final disposition plus at most one disconnected declared edge; it
  changes trace selection without changing generated topology.
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

## Animated flow profile

The curated `animated_flow` recipe uses the caller's bounded labels to generate one linear directed
topology in label order. The first fixture is `grant -> journal -> provider seam -> observe ->
receipt`. Its plan retains every declared edge while flow state selects `SUCCEEDED`, `FAILED`, or
`UNKNOWN` and may mark one edge disconnected.

Replay traverses every connected edge once in declared order. Each traversal lasts 240–480 ms and
uses browser-measured source and target ports. A selected disconnect suppresses that edge and every
later traversal, then emits one break-local spark cue at the selected edge midpoint. The cue contains
1–4 particles, lasts 120–240 ms, and does not imply that the target or terminal completed.

A fully connected flow may apply exactly one terminal cue after traversal:

- `SUCCEEDED` may add one static, decorative approval mark that the browser projects as a thumbs-up;
- `FAILED` emits one 140–260 ms glitch on the terminal's decorative backing; and
- `UNKNOWN` emits no success or failure cue.

The explicit result label remains readable for every state. When disconnected, the selected result
label remains visible as control state, but all terminal outcome cues are suppressed. Traversal,
sparks, approval marks, and glitches are decorative, `aria-hidden`, pointer-inert, and cannot move or
replace readable labels.

Animated-flow budgets:

| Budget | Limit |
| --- | --- |
| Flow nodes / edges | At most 16 / 15; first fixture uses 5 / 4 |
| Edge traversal | 240–480 ms per connected edge |
| Disconnected edges | At most one declared edge |
| Break sparks | One cue, 1–4 particles, 120–240 ms |
| Terminal cues | At most one, only for a fully connected flow |
| Complete flow replay | At most 8,000 ms |

## Future bounded profile

Ambient drift remains independent sparse growth with coordinate changes every 7–11 seconds.

No profile may move labels, displace the full screen, obscure panel text, or make meaning depend on
motion or color. Reduced motion suppresses traversal, sparks, pulses, and glitches.
