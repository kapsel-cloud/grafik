use std::collections::BTreeSet;

use crate::{
    generate_scene, simulate_scene, ActionContent, ButtonSize, DiagramForm, EffectPattern,
    FactContent, FinalDisposition, InteractionTrigger, LayoutProfile, NodeGeometry, Point,
    ReceiptContent, Rect, ResultSource, SceneBudgets, SceneError, SceneEvent, SceneNode,
    SceneNodeContent, SceneRecipe, SceneRequest, SceneSimulationInput,
};

#[test]
fn scene_generation_replays_complete_json() -> Result<(), Box<dyn std::error::Error>> {
    let request = valid_request(42);
    let first = generate_scene(&request)?;
    let second = generate_scene(&request)?;

    assert_eq!(first, second);
    assert_eq!(first.to_json()?, second.to_json()?);
    assert_eq!(first.root.children.len(), 8);
    assert!(!first.diagram.nodes.is_empty());
    assert!(!first.diagram.edges.is_empty());
    assert!(!first.effects.is_empty());
    assert_eq!(first.scripts.len(), request.content.actions.len() + 1);
    assert!(first.to_json()?.len() <= usize::try_from(request.budgets.max_json_bytes)?);
    Ok(())
}

#[test]
fn seed_corpus_covers_curated_scene_choices() -> Result<(), SceneError> {
    let mut layouts = [false; 7];
    let mut diagrams = [false; 4];
    let mut button_sizes = [false; 3];
    let mut patterns = [false; 7];

    for seed in 1..=512 {
        let plan = generate_scene(&valid_request(seed))?;
        layouts[layout_index(plan.layout)] = true;
        diagrams[diagram_index(plan.diagram.form)] = true;
        for action in scene_actions(&plan.root)? {
            button_sizes[button_index(action.size)] = true;
        }
        for effect in plan.effects {
            patterns[pattern_index(effect.pattern)] = true;
        }
    }

    assert!(layouts.into_iter().all(|seen| seen));
    assert!(diagrams.into_iter().all(|seen| seen));
    assert!(button_sizes.into_iter().all(|seen| seen));
    assert!(patterns.into_iter().all(|seen| seen));
    Ok(())
}

#[test]
fn diagram_edges_always_reference_generated_nodes() -> Result<(), SceneError> {
    for seed in 1..=64 {
        let plan = generate_scene(&valid_request(seed))?;
        let node_ids = plan
            .diagram
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>();
        let mut edge_ids = BTreeSet::new();
        for edge in &plan.diagram.edges {
            assert!(node_ids.contains(edge.from.as_str()));
            assert!(node_ids.contains(edge.to.as_str()));
            assert!(edge_ids.insert(edge.id.as_str()));
        }
    }
    Ok(())
}

#[test]
fn invalid_content_and_impossible_budgets_fail() {
    let mut zero_seed = valid_request(1);
    zero_seed.seed = 0;
    let mut uneven_table = valid_request(1);
    uneven_table.content.table_rows[0].pop();
    let mut insufficient_effects = valid_request(1);
    insufficient_effects.budgets.max_effects = 2;

    assert_eq!(generate_scene(&zero_seed), Err(SceneError::ZeroSeed));
    assert_eq!(
        generate_scene(&uneven_table),
        Err(SceneError::InvalidContent("table row width"))
    );
    assert_eq!(
        generate_scene(&insufficient_effects),
        Err(SceneError::BudgetExceeded("action and load effects"))
    );
}

#[test]
fn measured_scene_interaction_replays_complete_trace() -> Result<(), Box<dyn std::error::Error>> {
    let plan = generate_scene(&valid_request(73))?;
    let input = SceneSimulationInput {
        geometry: complete_geometry(&plan.root, &plan.diagram.nodes),
        trigger: InteractionTrigger::Load,
        plan,
    };
    let first = simulate_scene(&input)?;
    let second = simulate_scene(&input)?;

    assert_eq!(first, second);
    assert_eq!(first.to_json()?, second.to_json()?);
    assert!(matches!(
        first.events.first(),
        Some(SceneEvent::InteractionStarted { at_ms: 0 })
    ));
    assert!(matches!(
        first.events.last(),
        Some(SceneEvent::InteractionFinished { .. })
    ));
    assert!(first
        .events
        .windows(2)
        .all(|pair| event_time(&pair[0]) <= event_time(&pair[1])));
    Ok(())
}

