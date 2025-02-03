use askama::Template;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn render(route: &str) -> String {
    get_template(route).expect("Render")
}

fn get_template(route: &str) -> Result<String, askama::Error> {
    match route {
        "/" => index().render(),
        _ => error().render(),
    }
}

fn index() -> IndexTemplate {
    IndexTemplate {
        text: String::from("Hello from offline-first webassembly PWA"),
    }
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    text: String,
}

fn error() -> ErrorTemplate {
    ErrorTemplate {
        error: String::from("Something went wrong"),
    }
}

#[derive(askama::Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    error: String,
}
