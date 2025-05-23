use std::collections::HashMap;

use axum::{
    body::Body,
    http::{response::Parts, HeaderMap, HeaderValue, Method, Request as AxumRequest},
    Extension,
};
use http_body_util::BodyExt;
use lib::infra::{Db, DbState, UserClaims};
use surrealdb::engine::any;
use tower::ServiceExt;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
use web_sys::{Response, ResponseInit};

#[wasm_bindgen]
pub async fn render(req: JsValue) -> JsResult<Response> {
    let req = serde_wasm_bindgen::from_value(req)?;

    let req = build_request(req).await?;

    let db = get_db().await?;

    let res = lib::router(DbState::new(db))
        .layer(auth())
        .oneshot(req)
        .await?;

    let (parts, body) = res.into_parts();

    let mut body = body.collect().await?.to_bytes().to_vec();

    let options = build_options(parts);

    Response::new_with_opt_u8_array_and_init(Some(body.as_mut()), &options?).map_err(to_err)
}

fn auth() -> Extension<UserClaims> {
    let local = "local".to_string();

    let local_claims = UserClaims {
        name: local.clone(),
        email: local.clone(),
        picture: local,
    };

    Extension(local_claims)
}

async fn get_db() -> JsResult<Db> {
    let db = any::connect("indxdb://pp_db").await?;

    db.use_ns("pp").await?;

    Ok(db)
}

#[derive(serde::Deserialize, Debug)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    form: Option<String>,
}

fn build_options(parts: Parts) -> JsResult<ResponseInit> {
    let options = ResponseInit::new();

    options.set_status(parts.status.as_u16());
    options.set_headers(&get_headers(&parts.headers)?);

    Ok(options)
}

fn get_headers(headers: &HeaderMap<HeaderValue>) -> JsResult<JsValue> {
    let headers: HashMap<_, _> = headers
        .iter()
        .map(|(name, value)| (name.as_str(), value.to_str().ok()))
        .collect();

    Ok(serde_wasm_bindgen::to_value(&headers)?)
}

async fn build_request(req: Request) -> JsResult<AxumRequest<Body>> {
    let method = Method::from_bytes(req.method.as_bytes())?;

    let body = if let Some(form) = req.form {
        Body::from(form)
    } else {
        Body::empty()
    };

    let mut builder = AxumRequest::builder().uri(req.url).method(method);

    for (name, value) in req.headers {
        builder = builder.header(name, value);
    }

    Ok(builder.body(body)?)
}

fn to_err(v: JsValue) -> JsError {
    JsError::new(&format!("{:?}", v))
}

type JsResult<T> = Result<T, JsError>;

#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