#[test]
fn every_generated_action_has_a_replayable_local_script() -> Result<(), SceneError> {
    let plan = generate_scene(&valid_request(91))?;
    let geometry = complete_geometry(&plan.root, &plan.diagram.nodes);

    for action in scene_actions(&plan.root)? {
        let trace = simulate_scene(&SceneSimulationInput {
            plan: plan.clone(),
            geometry: geometry.clone(),
            trigger: InteractionTrigger::Action {
                action_id: action.id.clone(),
            },
        })?;
        assert!(trace.events.len() >= 4);
        assert!(trace.events.iter().all(|event| !matches!(
            event,
            SceneEvent::EdgeTraversed { .. } | SceneEvent::PacketTraversed { .. }
        )));
    }
    Ok(())
}

#[test]
fn missing_or_duplicate_geometry_is_rejected() -> Result<(), SceneError> {
    let plan = generate_scene(&valid_request(113))?;
    let mut geometry = complete_geometry(&plan.root, &plan.diagram.nodes);
    geometry.clear();
    let missing = SceneSimulationInput {
        plan: plan.clone(),
        geometry,
        trigger: InteractionTrigger::Load,
    };
    assert_eq!(
        simulate_scene(&missing),
        Err(SceneError::InvalidGeometry("missing required node"))
    );

    let mut geometry = complete_geometry(&plan.root, &plan.diagram.nodes);
    let duplicate = geometry
        .first()
        .cloned()
        .ok_or(SceneError::InvalidGeometry("test geometry"))?;
    geometry.push(duplicate);
    let duplicated = SceneSimulationInput {
        plan,
        geometry,
        trigger: InteractionTrigger::Load,
    };
    assert_eq!(
        simulate_scene(&duplicated),
        Err(SceneError::InvalidGeometry("duplicate node"))
    );
    Ok(())
}

#[test]
fn animated_flow_generates_the_curated_ordered_plan() -> Result<(), SceneError> {
    let plan = generate_scene(&flow_request(424_242))?;
    assert_eq!(plan.recipe, SceneRecipe::AnimatedFlow);
    assert_eq!(plan.layout, LayoutProfile::DiagramLed);
    assert_eq!(plan.diagram.form, DiagramForm::Linear);
    assert_eq!(
        plan.diagram
            .nodes
            .iter()
            .map(|node| node.label.as_str())
            .collect::<Vec<_>>(),
        ["grant", "journal", "provider seam", "observe", "receipt"]
    );
    let flow = plan
        .flow
        .as_ref()
        .ok_or(SceneError::InvalidIdentifier("test flow"))?;
    assert_eq!(flow.edge_phases.len(), 4);
    for (phase, edge) in flow.edge_phases.iter().zip(&plan.diagram.edges) {
        assert_eq!(phase.edge_id, edge.id);
        assert!((240..=480).contains(&phase.duration_ms));
    }
    assert!(plan.effects.is_empty());
    assert!(plan.scripts.is_empty());
    Ok(())
}

#[test]
fn animated_flow_rejects_claimed_linear_topology_drift() -> Result<(), SceneError> {
    let mut plan = generate_scene(&flow_request(80))?;
    plan.diagram.edges[1].from = plan.diagram.nodes[0].id.clone();
    let input = flow_input(
        plan.clone(),
        flow_geometry(&plan),
        FinalDisposition::Succeeded,
        None,
    );
    assert_eq!(
        simulate_scene(&input),
        Err(SceneError::InvalidIdentifier(
            "animated flow linear topology"
        ))
    );
    Ok(())
}

#[test]
fn connected_animated_flow_replays_each_outcome() -> Result<(), Box<dyn std::error::Error>> {
    let plan = generate_scene(&flow_request(81))?;
    let geometry = flow_geometry(&plan);
    for disposition in [
        FinalDisposition::Succeeded,
        FinalDisposition::Failed,
        FinalDisposition::Unknown,
    ] {
        let input = flow_input(plan.clone(), geometry.clone(), disposition, None);
        let first = simulate_scene(&input)?;
        assert_eq!(first.to_json()?, simulate_scene(&input)?.to_json()?);
        assert_connected_edges(&plan, &geometry, &first)?;
        assert_terminal_cue(&plan, &geometry, &first, disposition)?;
    }
    Ok(())
}

#[test]
fn not_attempted_animated_flow_has_only_boundaries() -> Result<(), SceneError> {
    let plan = generate_scene(&flow_request(82))?;
    let trace = simulate_scene(&flow_input(
        plan.clone(),
        flow_geometry(&plan),
        FinalDisposition::NotAttempted,
        None,
    ))?;
    assert_eq!(trace.events.len(), 2);
    assert!(matches!(
        trace.events.first(),
        Some(SceneEvent::InteractionStarted { at_ms: 0 })
    ));
    assert!(matches!(
        trace.events.last(),
        Some(SceneEvent::InteractionFinished { at_ms: 0 })
    ));
    Ok(())
}

