use core::fmt;
use serde::{Deserialize, Serialize};

use crate::{FinalDisposition, Point, Rect, ResultSource};

const SCENE_FORMAT_VERSION: u16 = 1;
const ABSOLUTE_MAX_NODES: u8 = 48;
const ABSOLUTE_MAX_DEPTH: u8 = 5;
const ABSOLUTE_MAX_ACTIONS: u8 = 8;
const ABSOLUTE_MAX_DIAGRAM_NODES: u8 = 16;
const ABSOLUTE_MAX_DIAGRAM_EDGES: u8 = 24;
const ABSOLUTE_MAX_EFFECTS: u8 = 12;
const ABSOLUTE_MAX_PHASES: u8 = 4;
const ABSOLUTE_MAX_LIVE_EFFECTS: u8 = 5;
const ABSOLUTE_MAX_FRAGMENTS: u8 = 8;
const ABSOLUTE_MAX_DISPLACEMENT: u8 = 3;
const ABSOLUTE_MAX_PHASE_MS: u16 = 2_400;
const ABSOLUTE_MAX_INTERACTION_MS: u32 = 8_000;
const ABSOLUTE_MAX_JSON_BYTES: u32 = 65_536;
const MAX_TEXT_BYTES: usize = 512;

/// Curated amount of decorative motion and variation in a scene.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneRecipe {
    /// Sparse decoration and the shortest scripts.
    Calm,
    /// A balanced selection of local patterns.
    Balanced,
    /// The highest density allowed by the supplied hard budgets.
    Vivid,
    /// One ordered, directed animated flow with explicit outcome and disconnect state.
    AnimatedFlow,
}

/// Hard caller limits that may only narrow Grafik's absolute budgets.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SceneBudgets {
    /// Maximum nodes in the complete scene tree.
    pub max_nodes: u8,
    /// Maximum scene-tree depth.
    pub max_depth: u8,
    /// Maximum actions in the scene.
    pub max_actions: u8,
    /// Maximum diagram nodes.
    pub max_diagram_nodes: u8,
    /// Maximum diagram edges.
    pub max_diagram_edges: u8,
    /// Maximum effect plans in the scene.
    pub max_effects: u8,
    /// Maximum phases in one effect plan.
    pub max_phases_per_effect: u8,
    /// Maximum simultaneously live effects.
    pub max_live_effects: u8,
    /// Maximum fragments emitted by one effect.
    pub max_fragments: u8,
    /// Maximum decorative displacement in CSS pixels.
    pub max_displacement: u8,
    /// Maximum duration of one phase in milliseconds.
    pub max_phase_ms: u16,
    /// Maximum duration of one interaction in milliseconds.
    pub max_interaction_ms: u32,
    /// Maximum serialized scene-plan size in bytes.
    pub max_json_bytes: u32,
}

impl Default for SceneBudgets {
    fn default() -> Self {
        Self {
            max_nodes: ABSOLUTE_MAX_NODES,
            max_depth: ABSOLUTE_MAX_DEPTH,
            max_actions: ABSOLUTE_MAX_ACTIONS,
            max_diagram_nodes: ABSOLUTE_MAX_DIAGRAM_NODES,
            max_diagram_edges: ABSOLUTE_MAX_DIAGRAM_EDGES,
            max_effects: ABSOLUTE_MAX_EFFECTS,
            max_phases_per_effect: ABSOLUTE_MAX_PHASES,
            max_live_effects: ABSOLUTE_MAX_LIVE_EFFECTS,
            max_fragments: ABSOLUTE_MAX_FRAGMENTS,
            max_displacement: ABSOLUTE_MAX_DISPLACEMENT,
            max_phase_ms: ABSOLUTE_MAX_PHASE_MS,
            max_interaction_ms: ABSOLUTE_MAX_INTERACTION_MS,
            max_json_bytes: ABSOLUTE_MAX_JSON_BYTES,
        }
    }
}

/// One key/value fact displayed in a receipt.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FactContent {
    /// Visible fact label.
    pub label: String,
    /// Visible fact value.
    pub value: String,
}

/// One caller-provided action displayed as a native browser control.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActionContent {
    /// Stable caller key used to derive a generated action ID.
    pub key: String,
    /// Visible accessible action label.
    pub label: String,
}

/// Bounded semantic display content used to generate a receipt scene.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptContent {
    /// Short context line above the title.
    pub eyebrow: String,
    /// Primary receipt title.
    pub title: String,
    /// Canonical result value preserved by Grafik.
    pub outcome: FinalDisposition,
    /// Label/value facts.
    pub facts: Vec<FactContent>,
    /// Ordered evidence list.
    pub evidence: Vec<String>,
    /// Table column labels.
    pub table_headers: Vec<String>,
    /// Table rows, each matching the header count.
    pub table_rows: Vec<Vec<String>>,
    /// Labels used to construct generated diagram topology.
    pub diagram_labels: Vec<String>,
    /// Native actions with deterministic generated IDs.
    pub actions: Vec<ActionContent>,
    /// Native menu item labels.
    pub menu_items: Vec<String>,
}

/// Complete controlled input for renderer-neutral scene generation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SceneRequest {
    /// Nonzero deterministic replay seed.
    pub seed: u64,
    /// Curated variation profile.
    pub recipe: SceneRecipe,
    /// Caller-provided bounded display content.
    pub content: ReceiptContent,
    /// Hard limits that generation must not exceed.
    pub budgets: SceneBudgets,
}

/// Curated responsive receipt arrangement.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutProfile {
    /// Single-column technical instrument.
    Instrument,
    /// Evidence and result split across the field.
    Split,
    /// Asymmetric twelve-column composition.
    Asymmetric,
    /// Result-first composition.
    ResultLed,
    /// Diagram-first composition.
    DiagramLed,
    /// Dense terminal-like receipt.
    Terminal,
    /// Sparse editorial receipt.
    Editorial,
}

/// Stable kind of one scene-tree node.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneNodeKind {
    /// Root scene frame.
    Frame,
    /// Receipt heading group.
    Header,
    /// Label/value fact group.
    Facts,
    /// Ordered evidence list.
    List,
    /// Bounded data table.
    Table,
    /// Diagram host.
    Diagram,
    /// Canonical result group.
    Result,
    /// Native action group.
    Actions,
    /// Native menu group.
    Menu,
}

/// Curated placement role interpreted responsively by a browser adapter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GridRole {
    /// Scene-wide frame.
    Frame,
    /// Leading region.
    Lead,
    /// Primary content region.
    Primary,
    /// Secondary content region.
    Secondary,
    /// Full-width content region.
    Full,
    /// Result emphasis region.
    Outcome,
    /// Trailing action region.
    Footer,
}

/// Semantic visual intent interpreted by a concrete renderer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualRole {
    /// Visually subordinate content.
    Quiet,
    /// Default readable content.
    Standard,
    /// Strong explanatory emphasis.
    Emphasis,
    /// Canonical result emphasis.
    Outcome,
    /// Interactive content.
    Interactive,
}

/// Semantic size of a native action control.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonSize {
    /// Smallest accessible control treatment.
    Compact,
    /// Default control treatment.
    Regular,
    /// Strong primary control treatment.
    Prominent,
}

/// One generated native action description.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SceneAction {
    /// Stable generated action ID.
    pub id: String,
    /// Stable caller key.
    pub key: String,
    /// Visible accessible label.
    pub label: String,
    /// Generated semantic control size.
    pub size: ButtonSize,
}

/// Structured content carried by one scene node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SceneNodeContent {
    /// A node with no direct readable content.
    Empty,
    /// Receipt eyebrow and title.
    Header {
        /// Short context line.
        eyebrow: String,
        /// Primary title.
        title: String,
    },
    /// Label/value facts.
    Facts {
        /// Ordered facts.
        items: Vec<FactContent>,
    },
    /// Ordered evidence strings.
    List {
        /// Ordered list items.
        items: Vec<String>,
    },
    /// Bounded table content.
    Table {
        /// Column labels.
        headers: Vec<String>,
        /// Rows matching the header count.
        rows: Vec<Vec<String>>,
    },
    /// Canonical result value.
    Result {
        /// Preserved final disposition.
        disposition: FinalDisposition,
    },
    /// Generated native actions.
    Actions {
        /// Ordered action descriptions.
        items: Vec<SceneAction>,
    },
    /// Native menu item labels.
    Menu {
        /// Ordered menu items.
        items: Vec<String>,
    },
}

