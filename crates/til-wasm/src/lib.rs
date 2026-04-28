use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use til::timeline::Timeline;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_timeline(bytes: &[u8]) -> Result<JsValue, JsError> {
    let timeline = Timeline::from_bytes(bytes).map_err(to_js_error)?;
    let serializer = Serializer::new().serialize_maps_as_objects(true);
    timeline.serialize(&serializer).map_err(to_js_error)
}

fn to_js_error<E: std::fmt::Display>(err: E) -> JsError {
    JsError::new(&err.to_string())
}
