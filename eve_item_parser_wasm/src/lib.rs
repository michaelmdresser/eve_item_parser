use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let items = eve_item_parser::parse(input)?;
    return Ok(serde_wasm_bindgen::to_value(&items)?);
}
