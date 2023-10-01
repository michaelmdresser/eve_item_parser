use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub quantity: i64,
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let items = eve_item_parser::parse(input)?;
    let converted: Vec<Item> = items
        .iter()
        .map(|i| Item {
            name: i.name.to_string(),
            quantity: i.quantity,
        })
        .collect();
    return Ok(serde_wasm_bindgen::to_value(&converted)?);
}
