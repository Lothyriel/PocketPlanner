pub fn content() -> ContentTemplate {
    ContentTemplate {
        text: String::from("Offline-first webassembly PWA"),
    }
}

#[derive(askama::Template)]
#[template(path = "content.html")]
pub struct ContentTemplate {
    text: String,
}

pub fn error() -> ErrorTemplate {
    ErrorTemplate {
        error: String::from("Something went wrong"),
    }
}

#[derive(askama::Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: String,
}
