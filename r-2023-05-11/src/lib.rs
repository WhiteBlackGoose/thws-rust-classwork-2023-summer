use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn square(a: i32) -> String {
    format!("<div width=100 align=center>{}</div>", a)
}