/// One stable node in the logical receipt tree.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SceneNode {
    /// Stable generated node ID.
    pub id: String,
    /// Semantic node kind.
    pub kind: SceneNodeKind,
    /// Responsive grid role.
    pub grid_role: GridRole,
    /// Theme-independent visual role.
    pub visual_role: VisualRole,
    /// Whether decorative effects may target this node.
    pub effect_target: bool,
    /// Structured readable content.
    pub content: SceneNodeContent,
    /// Child nodes in logical reading order.
    pub children: Vec<Self>,
}

/// Generated diagram family.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagramForm {
    /// Linear segmented progress.
    Linear,
    /// Branched functional tree.
    Tree,
    /// Directed state diagram.
    State,
    /// Network with cross-connections.
    Network,
}

/// One stable generated diagram node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiagramNode {
    /// Stable generated diagram-node ID.
    pub id: String,
    /// Visible node label.
    pub label: String,
    /// Whether this node is the canonical terminal result.
    pub terminal: bool,
}

/// One stable directed topology edge.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiagramEdge {
    /// Stable generated edge ID.
    pub id: String,
    /// Source diagram-node ID.
    pub from: String,
    /// Target diagram-node ID.
    pub to: String,
}

/// Complete renderer-neutral diagram topology.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiagramPlan {
    /// Generated diagram family.
    pub form: DiagramForm,
    /// Stable nodes.
    pub nodes: Vec<DiagramNode>,
    /// Directed edges derived from node relationships.
    pub edges: Vec<DiagramEdge>,
}

/// Bounded decorative primitive selected by scene generation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectPattern {
    /// Local decorative backing pulse.
    Pulse,
    /// Local decorative backing glitch.
    Glitch,
    /// Local decorative backing inversion.
    Invert,
    /// Bounded local fragment emission.
    Fragment,
    /// Traversal along one measured diagram edge.
    EdgeTraverse,
    /// Packet traversal along one measured diagram edge.
    Packet,
    /// Local decorative scanline sweep.
    Scanline,
}

/// Stable target of a generated decorative effect.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum EffectTarget {
    /// One stable scene or diagram node.
    Node(String),
    /// One stable diagram edge.
    Edge(String),
}

/// One controlled interval in an effect plan.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EffectPhase {
    /// Start time relative to the interaction in milliseconds.
    pub at_ms: u32,
    /// Phase duration in milliseconds.
    pub duration_ms: u16,
}

/// Seeded bounded parameters carried by an effect plan.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EffectParameters {
    /// Semantic intensity from one through three.
    pub intensity: u8,
    /// Decorative horizontal displacement in CSS pixels.
    pub offset_x: i8,
    /// Decorative vertical displacement in CSS pixels.
    pub offset_y: i8,
    /// Number of local decorative fragments.
    pub fragments: u8,
}

/// One generated decorative pattern and its complete phase plan.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EffectPlan {
    /// Stable generated effect ID.
    pub id: String,
    /// Decorative pattern.
    pub pattern: EffectPattern,
    /// Stable node or edge target.
    pub target: EffectTarget,
    /// Bounded seeded parameters.
    pub parameters: EffectParameters,
    /// Ordered controlled phases.
    pub phases: Vec<EffectPhase>,
}

/// Trigger selecting one deterministic interaction script.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InteractionTrigger {
    /// Scene-load script.
    Load,
    /// Script for one generated native action.
    Action {
        /// Stable generated action identifier.
        action_id: String,
    },
    /// Parameterized replay of one generated animated flow.
    Flow {
        /// Whether the selected result is simulated or intentionally recorded.
        result_source: ResultSource,
        /// Explicit final disposition preserved without reinterpretation.
        final_disposition: FinalDisposition,
        /// At most one declared edge at which traversal stops.
        disconnected_edge: Option<String>,
    },
}

/// One ordered measured-edge traversal phase in an animated flow.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FlowEdgePhase {
    /// Stable declared diagram-edge identifier.
    pub edge_id: String,
    /// Controlled traversal start time in milliseconds.
    pub at_ms: u32,
    /// Controlled traversal duration in milliseconds.
    pub duration_ms: u16,
}

/// Complete deterministic timing and terminal plan for one animated flow.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FlowPlan {
    /// Every declared edge exactly once in topology order.
    pub edge_phases: Vec<FlowEdgePhase>,
    /// Stable terminal diagram-node identifier.
    pub terminal_id: String,
    /// Time at which a connected terminal cue begins.
    pub terminal_at_ms: u32,
    /// Bounded failure-backing duration.
    pub failure_duration_ms: u16,
    /// Bounded horizontal failure-backing displacement.
    pub failure_offset_x: i8,
    /// Bounded vertical failure-backing displacement.
    pub failure_offset_y: i8,
    /// Bounded duration of one break-local spark cue.
    pub disconnect_duration_ms: u16,
    /// Bounded number of break-local sparks.
    pub disconnect_sparks: u8,
}

/// One trigger and the generated effect plans it runs.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionScript {
    /// Trigger selecting this script.
    pub trigger: InteractionTrigger,
    /// Stable effect IDs in deterministic order.
    pub effect_ids: Vec<String>,
}

/// Complete renderer-neutral receipt and diagram description.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScenePlan {
    /// Experimental scene schema version.
    pub format_version: u16,
    /// Seed used for every generated choice.
    pub seed: u64,
    /// Caller-selected recipe.
    pub recipe: SceneRecipe,
    /// Generated curated layout profile.
    pub layout: LayoutProfile,
    /// Logical receipt tree.
    pub root: SceneNode,
    /// Generated diagram topology.
    pub diagram: DiagramPlan,
    /// Complete animated-flow plan, present only for the animated-flow recipe.
    pub flow: Option<FlowPlan>,
    /// Generated decorative effect plans.
    pub effects: Vec<EffectPlan>,
    /// Generated load and action scripts.
    pub scripts: Vec<InteractionScript>,
}

impl ScenePlan {
    /// Serializes this plan for a concrete renderer.
    ///
    /// # Errors
    ///
    /// Returns [`SceneError::Serialization`] if JSON serialization fails.
    pub fn to_json(&self) -> Result<String, SceneError> {
        serde_json::to_string(self).map_err(|error| SceneError::Serialization(error.to_string()))
    }
}

/// Browser-measured geometry for one stable scene or diagram node.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NodeGeometry {
    /// Stable scene or diagram node ID.
    pub id: String,
    /// Measured node rectangle in the shared CSS-pixel field.
    pub rect: Rect,
    /// Selected incoming edge port.
    pub incoming: Point,
    /// Selected outgoing edge port.
    pub outgoing: Point,
}

/// Complete controlled input for one measured scene interaction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SceneSimulationInput {
    /// Generated scene plan being replayed.
    pub plan: ScenePlan,
    /// Browser-measured geometry keyed by stable node ID.
    pub geometry: Vec<NodeGeometry>,
    /// Declared load or action interaction.
    pub trigger: InteractionTrigger,
}

/// One renderer-neutral event in a measured scene interaction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SceneEvent {
    /// Marks the beginning of one interaction.
    InteractionStarted {
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
    },
    /// Marks the beginning of one generated effect phase.
    PhaseStarted {
        /// Stable effect ID.
        effect_id: String,
        /// Zero-based phase index.
        phase_index: u8,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Phase duration in milliseconds.
        duration_ms: u16,
    },
    /// Activates a decorative backing for one node.
    NodeActivated {
        /// Stable target node ID.
        target_id: String,
        /// Measured target center.
        point: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Effect duration in milliseconds.
        duration_ms: u16,
        /// Semantic intensity from one through three.
        intensity: u8,
    },
    /// Glitches a decorative backing without moving readable content.
    BackingGlitched {
        /// Stable target node ID.
        target_id: String,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Effect duration in milliseconds.
        duration_ms: u16,
        /// Horizontal decorative displacement in CSS pixels.
        offset_x: i8,
        /// Vertical decorative displacement in CSS pixels.
        offset_y: i8,
    },
    /// Inverts one decorative backing.
    BackingInverted {
        /// Stable target node ID.
        target_id: String,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Effect duration in milliseconds.
        duration_ms: u16,
    },
    /// Emits bounded local decorative fragments.
    FragmentsEmitted {
        /// Stable target node ID.
        target_id: String,
        /// Measured target center.
        point: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Effect duration in milliseconds.
        duration_ms: u16,
        /// Number of decorative fragments.
        count: u8,
    },
    /// Traverses one measured directed edge.
    EdgeTraversed {
        /// Stable diagram-edge ID.
        edge_id: String,
        /// Measured source port.
        from: Point,
        /// Measured target port.
        to: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Traversal duration in milliseconds.
        duration_ms: u16,
    },
    /// Moves one decorative packet along a measured directed edge.
    PacketTraversed {
        /// Stable diagram-edge ID.
        edge_id: String,
        /// Measured source port.
        from: Point,
        /// Measured target port.
        to: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Traversal duration in milliseconds.
        duration_ms: u16,
    },
    /// Marks one selected edge break and its bounded local spark cue.
    FlowDisconnected {
        /// Stable selected diagram-edge ID.
        edge_id: String,
        /// Measured midpoint of the disconnected edge.
        point: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Spark-cue duration in milliseconds.
        duration_ms: u16,
        /// Number of decorative spark particles.
        sparks: u8,
    },
    /// Reinforces a successful terminal with a semantic approval decoration.
    SuccessReinforced {
        /// Stable terminal diagram-node ID.
        target_id: String,
        /// Measured terminal center.
        point: Point,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
    },
    /// Sweeps one decorative scanline across a measured node.
    ScanlineSwept {
        /// Stable target node ID.
        target_id: String,
        /// Measured target rectangle.
        rect: Rect,
        /// Controlled simulation time in milliseconds.
        at_ms: u32,
        /// Sweep duration in milliseconds.
        duration_ms: u16,
    },
    /// Marks the end of one interaction.
    InteractionFinished {
        /// Final controlled simulation time in milliseconds.
        at_ms: u32,
    },
}

