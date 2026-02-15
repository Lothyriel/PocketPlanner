use axum::{
    extract::{Request, State},
    http::StatusCode,
    http::header::{COOKIE, SET_COOKIE},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{self as jwt};
use jwt::{TokenData, errors::Error, jwk::Jwk};
use lib::{Json, infra::UserClaims};
use reqwest::Client;
use serde_json::json;

use crate::{application::ApiState, expect_env};

pub async fn auth(
    State(state): State<ApiState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    insert_claims(state, &mut req).await?;

    Ok(next.run(req).await)
}

#[derive(serde::Deserialize)]
pub struct Params {
    refresh_token: String,
    client_id: String,
}

pub async fn refresh(Json(params): Json<Params>) -> String {
    let client = Client::new();

    let secret = expect_env!("G_CLIENT_SECRET");

    let body = json! ({
        "client_id": params.client_id,
        "refresh_token": params.refresh_token,
        "client_secret": secret,
        "grant_type": "refresh_token"
    });

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .json(&body)
        .send()
        .await
        .expect("Google is down");

    // todo: these expects are ugly
    let body = response.text().await.expect("Should have text");

    tracing::warn!("Google refresh token: {}", body);

    body
}

#[derive(serde::Deserialize)]
pub struct SessionParams {
    token: String,
}

pub async fn session(
    State(state): State<ApiState>,
    Json(params): Json<SessionParams>,
) -> Result<impl IntoResponse, AuthError> {
    let claims = validate_token(&state, &params.token).await?;
    let cookie = build_cookie(&params.token, &state);

    Ok(([(SET_COOKIE, cookie)], Json(claims)))
}

pub async fn clear_session(State(state): State<ApiState>) -> impl IntoResponse {
    let cookie = clear_cookie(&state);
    ([(SET_COOKIE, cookie)], StatusCode::NO_CONTENT)
}

async fn insert_claims(state: ApiState, req: &mut Request) -> Result<(), AuthError> {
    let token = extract_token(req)?;
    let claims = validate_token(&state, token).await?;
    req.extensions_mut().insert(claims);

    Ok(())
}

async fn validate_token(state: &ApiState, token: &str) -> Result<UserClaims, AuthError> {
    let header = jwt::decode_header(token)?;
    let kid = header.kid.ok_or(AuthError::InvalidKid)?;
    let token_data = get_token_data(state.clone(), token, kid).await?;
    Ok(token_data.claims)
}

fn extract_token(req: &Request) -> Result<&str, AuthError> {
    if let Some(cookie) = req.headers().get(COOKIE) {
        let cookie_str = cookie.to_str().map_err(|_| AuthError::TokenNotPresent)?;
        for part in cookie_str.split(';') {
            let trimmed = part.trim();
            if let Some(token) = trimmed.strip_prefix("id_token=") {
                return Ok(token);
            }
        }
        return Ok(cookie_str);
    }

    Err(AuthError::TokenNotPresent)
}

fn build_cookie(token: &str, state: &ApiState) -> String {
    let mut cookie = format!("id_token={}; HttpOnly; Path=/; SameSite=Lax", token);
    if state.secure_env {
        cookie.push_str("; Secure");
    }
    cookie
}

fn clear_cookie(state: &ApiState) -> String {
    let mut cookie = "id_token=; HttpOnly; Path=/; SameSite=Lax; Max-Age=0".to_string();
    if state.secure_env {
        cookie.push_str("; Secure");
    }
    cookie
}

async fn get_token_data(
    state: ApiState,
    token: &str,
    kid: String,
) -> Result<TokenData<UserClaims>, AuthError> {
    let jwkset = state.google_keys.read().await;

    let audiences = &state.audiences;

    let jwk = match jwkset.find(&kid) {
        Some(k) => k,
        None => {
            drop(jwkset);
            let mut jwkset = state.google_keys.write().await;
            *jwkset = get_google_jwks().await?;

            let jwk = jwkset.find(&kid).ok_or(AuthError::InvalidKid)?;
            return Ok(decode_token_data(token, jwk, audiences)?);
        }
    };

    Ok(decode_token_data(token, jwk, audiences)?)
}

fn decode_token_data(
    token: &str,
    jwk: &Jwk,
    audiences: &[String],
) -> Result<TokenData<UserClaims>, Error> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::RS256);

    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);

    validation.set_audience(audiences);

    jwt::decode::<UserClaims>(token, &jwt::DecodingKey::from_jwk(jwk)?, &validation)
}

pub async fn get_google_jwks() -> Result<jwt::jwk::JwkSet, reqwest::Error> {
    let response = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;

    response.json().await
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Auth token not found on the request")]
    TokenNotPresent,
    #[error("Invalid KeyId ('kid') on token")]
    InvalidKid,
    #[error("Invalid token: ({0})")]
    JwtValidation(#[from] jwt::errors::Error),
    #[error("Error during certificate retrieval: ({0})")]
    IO(#[from] reqwest::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({"error": self.to_string() }));

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}
