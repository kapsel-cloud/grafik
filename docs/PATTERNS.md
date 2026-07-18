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
weight from 1 through 3; weight 2 is most common, weight 1 is next, and weight 3 is rare. The browser
may project weight as subtle stroke emphasis, but HTML states the result independently. Growth keeps
the 120–260 ms segment budget, retraction remains leaf-first at 90–180 ms per segment, density stays
at five live segments or fewer, and direction remains orthogonal with no branching. The one seeded
lateral route choice is the entire chaos budget.

For final dispositions other than `SUCCEEDED`, this profile emits no segment growth or retraction.
Their outcome cues remain separate vertical tracers rather than reusing successful progress.

## Future bounded profiles

These names reserve vocabulary, not implementation scope:

- `FAILED`: brief, subtle, element-local glitch, preferably on the selected control.
- `UNKNOWN`: usually one or two `?` symbols; rare bounded clustering.
- Ambient drift: independent sparse growth with coordinate changes every 7–11 seconds.

No profile may move labels, displace the full screen, obscure panel text, or make meaning depend on
motion or color.
