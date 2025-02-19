use wasm_bindgen::prelude::wasm_bindgen;

mod views;

#[wasm_bindgen]
pub fn render(route: &str) -> String {
    views::get_template(route).expect("Render")
}