/// Complete replayable renderer-neutral scene interaction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SceneTrace {
    /// Scene format version.
    pub format_version: u16,
    /// Scene replay seed.
    pub seed: u64,
    /// Interaction that produced the trace.
    pub trigger: InteractionTrigger,
    /// Ordered renderer-neutral events.
    pub events: Vec<SceneEvent>,
}

impl SceneTrace {
    /// Serializes this trace for a concrete renderer.
    ///
    /// # Errors
    ///
    /// Returns [`SceneError::Serialization`] if JSON serialization fails.
    pub fn to_json(&self) -> Result<String, SceneError> {
        serde_json::to_string(self).map_err(|error| SceneError::Serialization(error.to_string()))
    }
}

/// Validation, budget, geometry, interaction, or serialization failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SceneError {
    /// Seed zero cannot identify a replayable scene.
    ZeroSeed,
    /// A named content invariant was violated.
    InvalidContent(&'static str),
    /// A named hard budget was invalid or exceeded.
    BudgetExceeded(&'static str),
    /// A stable generated identifier was missing, duplicated, or stale.
    InvalidIdentifier(&'static str),
    /// Browser-measured geometry was missing, duplicated, or invalid.
    InvalidGeometry(&'static str),
    /// The selected interaction was not declared by the plan.
    UnknownInteraction,
    /// A plan or trace could not be encoded as JSON.
    Serialization(String),
}

impl fmt::Display for SceneError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSeed => formatter.write_str("scene seed must be nonzero"),
            Self::InvalidContent(name) => write!(formatter, "invalid scene content: {name}"),
            Self::BudgetExceeded(name) => write!(formatter, "scene budget exceeded: {name}"),
            Self::InvalidIdentifier(name) => write!(formatter, "invalid scene identifier: {name}"),
            Self::InvalidGeometry(name) => write!(formatter, "invalid scene geometry: {name}"),
            Self::UnknownInteraction => formatter.write_str("interaction is not declared by scene"),
            Self::Serialization(message) => {
                write!(formatter, "scene serialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for SceneError {}

/// Generates one complete deterministic renderer-neutral receipt scene.
///
/// # Errors
///
/// Returns [`SceneError`] when content or budgets violate the scene contract.
pub fn generate_scene(request: &SceneRequest) -> Result<ScenePlan, SceneError> {
    validate_request(request)?;
    let mut choice_random = SceneRandom::new(request.seed);
    let mut timing_random = SceneRandom::new(request.seed ^ 0xe703_7ed1_a0b4_28db);
    let animated_flow = request.recipe == SceneRecipe::AnimatedFlow;
    let layout = if animated_flow {
        LayoutProfile::DiagramLed
    } else {
        choice_random.layout()
    };
    let diagram_form = if animated_flow {
        DiagramForm::Linear
    } else {
        choice_random.diagram_form()
    };
    let diagram = generate_diagram(&request.content.diagram_labels, diagram_form);
    let actions = generate_actions(&request.content.actions, &mut choice_random);
    let root = generate_tree(&request.content, &actions, layout);
    let (flow, effects, scripts) = if animated_flow {
        (
            Some(generate_flow_plan(request, &diagram, &mut timing_random)?),
            Vec::new(),
            Vec::new(),
        )
    } else {
        let (effects, scripts) = generate_effects(
            request,
            &root,
            &diagram,
            &actions,
            &mut choice_random,
            &mut timing_random,
        )?;
        (None, effects, scripts)
    };
    let plan = ScenePlan {
        format_version: SCENE_FORMAT_VERSION,
        seed: request.seed,
        recipe: request.recipe,
        layout,
        root,
        diagram,
        flow,
        effects,
        scripts,
    };
    validate_plan(&plan, request.budgets)?;
    let byte_count = plan.to_json()?.len();
    let byte_limit = usize::try_from(request.budgets.max_json_bytes)
        .map_err(|_| SceneError::BudgetExceeded("serialized scene bytes"))?;
    if byte_count > byte_limit {
        return Err(SceneError::BudgetExceeded("serialized scene bytes"));
    }
    Ok(plan)
}

/// Simulates one declared scene interaction against browser-measured geometry.
///
/// # Errors
///
/// Returns [`SceneError`] when the plan, interaction, or geometry violates the scene contract.
pub fn simulate_scene(input: &SceneSimulationInput) -> Result<SceneTrace, SceneError> {
    validate_plan(&input.plan, SceneBudgets::default())?;
    validate_geometry(&input.plan, &input.geometry)?;
    if input.plan.recipe == SceneRecipe::AnimatedFlow {
        return simulate_flow(input);
    }
    if matches!(input.trigger, InteractionTrigger::Flow { .. }) {
        return Err(SceneError::UnknownInteraction);
    }
    let script = input
        .plan
        .scripts
        .iter()
        .find(|script| script.trigger == input.trigger)
        .ok_or(SceneError::UnknownInteraction)?;
    let mut events = vec![SceneEvent::InteractionStarted { at_ms: 0 }];
    let mut finish_ms = 0_u32;
    for effect_id in &script.effect_ids {
        let effect = input
            .plan
            .effects
            .iter()
            .find(|effect| effect.id == *effect_id)
            .ok_or(SceneError::InvalidIdentifier("script effect"))?;
        for (phase_index, phase) in effect.phases.iter().enumerate() {
            let phase_index = u8::try_from(phase_index)
                .map_err(|_| SceneError::BudgetExceeded("phases per effect"))?;
            events.push(SceneEvent::PhaseStarted {
                effect_id: effect.id.clone(),
                phase_index,
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
            push_effect_event(&mut events, effect, *phase, &input.plan, &input.geometry)?;
            finish_ms = finish_ms.max(phase.at_ms + u32::from(phase.duration_ms));
        }
    }
    events.sort_by_key(event_time);
    events.push(SceneEvent::InteractionFinished { at_ms: finish_ms });
    Ok(SceneTrace {
        format_version: input.plan.format_version,
        seed: input.plan.seed,
        trigger: input.trigger.clone(),
        events,
    })
}

fn generate_flow_plan(
    request: &SceneRequest,
    diagram: &DiagramPlan,
    random: &mut SceneRandom,
) -> Result<FlowPlan, SceneError> {
    if request.budgets.max_phase_ms < 240 {
        return Err(SceneError::BudgetExceeded("animated flow phase duration"));
    }
    let mut at_ms = 0_u32;
    let mut edge_phases = Vec::with_capacity(diagram.edges.len());
    for edge in &diagram.edges {
        let duration_ms = random.range_u16(240, request.budgets.max_phase_ms.min(480));
        edge_phases.push(FlowEdgePhase {
            edge_id: edge.id.clone(),
            at_ms,
            duration_ms,
        });
        at_ms = at_ms.saturating_add(u32::from(duration_ms));
    }
    let terminal_id = diagram
        .nodes
        .last()
        .map(|node| node.id.clone())
        .ok_or(SceneError::InvalidContent("animated flow terminal"))?;
    let displacement = i8::try_from(request.budgets.max_displacement).map_or(3, |value| value);
    Ok(FlowPlan {
        edge_phases,
        terminal_id,
        terminal_at_ms: at_ms,
        failure_duration_ms: random.range_u16(140, request.budgets.max_phase_ms.min(260)),
        failure_offset_x: random.signed_range(displacement),
        failure_offset_y: random.signed_range(displacement),
        disconnect_duration_ms: random.range_u16(120, request.budgets.max_phase_ms.min(240)),
        disconnect_sparks: random.range_u8(1, 4),
    })
}

fn simulate_flow(input: &SceneSimulationInput) -> Result<SceneTrace, SceneError> {
    let flow = input
        .plan
        .flow
        .as_ref()
        .ok_or(SceneError::InvalidIdentifier("animated flow plan"))?;
    let InteractionTrigger::Flow {
        result_source: _,
        final_disposition,
        disconnected_edge,
    } = &input.trigger
    else {
        return Err(SceneError::UnknownInteraction);
    };
    let disconnected_index = disconnected_edge
        .as_ref()
        .map(|id| {
            flow.edge_phases
                .iter()
                .position(|phase| phase.edge_id == *id)
                .ok_or(SceneError::InvalidIdentifier("disconnected edge"))
        })
        .transpose()?;
    let mut events = vec![SceneEvent::InteractionStarted { at_ms: 0 }];
    let mut finish_ms = 0_u32;
    if *final_disposition != FinalDisposition::NotAttempted {
        for (index, phase) in flow.edge_phases.iter().enumerate() {
            if disconnected_index.is_some_and(|break_index| index >= break_index) {
                break;
            }
            let (from, to) = measured_edge(&input.plan, &input.geometry, &phase.edge_id)?;
            events.push(SceneEvent::EdgeTraversed {
                edge_id: phase.edge_id.clone(),
                from,
                to,
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
            finish_ms = phase.at_ms + u32::from(phase.duration_ms);
        }
        if let Some(index) = disconnected_index {
            let phase = &flow.edge_phases[index];
            let (from, to) = measured_edge(&input.plan, &input.geometry, &phase.edge_id)?;
            events.push(SceneEvent::FlowDisconnected {
                edge_id: phase.edge_id.clone(),
                point: Point::new(from.x.midpoint(to.x), from.y.midpoint(to.y)),
                at_ms: phase.at_ms,
                duration_ms: flow.disconnect_duration_ms,
                sparks: flow.disconnect_sparks,
            });
            finish_ms = phase.at_ms + u32::from(flow.disconnect_duration_ms);
        } else {
            let terminal = find_geometry(&input.geometry, &flow.terminal_id)?;
            match final_disposition {
                FinalDisposition::Succeeded => {
                    events.push(SceneEvent::SuccessReinforced {
                        target_id: flow.terminal_id.clone(),
                        point: rect_center(terminal.rect),
                        at_ms: flow.terminal_at_ms,
                    });
                    finish_ms = flow.terminal_at_ms;
                }
                FinalDisposition::Failed => {
                    events.push(SceneEvent::BackingGlitched {
                        target_id: flow.terminal_id.clone(),
                        at_ms: flow.terminal_at_ms,
                        duration_ms: flow.failure_duration_ms,
                        offset_x: flow.failure_offset_x,
                        offset_y: flow.failure_offset_y,
                    });
                    finish_ms = flow.terminal_at_ms + u32::from(flow.failure_duration_ms);
                }
                FinalDisposition::Unknown | FinalDisposition::NotAttempted => {}
            }
        }
    }
    events.push(SceneEvent::InteractionFinished { at_ms: finish_ms });
    Ok(SceneTrace {
        format_version: input.plan.format_version,
        seed: input.plan.seed,
        trigger: input.trigger.clone(),
        events,
    })
}

fn validate_request(request: &SceneRequest) -> Result<(), SceneError> {
    if request.seed == 0 {
        return Err(SceneError::ZeroSeed);
    }
    validate_budgets(request.budgets)?;
    validate_text(&request.content.eyebrow, "eyebrow")?;
    validate_text(&request.content.title, "title")?;
    if request.content.facts.is_empty() || request.content.facts.len() > 12 {
        return Err(SceneError::InvalidContent("facts"));
    }
    for fact in &request.content.facts {
        validate_text(&fact.label, "fact label")?;
        validate_text(&fact.value, "fact value")?;
    }
    if request.content.evidence.is_empty() || request.content.evidence.len() > 12 {
        return Err(SceneError::InvalidContent("evidence list"));
    }
    for item in &request.content.evidence {
        validate_text(item, "evidence item")?;
    }
    if request.content.table_headers.is_empty() || request.content.table_headers.len() > 6 {
        return Err(SceneError::InvalidContent("table headers"));
    }
    for header in &request.content.table_headers {
        validate_text(header, "table header")?;
    }
    if request.content.table_rows.is_empty() || request.content.table_rows.len() > 12 {
        return Err(SceneError::InvalidContent("table rows"));
    }
    for row in &request.content.table_rows {
        if row.len() != request.content.table_headers.len() {
            return Err(SceneError::InvalidContent("table row width"));
        }
        for cell in row {
            validate_text(cell, "table cell")?;
        }
    }
    let diagram_count = request.content.diagram_labels.len();
    if !(2..=usize::from(request.budgets.max_diagram_nodes)).contains(&diagram_count) {
        return Err(SceneError::InvalidContent("diagram labels"));
    }
    let worst_case_edges = if request.recipe == SceneRecipe::AnimatedFlow {
        diagram_count.saturating_sub(1)
    } else if diagram_count > 3 {
        diagram_count + 1
    } else {
        diagram_count.saturating_sub(1)
    };
    if worst_case_edges > usize::from(request.budgets.max_diagram_edges) {
        return Err(SceneError::BudgetExceeded("diagram edges"));
    }
    for label in &request.content.diagram_labels {
        validate_text(label, "diagram label")?;
    }
    if request.content.actions.is_empty()
        || request.content.actions.len() > usize::from(request.budgets.max_actions)
    {
        return Err(SceneError::InvalidContent("actions"));
    }
    if request.recipe != SceneRecipe::AnimatedFlow
        && request.content.actions.len() >= usize::from(request.budgets.max_effects)
    {
        return Err(SceneError::BudgetExceeded("action and load effects"));
    }
    if request.content.actions.len() + 9 > usize::from(request.budgets.max_nodes) {
        return Err(SceneError::BudgetExceeded("scene and action nodes"));
    }
    for action in &request.content.actions {
        validate_key(&action.key, "action key")?;
        validate_text(&action.label, "action label")?;
    }
    if request.content.menu_items.is_empty() || request.content.menu_items.len() > 8 {
        return Err(SceneError::InvalidContent("menu items"));
    }
    for item in &request.content.menu_items {
        validate_text(item, "menu item")?;
    }
    Ok(())
}

fn validate_budgets(budgets: SceneBudgets) -> Result<(), SceneError> {
    let valid = budgets.max_nodes >= 9
        && budgets.max_nodes <= ABSOLUTE_MAX_NODES
        && budgets.max_depth >= 2
        && budgets.max_depth <= ABSOLUTE_MAX_DEPTH
        && budgets.max_actions > 0
        && budgets.max_actions <= ABSOLUTE_MAX_ACTIONS
        && budgets.max_diagram_nodes >= 2
        && budgets.max_diagram_nodes <= ABSOLUTE_MAX_DIAGRAM_NODES
        && budgets.max_diagram_edges > 0
        && budgets.max_diagram_edges <= ABSOLUTE_MAX_DIAGRAM_EDGES
        && budgets.max_effects > 0
        && budgets.max_effects <= ABSOLUTE_MAX_EFFECTS
        && budgets.max_phases_per_effect > 0
        && budgets.max_phases_per_effect <= ABSOLUTE_MAX_PHASES
        && budgets.max_live_effects > 0
        && budgets.max_live_effects <= ABSOLUTE_MAX_LIVE_EFFECTS
        && budgets.max_fragments > 0
        && budgets.max_fragments <= ABSOLUTE_MAX_FRAGMENTS
        && budgets.max_displacement <= ABSOLUTE_MAX_DISPLACEMENT
        && budgets.max_phase_ms >= 60
        && budgets.max_phase_ms <= ABSOLUTE_MAX_PHASE_MS
        && budgets.max_interaction_ms >= u32::from(budgets.max_phase_ms)
        && budgets.max_interaction_ms <= ABSOLUTE_MAX_INTERACTION_MS
        && budgets.max_json_bytes > 0
        && budgets.max_json_bytes <= ABSOLUTE_MAX_JSON_BYTES;
    if !valid {
        return Err(SceneError::BudgetExceeded("invalid hard limits"));
    }
    Ok(())
}

fn validate_text(value: &str, name: &'static str) -> Result<(), SceneError> {
    if value.trim().is_empty() || value.len() > MAX_TEXT_BYTES {
        return Err(SceneError::InvalidContent(name));
    }
    Ok(())
}

fn validate_key(value: &str, name: &'static str) -> Result<(), SceneError> {
    validate_text(value, name)?;
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(SceneError::InvalidContent(name));
    }
    Ok(())
}

fn generate_actions(actions: &[ActionContent], random: &mut SceneRandom) -> Vec<SceneAction> {
    actions
        .iter()
        .enumerate()
        .map(|(index, action)| SceneAction {
            id: format!("action-{index}"),
            key: action.key.clone(),
            label: action.label.clone(),
            size: random.button_size(),
        })
        .collect()
}

fn generate_tree(
    content: &ReceiptContent,
    actions: &[SceneAction],
    layout: LayoutProfile,
) -> SceneNode {
    let roles = layout_roles(layout);
    let children = vec![
        scene_node(
            "header",
            SceneNodeKind::Header,
            roles[0],
            VisualRole::Emphasis,
            true,
            SceneNodeContent::Header {
                eyebrow: content.eyebrow.clone(),
                title: content.title.clone(),
            },
        ),
        scene_node(
            "facts",
            SceneNodeKind::Facts,
            roles[1],
            VisualRole::Standard,
            true,
            SceneNodeContent::Facts {
                items: content.facts.clone(),
            },
        ),
        scene_node(
            "evidence",
            SceneNodeKind::List,
            roles[2],
            VisualRole::Quiet,
            true,
            SceneNodeContent::List {
                items: content.evidence.clone(),
            },
        ),
        scene_node(
            "records",
            SceneNodeKind::Table,
            roles[3],
            VisualRole::Standard,
            true,
            SceneNodeContent::Table {
                headers: content.table_headers.clone(),
                rows: content.table_rows.clone(),
            },
        ),
        scene_node(
            "diagram",
            SceneNodeKind::Diagram,
            roles[4],
            VisualRole::Emphasis,
            true,
            SceneNodeContent::Empty,
        ),
        scene_node(
            "result",
            SceneNodeKind::Result,
            roles[5],
            VisualRole::Outcome,
            true,
            SceneNodeContent::Result {
                disposition: content.outcome,
            },
        ),
        scene_node(
            "actions",
            SceneNodeKind::Actions,
            roles[6],
            VisualRole::Interactive,
            true,
            SceneNodeContent::Actions {
                items: actions.to_vec(),
            },
        ),
        scene_node(
            "menu",
            SceneNodeKind::Menu,
            GridRole::Footer,
            VisualRole::Interactive,
            true,
            SceneNodeContent::Menu {
                items: content.menu_items.clone(),
            },
        ),
    ];
    SceneNode {
        id: "scene".to_owned(),
        kind: SceneNodeKind::Frame,
        grid_role: GridRole::Frame,
        visual_role: VisualRole::Standard,
        effect_target: true,
        content: SceneNodeContent::Empty,
        children,
    }
}

fn scene_node(
    id: &str,
    kind: SceneNodeKind,
    grid_role: GridRole,
    visual_role: VisualRole,
    effect_target: bool,
    content: SceneNodeContent,
) -> SceneNode {
    SceneNode {
        id: id.to_owned(),
        kind,
        grid_role,
        visual_role,
        effect_target,
        content,
        children: Vec::new(),
    }
}

fn layout_roles(layout: LayoutProfile) -> [GridRole; 7] {
    match layout {
        LayoutProfile::Instrument | LayoutProfile::Terminal => [
            GridRole::Lead,
            GridRole::Primary,
            GridRole::Primary,
            GridRole::Full,
            GridRole::Full,
            GridRole::Outcome,
            GridRole::Footer,
        ],
        LayoutProfile::Split | LayoutProfile::Asymmetric => [
            GridRole::Lead,
            GridRole::Primary,
            GridRole::Secondary,
            GridRole::Primary,
            GridRole::Secondary,
            GridRole::Outcome,
            GridRole::Footer,
        ],
        LayoutProfile::ResultLed | LayoutProfile::DiagramLed => [
            GridRole::Lead,
            GridRole::Secondary,
            GridRole::Secondary,
            GridRole::Full,
            GridRole::Primary,
            GridRole::Outcome,
            GridRole::Footer,
        ],
        LayoutProfile::Editorial => [
            GridRole::Lead,
            GridRole::Primary,
            GridRole::Secondary,
            GridRole::Full,
            GridRole::Full,
            GridRole::Outcome,
            GridRole::Footer,
        ],
    }
}

fn generate_diagram(labels: &[String], form: DiagramForm) -> DiagramPlan {
    let nodes = labels
        .iter()
        .enumerate()
        .map(|(index, label)| DiagramNode {
            id: format!("diagram-node-{index}"),
            label: label.clone(),
            terminal: index + 1 == labels.len(),
        })
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();
    match form {
        DiagramForm::Linear | DiagramForm::State => {
            for index in 1..nodes.len() {
                pairs.push((index - 1, index));
            }
            if form == DiagramForm::State && nodes.len() > 2 {
                pairs.push((nodes.len() - 1, 0));
            }
        }
        DiagramForm::Tree => {
            for index in 1..nodes.len() {
                pairs.push(((index - 1) / 2, index));
            }
        }
        DiagramForm::Network => {
            for index in 1..nodes.len() {
                pairs.push((index - 1, index));
            }
            if nodes.len() > 3 {
                pairs.push((0, 2));
                pairs.push((1, nodes.len() - 1));
            }
        }
    }
    let edges = pairs
        .into_iter()
        .enumerate()
        .map(|(index, (from, to))| DiagramEdge {
            id: format!("diagram-edge-{index}"),
            from: nodes[from].id.clone(),
            to: nodes[to].id.clone(),
        })
        .collect();
    DiagramPlan { form, nodes, edges }
}

fn generate_effects(
    request: &SceneRequest,
    root: &SceneNode,
    diagram: &DiagramPlan,
    actions: &[SceneAction],
    choice_random: &mut SceneRandom,
    timing_random: &mut SceneRandom,
) -> Result<(Vec<EffectPlan>, Vec<InteractionScript>), SceneError> {
    let mut node_targets = Vec::new();
    collect_effect_targets(root, &mut node_targets);
    node_targets.extend(diagram.nodes.iter().map(|node| node.id.clone()));
    let desired_load_effects = match request.recipe {
        SceneRecipe::Calm => 2,
        SceneRecipe::Balanced => 4,
        SceneRecipe::Vivid => 6,
        SceneRecipe::AnimatedFlow => {
            return Err(SceneError::InvalidContent(
                "animated flow effect generation",
            ));
        }
    };
    let action_effects = actions.len();
    let effect_limit = usize::from(request.budgets.max_effects);
    let load_effects = desired_load_effects.min(effect_limit.saturating_sub(action_effects));
    if load_effects == 0 {
        return Err(SceneError::BudgetExceeded("load effects"));
    }
    let mut effects = Vec::with_capacity(load_effects + action_effects);
    let mut load_ids = Vec::with_capacity(load_effects);
    let mut cursor_ms = 0_u32;
    for index in 0..load_effects {
        let pattern = choice_random.pattern(!diagram.edges.is_empty());
        let target = target_for_pattern(pattern, &node_targets, diagram, choice_random)?;
        let effect = generated_effect(
            format!("effect-load-{index}"),
            pattern,
            target,
            cursor_ms,
            request.budgets,
            timing_random,
        );
        cursor_ms = effect_end(&effect).saturating_add(40);
        load_ids.push(effect.id.clone());
        effects.push(effect);
    }
    let mut scripts = vec![InteractionScript {
        trigger: InteractionTrigger::Load,
        effect_ids: load_ids,
    }];
    for (index, action) in actions.iter().enumerate() {
        let pattern = choice_random.action_pattern();
        let target = EffectTarget::Node(action.id.clone());
        let effect = generated_effect(
            format!("effect-action-{index}"),
            pattern,
            target,
            0,
            request.budgets,
            timing_random,
        );
        scripts.push(InteractionScript {
            trigger: InteractionTrigger::Action {
                action_id: action.id.clone(),
            },
            effect_ids: vec![effect.id.clone()],
        });
        effects.push(effect);
    }
    Ok((effects, scripts))
}

fn generated_effect(
    id: String,
    pattern: EffectPattern,
    target: EffectTarget,
    start_ms: u32,
    budgets: SceneBudgets,
    random: &mut SceneRandom,
) -> EffectPlan {
    let maximum_phases = budgets.max_phases_per_effect.min(3);
    let phase_count = random.range_u8(1, maximum_phases);
    let mut phases = Vec::with_capacity(usize::from(phase_count));
    let mut cursor_ms = start_ms;
    for _ in 0..phase_count {
        let maximum_duration = budgets.max_phase_ms.min(640);
        let minimum_duration = maximum_duration.min(120);
        let duration_ms = random.range_u16(minimum_duration, maximum_duration);
        phases.push(EffectPhase {
            at_ms: cursor_ms,
            duration_ms,
        });
        cursor_ms = cursor_ms.saturating_add(u32::from(duration_ms) + 30);
    }
    let displacement = i8::try_from(budgets.max_displacement).map_or(3, |value| value);
    EffectPlan {
        id,
        pattern,
        target,
        parameters: EffectParameters {
            intensity: random.range_u8(1, 3),
            offset_x: random.signed_range(displacement),
            offset_y: random.signed_range(displacement),
            fragments: random.range_u8(1, budgets.max_fragments),
        },
        phases,
    }
}

fn target_for_pattern(
    pattern: EffectPattern,
    node_targets: &[String],
    diagram: &DiagramPlan,
    random: &mut SceneRandom,
) -> Result<EffectTarget, SceneError> {
    if matches!(pattern, EffectPattern::EdgeTraverse | EffectPattern::Packet) {
        let index = random.index(diagram.edges.len());
        return diagram
            .edges
            .get(index)
            .map(|edge| EffectTarget::Edge(edge.id.clone()))
            .ok_or(SceneError::InvalidContent("diagram edge target"));
    }
    let index = random.index(node_targets.len());
    node_targets
        .get(index)
        .cloned()
        .map(EffectTarget::Node)
        .ok_or(SceneError::InvalidContent("scene node target"))
}

fn collect_effect_targets(node: &SceneNode, targets: &mut Vec<String>) {
    if node.effect_target {
        targets.push(node.id.clone());
    }
    for child in &node.children {
        collect_effect_targets(child, targets);
    }
}

fn effect_end(effect: &EffectPlan) -> u32 {
    effect.phases.iter().fold(0, |end_ms, phase| {
        end_ms.max(phase.at_ms + u32::from(phase.duration_ms))
    })
}

fn validate_plan(plan: &ScenePlan, budgets: SceneBudgets) -> Result<(), SceneError> {
    if plan.format_version != SCENE_FORMAT_VERSION {
        return Err(SceneError::InvalidIdentifier("scene format version"));
    }
    if plan.seed == 0 {
        return Err(SceneError::ZeroSeed);
    }
    validate_budgets(budgets)?;
    let mut node_ids = Vec::new();
    validate_tree(&plan.root, 1, budgets, &mut node_ids)?;
    let mut action_ids = Vec::new();
    collect_action_ids(&plan.root, &mut action_ids)?;
    for action_id in &action_ids {
        if node_ids.contains(action_id) {
            return Err(SceneError::InvalidIdentifier("duplicate action node"));
        }
        node_ids.push(action_id.clone());
    }
    if node_ids.len() > usize::from(budgets.max_nodes) {
        return Err(SceneError::BudgetExceeded("scene and action nodes"));
    }
    let edge_ids = validate_diagram(plan, budgets, &mut node_ids)?;
    if plan.recipe == SceneRecipe::AnimatedFlow {
        return validate_animated_flow_shape(plan, budgets);
    }
    if plan.flow.is_some() {
        return Err(SceneError::InvalidIdentifier(
            "unexpected animated flow plan",
        ));
    }
    if plan.effects.len() > usize::from(budgets.max_effects) {
        return Err(SceneError::BudgetExceeded("effects"));
    }
    let mut effect_ids = Vec::new();
    for effect in &plan.effects {
        validate_effect(effect, &node_ids, &edge_ids, budgets)?;
        if effect_ids.contains(&effect.id) {
            return Err(SceneError::InvalidIdentifier("duplicate effect"));
        }
        effect_ids.push(effect.id.clone());
    }
    let mut triggers = Vec::new();
    for script in &plan.scripts {
        if triggers.contains(&script.trigger) {
            return Err(SceneError::InvalidIdentifier("duplicate interaction"));
        }
        if let InteractionTrigger::Action { action_id } = &script.trigger {
            if !action_ids.contains(action_id) {
                return Err(SceneError::InvalidIdentifier("interaction action"));
            }
        }
        if matches!(script.trigger, InteractionTrigger::Flow { .. }) {
            return Err(SceneError::InvalidIdentifier("receipt flow interaction"));
        }
        triggers.push(script.trigger.clone());
        if script.effect_ids.is_empty()
            || script
                .effect_ids
                .iter()
                .any(|effect_id| !effect_ids.contains(effect_id))
        {
            return Err(SceneError::InvalidIdentifier("script effect"));
        }
        let script_effects = script
            .effect_ids
            .iter()
            .filter_map(|id| plan.effects.iter().find(|effect| effect.id == **id))
            .collect::<Vec<_>>();
        if script_effects
            .iter()
            .fold(0, |end_ms, effect| end_ms.max(effect_end(effect)))
            > budgets.max_interaction_ms
        {
            return Err(SceneError::BudgetExceeded("interaction duration"));
        }
        validate_live_effects(&script_effects, budgets.max_live_effects)?;
    }
    if !triggers.contains(&InteractionTrigger::Load)
        || action_ids.iter().any(|action_id| {
            !triggers.contains(&InteractionTrigger::Action {
                action_id: action_id.clone(),
            })
        })
    {
        return Err(SceneError::InvalidIdentifier("missing interaction"));
    }
    Ok(())
}

fn validate_diagram(
    plan: &ScenePlan,
    budgets: SceneBudgets,
    node_ids: &mut Vec<String>,
) -> Result<Vec<String>, SceneError> {
    if plan.diagram.nodes.len() > usize::from(budgets.max_diagram_nodes) {
        return Err(SceneError::BudgetExceeded("diagram nodes"));
    }
    if plan.diagram.edges.len() > usize::from(budgets.max_diagram_edges) {
        return Err(SceneError::BudgetExceeded("diagram edges"));
    }
    for node in &plan.diagram.nodes {
        if node_ids.contains(&node.id) {
            return Err(SceneError::InvalidIdentifier("duplicate node"));
        }
        node_ids.push(node.id.clone());
    }
    let mut edge_ids = Vec::new();
    for edge in &plan.diagram.edges {
        if edge_ids.contains(&edge.id)
            || !node_ids.contains(&edge.from)
            || !node_ids.contains(&edge.to)
        {
            return Err(SceneError::InvalidIdentifier("diagram edge"));
        }
        edge_ids.push(edge.id.clone());
    }
    Ok(edge_ids)
}

fn validate_animated_flow_shape(plan: &ScenePlan, budgets: SceneBudgets) -> Result<(), SceneError> {
    if plan.layout != LayoutProfile::DiagramLed
        || plan.diagram.form != DiagramForm::Linear
        || !plan.effects.is_empty()
        || !plan.scripts.is_empty()
    {
        return Err(SceneError::InvalidIdentifier("animated flow shape"));
    }
    validate_linear_topology(&plan.diagram)?;
    let flow = plan
        .flow
        .as_ref()
        .ok_or(SceneError::InvalidIdentifier("animated flow plan"))?;
    validate_flow_plan(plan, flow, budgets)
}

fn validate_linear_topology(diagram: &DiagramPlan) -> Result<(), SceneError> {
    if diagram.nodes.len() != diagram.edges.len().saturating_add(1) {
        return Err(SceneError::InvalidIdentifier(
            "animated flow linear topology",
        ));
    }
    for (index, edge) in diagram.edges.iter().enumerate() {
        let source = &diagram.nodes[index];
        let target = &diagram.nodes[index + 1];
        if edge.from != source.id || edge.to != target.id || source.terminal {
            return Err(SceneError::InvalidIdentifier(
                "animated flow linear topology",
            ));
        }
    }
    Ok(())
}

fn validate_flow_plan(
    plan: &ScenePlan,
    flow: &FlowPlan,
    budgets: SceneBudgets,
) -> Result<(), SceneError> {
    if flow.edge_phases.len() != plan.diagram.edges.len() || flow.edge_phases.is_empty() {
        return Err(SceneError::InvalidIdentifier("animated flow edge phases"));
    }
    let mut expected_at_ms = 0_u32;
    for (phase, edge) in flow.edge_phases.iter().zip(&plan.diagram.edges) {
        if phase.edge_id != edge.id || phase.at_ms != expected_at_ms {
            return Err(SceneError::InvalidIdentifier("animated flow edge order"));
        }
        if !(240..=480).contains(&phase.duration_ms) || phase.duration_ms > budgets.max_phase_ms {
            return Err(SceneError::BudgetExceeded("animated flow traversal"));
        }
        expected_at_ms = expected_at_ms.saturating_add(u32::from(phase.duration_ms));
    }
    let terminal = plan
        .diagram
        .nodes
        .last()
        .ok_or(SceneError::InvalidIdentifier("animated flow terminal"))?;
    if flow.terminal_id != terminal.id
        || !terminal.terminal
        || flow.terminal_at_ms != expected_at_ms
    {
        return Err(SceneError::InvalidIdentifier("animated flow terminal"));
    }
    let displacement_limit = i16::from(budgets.max_displacement);
    if !(140..=260).contains(&flow.failure_duration_ms)
        || flow.failure_duration_ms > budgets.max_phase_ms
        || i16::from(flow.failure_offset_x).abs() > displacement_limit
        || i16::from(flow.failure_offset_y).abs() > displacement_limit
        || !(120..=240).contains(&flow.disconnect_duration_ms)
        || flow.disconnect_duration_ms > budgets.max_phase_ms
        || !(1..=4).contains(&flow.disconnect_sparks)
    {
        return Err(SceneError::BudgetExceeded("animated flow cue"));
    }
    let terminal_finish = flow
        .terminal_at_ms
        .saturating_add(u32::from(flow.failure_duration_ms));
    let disconnect_finish = flow.edge_phases.iter().fold(0, |finish, phase| {
        finish.max(
            phase
                .at_ms
                .saturating_add(u32::from(flow.disconnect_duration_ms)),
        )
    });
    if terminal_finish.max(disconnect_finish) > budgets.max_interaction_ms {
        return Err(SceneError::BudgetExceeded(
            "animated flow interaction duration",
        ));
    }
    Ok(())
}

fn validate_tree(
    node: &SceneNode,
    depth: u8,
    budgets: SceneBudgets,
    ids: &mut Vec<String>,
) -> Result<(), SceneError> {
    if depth > budgets.max_depth {
        return Err(SceneError::BudgetExceeded("scene depth"));
    }
    if ids.contains(&node.id) {
        return Err(SceneError::InvalidIdentifier("duplicate scene node"));
    }
    ids.push(node.id.clone());
    if ids.len() > usize::from(budgets.max_nodes) {
        return Err(SceneError::BudgetExceeded("scene nodes"));
    }
    for child in &node.children {
        validate_tree(child, depth + 1, budgets, ids)?;
    }
    Ok(())
}

fn collect_action_ids(node: &SceneNode, ids: &mut Vec<String>) -> Result<(), SceneError> {
    if let SceneNodeContent::Actions { items } = &node.content {
        for action in items {
            if ids.contains(&action.id) {
                return Err(SceneError::InvalidIdentifier("duplicate action node"));
            }
            ids.push(action.id.clone());
        }
    }
    for child in &node.children {
        collect_action_ids(child, ids)?;
    }
    Ok(())
}

fn validate_live_effects(effects: &[&EffectPlan], maximum_live: u8) -> Result<(), SceneError> {
    let intervals = effects
        .iter()
        .flat_map(|effect| {
            effect
                .phases
                .iter()
                .map(|phase| (phase.at_ms, phase.at_ms + u32::from(phase.duration_ms)))
        })
        .collect::<Vec<_>>();
    let exceeds_limit = intervals.iter().any(|(start, _)| {
        intervals
            .iter()
            .filter(|(other_start, other_end)| other_start <= start && start < other_end)
            .count()
            > usize::from(maximum_live)
    });
    if exceeds_limit {
        return Err(SceneError::BudgetExceeded("simultaneously live effects"));
    }
    Ok(())
}

fn validate_effect(
    effect: &EffectPlan,
    node_ids: &[String],
    edge_ids: &[String],
    budgets: SceneBudgets,
) -> Result<(), SceneError> {
    let target_valid = match &effect.target {
        EffectTarget::Node(id) => node_ids.contains(id),
        EffectTarget::Edge(id) => edge_ids.contains(id),
    };
    if !target_valid {
        return Err(SceneError::InvalidIdentifier("effect target"));
    }
    let target_kind_valid = matches!(
        (&effect.pattern, &effect.target),
        (
            EffectPattern::EdgeTraverse | EffectPattern::Packet,
            EffectTarget::Edge(_)
        ) | (
            EffectPattern::Pulse
                | EffectPattern::Glitch
                | EffectPattern::Invert
                | EffectPattern::Fragment
                | EffectPattern::Scanline,
            EffectTarget::Node(_)
        )
    );
    if !target_kind_valid {
        return Err(SceneError::InvalidIdentifier("pattern target kind"));
    }
    if effect.phases.is_empty() || effect.phases.len() > usize::from(budgets.max_phases_per_effect)
    {
        return Err(SceneError::BudgetExceeded("effect phases"));
    }
    for phase in &effect.phases {
        if phase.duration_ms == 0 || phase.duration_ms > budgets.max_phase_ms {
            return Err(SceneError::BudgetExceeded("phase duration"));
        }
    }
    if effect
        .phases
        .windows(2)
        .any(|pair| pair[0].at_ms > pair[1].at_ms)
    {
        return Err(SceneError::InvalidIdentifier("phase order"));
    }
    let displacement_limit = i16::from(budgets.max_displacement);
    if i16::from(effect.parameters.offset_x).abs() > displacement_limit
        || i16::from(effect.parameters.offset_y).abs() > displacement_limit
        || effect.parameters.fragments == 0
        || effect.parameters.fragments > budgets.max_fragments
        || !(1..=3).contains(&effect.parameters.intensity)
    {
        return Err(SceneError::BudgetExceeded("effect parameters"));
    }
    Ok(())
}

fn validate_geometry(plan: &ScenePlan, geometry: &[NodeGeometry]) -> Result<(), SceneError> {
    let mut valid_ids = Vec::new();
    collect_geometry_ids(&plan.root, &mut valid_ids);
    for node in &plan.diagram.nodes {
        push_unique(&mut valid_ids, &node.id);
    }
    let mut required = Vec::new();
    for effect in &plan.effects {
        match &effect.target {
            EffectTarget::Node(id) => push_unique(&mut required, id),
            EffectTarget::Edge(id) => {
                let edge = plan
                    .diagram
                    .edges
                    .iter()
                    .find(|edge| edge.id == *id)
                    .ok_or(SceneError::InvalidIdentifier("effect edge"))?;
                push_unique(&mut required, &edge.from);
                push_unique(&mut required, &edge.to);
            }
        }
    }
    if plan.flow.is_some() {
        for node in &plan.diagram.nodes {
            push_unique(&mut required, &node.id);
        }
    }
    let mut seen = Vec::new();
    for measurement in geometry {
        if seen.contains(&measurement.id) {
            return Err(SceneError::InvalidGeometry("duplicate node"));
        }
        if !valid_ids.contains(&measurement.id) {
            return Err(SceneError::InvalidGeometry("stale node"));
        }
        seen.push(measurement.id.clone());
        if !rect_is_valid(measurement.rect)
            || !point_is_finite(measurement.incoming)
            || !point_is_finite(measurement.outgoing)
        {
            return Err(SceneError::InvalidGeometry("non-finite node"));
        }
    }
    if required.iter().any(|id| !seen.contains(id)) {
        return Err(SceneError::InvalidGeometry("missing required node"));
    }
    if plan.flow.is_some() {
        validate_flow_geometry(plan, geometry)?;
    }
    Ok(())
}

fn validate_flow_geometry(plan: &ScenePlan, geometry: &[NodeGeometry]) -> Result<(), SceneError> {
    for edge in &plan.diagram.edges {
        let source = find_geometry(geometry, &edge.from)?;
        let target = find_geometry(geometry, &edge.to)?;
        if !point_on_rect_boundary(source.rect, source.outgoing)
            || !point_on_rect_boundary(target.rect, target.incoming)
        {
            return Err(SceneError::InvalidGeometry("animated flow boundary port"));
        }
        if points_are_equal(source.outgoing, target.incoming) {
            return Err(SceneError::InvalidGeometry(
                "animated flow zero-length edge",
            ));
        }
        for node in &plan.diagram.nodes {
            let measurement = find_geometry(geometry, &node.id)?;
            if segment_enters_rect(source.outgoing, target.incoming, measurement.rect) {
                return Err(SceneError::InvalidGeometry(
                    "animated flow crosses node interior",
                ));
            }
        }
    }
    Ok(())
}

fn collect_geometry_ids(node: &SceneNode, ids: &mut Vec<String>) {
    push_unique(ids, &node.id);
    if let SceneNodeContent::Actions { items } = &node.content {
        for action in items {
            push_unique(ids, &action.id);
        }
    }
    for child in &node.children {
        collect_geometry_ids(child, ids);
    }
}

fn points_are_equal(left: Point, right: Point) -> bool {
    const EPSILON: f64 = 0.001;
    (left.x - right.x).abs() <= EPSILON && (left.y - right.y).abs() <= EPSILON
}

fn point_on_rect_boundary(rect: Rect, point: Point) -> bool {
    const EPSILON: f64 = 0.001;
    let within_x = point.x + EPSILON >= rect.x && point.x <= rect.x + rect.width + EPSILON;
    let within_y = point.y + EPSILON >= rect.y && point.y <= rect.y + rect.height + EPSILON;
    let on_vertical =
        (point.x - rect.x).abs() <= EPSILON || (point.x - (rect.x + rect.width)).abs() <= EPSILON;
    let on_horizontal =
        (point.y - rect.y).abs() <= EPSILON || (point.y - (rect.y + rect.height)).abs() <= EPSILON;
    within_x && within_y && (on_vertical || on_horizontal)
}

fn segment_enters_rect(from: Point, to: Point, rect: Rect) -> bool {
    const EPSILON: f64 = 0.001;
    let left = rect.x + EPSILON;
    let right = rect.x + rect.width - EPSILON;
    let top = rect.y + EPSILON;
    let bottom = rect.y + rect.height - EPSILON;
    if left >= right || top >= bottom {
        return false;
    }
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let mut minimum = 0.0_f64;
    let mut maximum = 1.0_f64;
    for (p, q) in [
        (-dx, from.x - left),
        (dx, right - from.x),
        (-dy, from.y - top),
        (dy, bottom - from.y),
    ] {
        if p.abs() <= f64::EPSILON {
            if q < 0.0 {
                return false;
            }
        } else {
            let ratio = q / p;
            if p < 0.0 {
                minimum = minimum.max(ratio);
            } else {
                maximum = maximum.min(ratio);
            }
            if minimum > maximum {
                return false;
            }
        }
    }
    minimum <= maximum && maximum >= 0.0 && minimum <= 1.0
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_owned());
    }
}

fn point_is_finite(point: Point) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

fn rect_is_valid(rect: Rect) -> bool {
    rect.x.is_finite()
        && rect.y.is_finite()
        && rect.width.is_finite()
        && rect.height.is_finite()
        && rect.width > 0.0
        && rect.height > 0.0
        && (rect.x + rect.width).is_finite()
        && (rect.y + rect.height).is_finite()
}

fn push_effect_event(
    events: &mut Vec<SceneEvent>,
    effect: &EffectPlan,
    phase: EffectPhase,
    plan: &ScenePlan,
    geometry: &[NodeGeometry],
) -> Result<(), SceneError> {
    match (&effect.pattern, &effect.target) {
        (EffectPattern::Pulse, EffectTarget::Node(id)) => {
            let measurement = find_geometry(geometry, id)?;
            events.push(SceneEvent::NodeActivated {
                target_id: id.clone(),
                point: rect_center(measurement.rect),
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
                intensity: effect.parameters.intensity,
            });
        }
        (EffectPattern::Glitch, EffectTarget::Node(id)) => {
            events.push(SceneEvent::BackingGlitched {
                target_id: id.clone(),
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
                offset_x: effect.parameters.offset_x,
                offset_y: effect.parameters.offset_y,
            });
        }
        (EffectPattern::Invert, EffectTarget::Node(id)) => {
            events.push(SceneEvent::BackingInverted {
                target_id: id.clone(),
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
        }
        (EffectPattern::Fragment, EffectTarget::Node(id)) => {
            let measurement = find_geometry(geometry, id)?;
            events.push(SceneEvent::FragmentsEmitted {
                target_id: id.clone(),
                point: rect_center(measurement.rect),
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
                count: effect.parameters.fragments,
            });
        }
        (EffectPattern::Scanline, EffectTarget::Node(id)) => {
            let measurement = find_geometry(geometry, id)?;
            events.push(SceneEvent::ScanlineSwept {
                target_id: id.clone(),
                rect: measurement.rect,
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
        }
        (EffectPattern::EdgeTraverse, EffectTarget::Edge(id)) => {
            let (from, to) = measured_edge(plan, geometry, id)?;
            events.push(SceneEvent::EdgeTraversed {
                edge_id: id.clone(),
                from,
                to,
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
        }
        (EffectPattern::Packet, EffectTarget::Edge(id)) => {
            let (from, to) = measured_edge(plan, geometry, id)?;
            events.push(SceneEvent::PacketTraversed {
                edge_id: id.clone(),
                from,
                to,
                at_ms: phase.at_ms,
                duration_ms: phase.duration_ms,
            });
        }
        _ => return Err(SceneError::InvalidIdentifier("pattern target kind")),
    }
    Ok(())
}

fn find_geometry<'a>(
    geometry: &'a [NodeGeometry],
    id: &str,
) -> Result<&'a NodeGeometry, SceneError> {
    geometry
        .iter()
        .find(|measurement| measurement.id == id)
        .ok_or(SceneError::InvalidGeometry("missing effect target"))
}

fn measured_edge(
    plan: &ScenePlan,
    geometry: &[NodeGeometry],
    id: &str,
) -> Result<(Point, Point), SceneError> {
    let edge = plan
        .diagram
        .edges
        .iter()
        .find(|edge| edge.id == id)
        .ok_or(SceneError::InvalidIdentifier("diagram edge"))?;
    let from = find_geometry(geometry, &edge.from)?;
    let to = find_geometry(geometry, &edge.to)?;
    Ok((from.outgoing, to.incoming))
}

fn rect_center(rect: Rect) -> Point {
    Point::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0)
}

fn event_time(event: &SceneEvent) -> u32 {
    match event {
        SceneEvent::InteractionStarted { at_ms }
        | SceneEvent::PhaseStarted { at_ms, .. }
        | SceneEvent::NodeActivated { at_ms, .. }
        | SceneEvent::BackingGlitched { at_ms, .. }
        | SceneEvent::BackingInverted { at_ms, .. }
        | SceneEvent::FragmentsEmitted { at_ms, .. }
        | SceneEvent::EdgeTraversed { at_ms, .. }
        | SceneEvent::PacketTraversed { at_ms, .. }
        | SceneEvent::FlowDisconnected { at_ms, .. }
        | SceneEvent::SuccessReinforced { at_ms, .. }
        | SceneEvent::ScanlineSwept { at_ms, .. }
        | SceneEvent::InteractionFinished { at_ms } => *at_ms,
    }
}

struct SceneRandom {
    state: u64,
}

impl SceneRandom {
    const fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0xa076_1d64_78bd_642f,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn index(&mut self, length: usize) -> usize {
        if length == 0 {
            return 0;
        }
        usize::from(self.next_u64().to_le_bytes()[0]) % length
    }

    fn layout(&mut self) -> LayoutProfile {
        match self.index(7) {
            0 => LayoutProfile::Instrument,
            1 => LayoutProfile::Split,
            2 => LayoutProfile::Asymmetric,
            3 => LayoutProfile::ResultLed,
            4 => LayoutProfile::DiagramLed,
            5 => LayoutProfile::Terminal,
            _ => LayoutProfile::Editorial,
        }
    }

    fn diagram_form(&mut self) -> DiagramForm {
        match self.index(4) {
            0 => DiagramForm::Linear,
            1 => DiagramForm::Tree,
            2 => DiagramForm::State,
            _ => DiagramForm::Network,
        }
    }

    fn button_size(&mut self) -> ButtonSize {
        match self.index(3) {
            0 => ButtonSize::Compact,
            1 => ButtonSize::Regular,
            _ => ButtonSize::Prominent,
        }
    }

    fn pattern(&mut self, allow_edge: bool) -> EffectPattern {
        match self.index(if allow_edge { 7 } else { 5 }) {
            0 => EffectPattern::Pulse,
            1 => EffectPattern::Glitch,
            2 => EffectPattern::Invert,
            3 => EffectPattern::Fragment,
            4 => EffectPattern::Scanline,
            5 => EffectPattern::EdgeTraverse,
            _ => EffectPattern::Packet,
        }
    }

    fn action_pattern(&mut self) -> EffectPattern {
        match self.index(4) {
            0 => EffectPattern::Pulse,
            1 => EffectPattern::Glitch,
            2 => EffectPattern::Invert,
            _ => EffectPattern::Fragment,
        }
    }

    fn range_u8(&mut self, minimum: u8, maximum: u8) -> u8 {
        let width = maximum - minimum + 1;
        minimum + self.next_u64().to_le_bytes()[0] % width
    }

    fn range_u16(&mut self, minimum: u16, maximum: u16) -> u16 {
        let width = maximum - minimum + 1;
        let sample = u16::from(self.next_u64().to_le_bytes()[0]);
        minimum + sample % width
    }

    fn signed_range(&mut self, maximum: i8) -> i8 {
        if maximum == 0 {
            return 0;
        }
        let width = u8::try_from(i16::from(maximum) * 2 + 1).map_or(1, |value| value);
        let sample = self.next_u64().to_le_bytes()[0] % width;
        i8::try_from(sample).map_or(0, |value| value) - maximum
    }
}
