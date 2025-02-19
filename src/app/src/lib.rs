use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn render(route: &str) -> String {
    lib::get_template(route).expect("Render")
}