#[test]
fn animated_flow_disconnects_each_declared_edge() -> Result<(), SceneError> {
    let plan = generate_scene(&flow_request(83))?;
    let geometry = flow_geometry(&plan);
    for (index, edge) in plan.diagram.edges.iter().enumerate() {
        let trace = simulate_scene(&flow_input(
            plan.clone(),
            geometry.clone(),
            FinalDisposition::Succeeded,
            Some(edge.id.clone()),
        ))?;
        assert_eq!(edge_events(&trace).len(), index);
        let disconnected = trace.events.iter().find_map(|event| match event {
            SceneEvent::FlowDisconnected {
                edge_id,
                point,
                duration_ms,
                sparks,
                ..
            } => Some((edge_id, point, duration_ms, sparks)),
            _ => None,
        });
        let (edge_id, point, duration_ms, sparks) =
            disconnected.ok_or(SceneError::InvalidIdentifier("test disconnect event"))?;
        let source = find_measurement(&geometry, &edge.from)?.outgoing;
        let target = find_measurement(&geometry, &edge.to)?.incoming;
        assert_eq!(edge_id, &edge.id);
        assert_eq!(
            *point,
            Point::new(source.x.midpoint(target.x), source.y.midpoint(target.y))
        );
        assert!((120..=240).contains(duration_ms));
        assert!((1..=4).contains(sparks));
        assert!(trace.events.iter().all(|event| !matches!(
            event,
            SceneEvent::SuccessReinforced { .. } | SceneEvent::BackingGlitched { .. }
        )));
    }
    Ok(())
}

#[test]
fn animated_flow_rejects_unknown_breaks_and_invalid_geometry() -> Result<(), SceneError> {
    let plan = generate_scene(&flow_request(99))?;
    let geometry = flow_geometry(&plan);
    let unknown = flow_input(
        plan.clone(),
        geometry.clone(),
        FinalDisposition::Succeeded,
        Some("missing-edge".to_owned()),
    );
    assert_eq!(
        simulate_scene(&unknown),
        Err(SceneError::InvalidIdentifier("disconnected edge"))
    );

    let invalid_cases = [
        (
            {
                let mut value = geometry.clone();
                value[4].rect = Rect::new(140.0, 35.0, 40.0, 50.0);
                value[4].incoming = Point::new(140.0, 60.0);
                value[4].outgoing = Point::new(180.0, 60.0);
                value
            },
            "animated flow crosses node interior",
        ),
        (
            {
                let mut value = geometry.clone();
                value[1].incoming = Point::new(201.0, 61.0);
                value
            },
            "animated flow boundary port",
        ),
        (
            {
                let mut value = geometry.clone();
                value[1].rect = Rect::new(80.0, 40.0, 80.0, 40.0);
                value[1].incoming = value[0].outgoing;
                value[1].outgoing = Point::new(160.0, 60.0);
                value
            },
            "animated flow zero-length edge",
        ),
        (
            {
                let mut value = geometry.clone();
                value[1].incoming.x = f64::NAN;
                value
            },
            "non-finite node",
        ),
        (
            {
                let mut value = geometry.clone();
                value.remove(0);
                value
            },
            "missing required node",
        ),
        (
            {
                let mut value = geometry.clone();
                value[0].rect = Rect::new(f64::MAX, 40.0, f64::MAX, 40.0);
                value
            },
            "non-finite node",
        ),
        (
            {
                let mut value = geometry;
                let mut stale = value[0].clone();
                stale.id = "stale".to_owned();
                value.push(stale);
                value
            },
            "stale node",
        ),
    ];
    for (invalid_geometry, message) in invalid_cases {
        let invalid = flow_input(
            plan.clone(),
            invalid_geometry,
            FinalDisposition::Unknown,
            None,
        );
        assert_eq!(
            simulate_scene(&invalid),
            Err(SceneError::InvalidGeometry(message))
        );
    }
    Ok(())
}

#[test]
fn animated_flow_serializes_recorded_provenance_and_trigger_state(
) -> Result<(), Box<dyn std::error::Error>> {
    let plan = generate_scene(&flow_request(101))?;
    let edge_id = plan.diagram.edges[2].id.clone();
    let input = SceneSimulationInput {
        geometry: flow_geometry(&plan),
        trigger: InteractionTrigger::Flow {
            result_source: ResultSource::Recorded,
            final_disposition: FinalDisposition::Unknown,
            disconnected_edge: Some(edge_id.clone()),
        },
        plan,
    };
    let json = simulate_scene(&input)?.to_json()?;
    assert!(json.contains("\"result_source\":\"recorded\""));
    assert!(json.contains("\"final_disposition\":\"UNKNOWN\""));
    assert!(json.contains(&format!("\"disconnected_edge\":\"{edge_id}\"")));
    assert!(json.contains("\"kind\":\"flow_disconnected\""));
    Ok(())
}

