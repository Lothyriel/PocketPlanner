use anyhow::{anyhow, Result};
use askama::Template;
use lib::templates::init_db;
use rusqlite::Connection;
use std::collections::HashMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod fragments;
mod router;

#[wasm_bindgen]
pub async fn render(req: JsValue) -> String {
    let req = deser(req).expect("Valid request object");

    router::render(req).await.unwrap_or_else(render_error)
}

fn render_error(error: anyhow::Error) -> String {
    lib::templates::error(error)
        .render()
        .expect("Render error template")
}

fn connect_db() -> Result<Connection> {
    Ok(Connection::open("opfs-sahpool.db")?)
}

fn deser<T: serde::de::DeserializeOwned>(val: JsValue) -> Result<T> {
    serde_wasm_bindgen::from_value(val).map_err(|e| anyhow!("{e}"))
}

#[derive(serde::Deserialize)]
struct ReqParts {
    method: String,
    route: String,
    form: HashMap<String, String>,
}

impl ReqParts {
    fn strip_route(&self) -> &str {
        self.route
            .strip_prefix("/fragments")
            .expect("/fragments prefix")
    }
}

type Form = HashMap<String, String>;

trait FromFormData {
    fn from(form: Form) -> Result<Self>
    where
        Self: Sized;
}

#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    init_db(&connect_db().expect("Connect DB in wasm")).expect("Create DB in wasm");

    web_sys::console::log_1(&JsValue::from_str("wasm mod started"));

    sqlite_wasm_rs::export::install_opfs_sahpool(None, true)
        .await
        .expect("OPFS");
}
