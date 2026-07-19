use std::collections::BTreeSet;

use crate::{
    generate_scene, simulate_scene, ActionContent, ButtonSize, DiagramForm, EffectPattern,
    FactContent, FinalDisposition, InteractionTrigger, LayoutProfile, NodeGeometry, Point,
    ReceiptContent, Rect, SceneBudgets, SceneError, SceneEvent, SceneNode, SceneNodeContent,
    SceneRecipe, SceneRequest, SceneSimulationInput,
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
            trigger: InteractionTrigger::Action(action.id.clone()),
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
        | SceneEvent::ScanlineSwept { at_ms, .. }
        | SceneEvent::InteractionFinished { at_ms } => *at_ms,
    }
}
