use std::collections::HashMap;

use axum::{
    body::Body,
    http::{response::Parts, HeaderMap, HeaderValue, Method, Request},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
use web_sys::{Response, ResponseInit};

#[wasm_bindgen]
pub async fn render(method: String, uri: String, form: Option<String>) -> JsResult<Response> {
    let req = build_request(&method, &uri, form)?;

    let res = lib::router().oneshot(req).await?;

    let (parts, body) = res.into_parts();

    let mut body = body.collect().await?.to_bytes().to_vec();

    let options = build_options(parts);

    Response::new_with_opt_u8_array_and_init(Some(body.as_mut()), &options?)
        .map_err(|_| JsError::new(&format!("Error creating response {method} - {uri}")))
}

fn build_options(parts: Parts) -> JsResult<ResponseInit> {
    let options = ResponseInit::new();

    options.set_status(parts.status.as_u16());
    options.set_headers(&serde_wasm_bindgen::to_value(&get_headers(&parts.headers))?);

    Ok(options)
}

fn get_headers(header_map: &HeaderMap<HeaderValue>) -> HashMap<&str, Option<&str>> {
    header_map
        .iter()
        .map(|(name, value)| (name.as_str(), value.to_str().ok()))
        .collect()
}

fn build_request(method: &str, uri: &str, form: Option<String>) -> JsResult<Request<Body>> {
    let method = Method::from_bytes(method.as_bytes())?;

    let body = if let Some(form) = form {
        Body::from(form)
    } else {
        Body::empty()
    };

    let req = Request::builder()
        .header("Content-Type", "application/x-www-form-urlencoded")
        .uri(uri)
        .method(method)
        .body(body)?;

    Ok(req)
}

type JsResult<T> = Result<T, JsError>;

#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    lib::init_db().expect("Create DB in wasm");

    web_sys::console::log_1(&JsValue::from_str("wasm mod started"));

    sqlite_wasm_rs::export::install_opfs_sahpool(None, true)
        .await
        .expect("OPFS");
}
