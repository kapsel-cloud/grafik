use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::{
    generate_scene, simulate, simulate_scene, SceneRequest, SceneSimulationInput, SimulationInput,
};

/// Generates one complete renderer-neutral scene plan from controlled JSON.
///
/// # Errors
///
/// Returns a JavaScript error when request JSON is malformed, content or budgets are invalid, or
/// the generated plan cannot be serialized.
#[wasm_bindgen]
pub fn grafik_scene(request_json: &str) -> Result<String, JsValue> {
    let request = serde_json::from_str::<SceneRequest>(request_json)
        .map_err(|error| JsValue::from_str(&format!("invalid scene request: {error}")))?;
    let plan = generate_scene(&request).map_err(|error| JsValue::from_str(&error.to_string()))?;
    plan.to_json()
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

/// Simulates one declared scene interaction against controlled browser geometry.
///
/// # Errors
///
/// Returns a JavaScript error when input JSON is malformed, the plan or geometry is invalid, or the
/// generated trace cannot be serialized.
#[wasm_bindgen]
pub fn grafik_scene_trace(input_json: &str) -> Result<String, JsValue> {
    let input = serde_json::from_str::<SceneSimulationInput>(input_json)
        .map_err(|error| JsValue::from_str(&format!("invalid scene simulation input: {error}")))?;
    let trace = simulate_scene(&input).map_err(|error| JsValue::from_str(&error.to_string()))?;
    trace
        .to_json()
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

/// Parses a controlled semantic result and browser geometry into one complete spatial trace.
///
/// # Errors
///
/// Returns a JavaScript error when input JSON is malformed, geometry violates the tracer contract,
/// or the output trace cannot be serialized.
#[wasm_bindgen]
pub fn grafik_trace(input_json: &str) -> Result<String, JsValue> {
    let input = serde_json::from_str::<SimulationInput>(input_json)
        .map_err(|error| JsValue::from_str(&format!("invalid simulation input: {error}")))?;
    let trace = simulate(input).map_err(|error| JsValue::from_str(&error.to_string()))?;
    trace
        .to_json()
        .map_err(|error| JsValue::from_str(&error.to_string()))
}
