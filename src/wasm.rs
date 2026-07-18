use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::{simulate, SimulationInput};

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
