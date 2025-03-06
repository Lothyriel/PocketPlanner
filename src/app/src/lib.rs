use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    http::{Method, Request},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub async fn render(req: JsValue) -> String {
    let req: ReqParts = deser(req).expect("Valid request object");

    let method = Method::from_bytes(req.method.as_bytes()).expect("Valid HTTP method");

    let route = req
        .route
        .strip_prefix("/fragments")
        .expect("strip /fragments prefix");

    let body = if let Some(form) = req.form {
        Body::from(form)
    } else {
        Body::empty()
    };

    let req = Request::builder()
        .header("Content-Type", "application/x-www-form-urlencoded")
        .uri(route)
        .method(method)
        .body(body)
        .expect("Valid request");

    let response = lib::fragments::router()
        .oneshot(req)
        .await
        .expect("Valid response");

    let response = response
        .into_body()
        .collect()
        .await
        .expect("Get response body as bytes")
        .to_bytes()
        .to_vec();

    String::from_utf8(response).expect("Valid UTF-8 String")
}

fn deser<T: serde::de::DeserializeOwned>(val: JsValue) -> Result<T> {
    serde_wasm_bindgen::from_value(val).map_err(|e| anyhow!("{e}"))
}

#[derive(serde::Deserialize)]
struct ReqParts {
    method: String,
    route: String,
    form: Option<String>,
}

#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    lib::init_db().expect("Create DB in wasm");

    web_sys::console::log_1(&JsValue::from_str("wasm mod started"));

    sqlite_wasm_rs::export::install_opfs_sahpool(None, true)
        .await
        .expect("OPFS");
}
