use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../web_src/index")]
pub extern {
    pub fn drawPixel(x: f64, y: f64, isFilled: bool);
    pub fn clear();
}
