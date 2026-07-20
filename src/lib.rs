//! Seeded renderer-neutral scene generation and simulation for Grafik living diagrams.
//!
//! [`generate_scene`] produces one bounded scene plan from explicit content, recipe, seed, and
//! budgets. [`simulate_scene`] accepts that plan plus browser-measured geometry and returns one
//! complete deterministic trace. The implementation performs no I/O, reads no ambient randomness
//! or clock, and has no knowledge of the DOM.

use serde::{Deserialize, Serialize};

mod scene;
pub use scene::{
    generate_scene, simulate_scene, ActionContent, ButtonSize, DiagramEdge, DiagramForm,
    DiagramNode, DiagramPlan, EffectParameters, EffectPattern, EffectPhase, EffectPlan,
    EffectTarget, FactContent, FlowEdgePhase, FlowPlan, GridRole, InteractionScript,
    InteractionTrigger, LayoutProfile, NodeGeometry, ReceiptContent, SceneAction, SceneBudgets,
    SceneError, SceneEvent, SceneNode, SceneNodeContent, SceneNodeKind, ScenePlan, SceneRecipe,
    SceneRequest, SceneSimulationInput, SceneTrace, VisualRole,
};

#[cfg(test)]
mod scene_tests;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::{grafik_scene, grafik_scene_trace};

/// A point in the browser's CSS-pixel coordinate space.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f64,
    /// Vertical coordinate.
    pub y: f64,
}

impl Point {
    /// Creates a point from CSS-pixel coordinates.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A rectangular browser-measured region.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rect {
    /// Left coordinate.
    pub x: f64,
    /// Top coordinate.
    pub y: f64,
    /// Width in CSS pixels.
    pub width: f64,
    /// Height in CSS pixels.
    pub height: f64,
}

impl Rect {
    /// Creates a rectangle from CSS-pixel coordinates.
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Provenance of a bounded final result supplied to an animated flow.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultSource {
    /// A generated result that performed no infrastructure work.
    Simulated,
    /// An intentionally published result captured from an earlier experiment run.
    Recorded,
}

/// Final disposition of one bounded operation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinalDisposition {
    /// Local rejection before any receiver attempt; not a receiver result.
    NotAttempted,
    /// Receiver facts established the defined successful outcome.
    Succeeded,
    /// Receiver facts established the defined failed outcome.
    Failed,
    /// Bounded observation established neither success nor failure.
    Unknown,
}