fn flow_request(seed: u64) -> SceneRequest {
    let mut request = valid_request(seed);
    request.recipe = SceneRecipe::AnimatedFlow;
    request.content.diagram_labels = ["grant", "journal", "provider seam", "observe", "receipt"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    request
}

fn flow_geometry(plan: &crate::ScenePlan) -> Vec<NodeGeometry> {
    plan.diagram
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let x = f64::from(u32::try_from(index).map_or(0, |value| value)) * 200.0;
            NodeGeometry {
                id: node.id.clone(),
                rect: Rect::new(x, 40.0, 80.0, 40.0),
                incoming: Point::new(x, 60.0),
                outgoing: Point::new(x + 80.0, 60.0),
            }
        })
        .collect()
}

fn flow_input(
    plan: crate::ScenePlan,
    geometry: Vec<NodeGeometry>,
    final_disposition: FinalDisposition,
    disconnected_edge: Option<String>,
) -> SceneSimulationInput {
    SceneSimulationInput {
        plan,
        geometry,
        trigger: InteractionTrigger::Flow {
            result_source: ResultSource::Simulated,
            final_disposition,
            disconnected_edge,
        },
    }
}

fn assert_connected_edges(
    plan: &crate::ScenePlan,
    geometry: &[NodeGeometry],
    trace: &crate::SceneTrace,
) -> Result<(), SceneError> {
    let traversed = edge_events(trace);
    let flow = plan
        .flow
        .as_ref()
        .ok_or(SceneError::InvalidIdentifier("test flow"))?;
    assert_eq!(traversed.len(), plan.diagram.edges.len());
    for ((event, edge), phase) in traversed
        .iter()
        .zip(&plan.diagram.edges)
        .zip(&flow.edge_phases)
    {
        let SceneEvent::EdgeTraversed {
            edge_id,
            from,
            to,
            at_ms,
            duration_ms,
        } = event
        else {
            unreachable!("edge_events returns traversals");
        };
        assert_eq!(edge_id, &edge.id);
        assert_eq!((*at_ms, *duration_ms), (phase.at_ms, phase.duration_ms));
        assert_eq!(*from, find_measurement(geometry, &edge.from)?.outgoing);
        assert_eq!(*to, find_measurement(geometry, &edge.to)?.incoming);
    }
    Ok(())
}

fn assert_terminal_cue(
    plan: &crate::ScenePlan,
    geometry: &[NodeGeometry],
    trace: &crate::SceneTrace,
    disposition: FinalDisposition,
) -> Result<(), SceneError> {
    let flow = plan
        .flow
        .as_ref()
        .ok_or(SceneError::InvalidIdentifier("test flow"))?;
    let success = trace.events.iter().find_map(|event| match event {
        SceneEvent::SuccessReinforced {
            target_id, point, ..
        } => Some((target_id, point)),
        _ => None,
    });
    let failure = trace.events.iter().find_map(|event| match event {
        SceneEvent::BackingGlitched {
            target_id,
            duration_ms,
            offset_x,
            offset_y,
            ..
        } => Some((target_id, duration_ms, offset_x, offset_y)),
        _ => None,
    });
    assert_eq!(
        success.is_some(),
        disposition == FinalDisposition::Succeeded
    );
    assert_eq!(failure.is_some(), disposition == FinalDisposition::Failed);
    if let Some((target_id, point)) = success {
        let terminal = find_measurement(geometry, &flow.terminal_id)?;
        assert_eq!(target_id, &flow.terminal_id);
        assert_eq!(
            *point,
            Point::new(terminal.rect.x + 40.0, terminal.rect.y + 20.0)
        );
    }
    if let Some((target_id, duration_ms, offset_x, offset_y)) = failure {
        assert_eq!(target_id, &flow.terminal_id);
        assert!((140..=260).contains(duration_ms));
        assert!((-3..=3).contains(offset_x));
        assert!((-3..=3).contains(offset_y));
    }
    Ok(())
}

fn edge_events(trace: &crate::SceneTrace) -> Vec<&SceneEvent> {
    trace
        .events
        .iter()
        .filter(|event| matches!(event, SceneEvent::EdgeTraversed { .. }))
        .collect()
}

fn find_measurement<'a>(
    geometry: &'a [NodeGeometry],
    id: &str,
) -> Result<&'a NodeGeometry, SceneError> {
    geometry
        .iter()
        .find(|measurement| measurement.id == id)
        .ok_or(SceneError::InvalidGeometry("test geometry"))
}

fn valid_request(seed: u64) -> SceneRequest {
    SceneRequest {
        seed,
        recipe: SceneRecipe::Vivid,
        content: ReceiptContent {
            eyebrow: "Recorded operation · local presentation".to_owned(),
            title: "A bounded receipt scene".to_owned(),
            outcome: FinalDisposition::Succeeded,
            facts: vec![
                FactContent {
                    label: "target".to_owned(),
                    value: "example/receiver".to_owned(),
                },
                FactContent {
                    label: "digest".to_owned(),
                    value: "sha256:0123456789abcdef".to_owned(),
                },
            ],
            evidence: vec![
                "request accepted".to_owned(),
                "receiver facts observed".to_owned(),
                "result classified".to_owned(),
            ],
            table_headers: vec!["stage".to_owned(), "state".to_owned()],
            table_rows: vec![
                vec!["journal".to_owned(), "written".to_owned()],
                vec!["provider".to_owned(), "delivered".to_owned()],
                vec!["receiver".to_owned(), "succeeded".to_owned()],
            ],
            diagram_labels: vec![
                "request".to_owned(),
                "journal".to_owned(),
                "provider".to_owned(),
                "receiver".to_owned(),
                "result".to_owned(),
            ],
            actions: vec![
                ActionContent {
                    key: "replay".to_owned(),
                    label: "Replay".to_owned(),
                },
                ActionContent {
                    key: "inspect".to_owned(),
                    label: "Inspect".to_owned(),
                },
            ],
            menu_items: vec!["Copy URL".to_owned(), "Pin scene".to_owned()],
        },
        budgets: SceneBudgets::default(),
    }
}

fn complete_geometry(root: &SceneNode, diagram_nodes: &[crate::DiagramNode]) -> Vec<NodeGeometry> {
    let mut ids = Vec::new();
    collect_node_ids(root, &mut ids);
    ids.extend(diagram_nodes.iter().map(|node| node.id.clone()));
    ids.into_iter()
        .enumerate()
        .map(|(index, id)| {
            let index = f64::from(u32::try_from(index).map_or(0, |value| value));
            let x = index.mul_add(30.0, 20.0);
            let y = index.mul_add(24.0, 40.0);
            NodeGeometry {
                id,
                rect: Rect::new(x, y, 120.0, 48.0),
                incoming: Point::new(x, y + 24.0),
                outgoing: Point::new(x + 120.0, y + 24.0),
            }
        })
        .collect()
}

fn collect_node_ids(node: &SceneNode, ids: &mut Vec<String>) {
    ids.push(node.id.clone());
    if let SceneNodeContent::Actions { items } = &node.content {
        ids.extend(items.iter().map(|action| action.id.clone()));
    }
    for child in &node.children {
        collect_node_ids(child, ids);
    }
}

fn scene_actions(root: &SceneNode) -> Result<&[crate::SceneAction], SceneError> {
    root.children
        .iter()
        .find_map(|node| match &node.content {
            SceneNodeContent::Actions { items } => Some(items.as_slice()),
            _ => None,
        })
        .ok_or(SceneError::InvalidContent("generated actions"))
}

const fn layout_index(layout: LayoutProfile) -> usize {
    match layout {
        LayoutProfile::Instrument => 0,
        LayoutProfile::Split => 1,
        LayoutProfile::Asymmetric => 2,
        LayoutProfile::ResultLed => 3,
        LayoutProfile::DiagramLed => 4,
        LayoutProfile::Terminal => 5,
        LayoutProfile::Editorial => 6,
    }
}

const fn diagram_index(form: DiagramForm) -> usize {
    match form {
        DiagramForm::Linear => 0,
        DiagramForm::Tree => 1,
        DiagramForm::State => 2,
        DiagramForm::Network => 3,
    }
}

const fn button_index(size: ButtonSize) -> usize {
    match size {
        ButtonSize::Compact => 0,
        ButtonSize::Regular => 1,
        ButtonSize::Prominent => 2,
    }
}

const fn pattern_index(pattern: EffectPattern) -> usize {
    match pattern {
        EffectPattern::Pulse => 0,
        EffectPattern::Glitch => 1,
        EffectPattern::Invert => 2,
        EffectPattern::Fragment => 3,
        EffectPattern::EdgeTraverse => 4,
        EffectPattern::Packet => 5,
        EffectPattern::Scanline => 6,
    }
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
