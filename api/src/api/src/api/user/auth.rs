use axum::{
    extract::{Request, State},
    http::{
        StatusCode,
        header::{COOKIE, HeaderValue, SET_COOKIE},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{self as jwt};
use jwt::{TokenData, errors::Error, jwk::Jwk};
use lib::{Json, infra::UserClaims};
use serde_json::json;

use crate::application::ApiState;

const ACCESS_COOKIE: &str = "access_token";
const REFRESH_COOKIE: &str = "refresh_token";
const ACCESS_COOKIE_PATH: &str = "/api";
const REFRESH_COOKIE_PATH: &str = "/api/user/session/refresh";

pub async fn auth(
    State(state): State<ApiState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let token = extract_cookie(&req, ACCESS_COOKIE).ok_or(AuthError::TokenNotPresent)?;
    let claims = validate_local_token(&state, token, TokenKind::Access)?;
    req.extensions_mut().insert(claims.to_user_claims());
    Ok(next.run(req).await)
}

#[derive(serde::Deserialize)]
pub struct SessionParams {
    token: String,
}

pub async fn login(
    State(state): State<ApiState>,
    Json(params): Json<SessionParams>,
) -> Result<Response, AuthError> {
    let claims = validate_google_token(&state, &params.token).await?;

    let access_token = encode_local_token(&state, &claims, TokenKind::Access)?;
    let refresh_token = encode_local_token(&state, &claims, TokenKind::Refresh)?;

    let mut response = Json(claims).into_response();
    append_set_cookie(
        &mut response,
        build_cookie(
            ACCESS_COOKIE,
            &access_token,
            ACCESS_COOKIE_PATH,
            &state,
            Some(state.access_ttl),
        ),
    )?;
    append_set_cookie(
        &mut response,
        build_cookie(
            REFRESH_COOKIE,
            &refresh_token,
            REFRESH_COOKIE_PATH,
            &state,
            Some(state.refresh_ttl),
        ),
    )?;

    Ok(response)
}

pub async fn refresh(State(state): State<ApiState>, req: Request) -> Result<Response, AuthError> {
    let refresh_token = extract_cookie(&req, REFRESH_COOKIE).ok_or(AuthError::TokenNotPresent)?;
    let claims = validate_local_token(&state, refresh_token, TokenKind::Refresh)?;

    let user_claims = claims.to_user_claims();
    let access_token = encode_local_token(&state, &user_claims, TokenKind::Access)?;
    let next_refresh_token = encode_local_token(&state, &user_claims, TokenKind::Refresh)?;

    let mut response = Json(user_claims).into_response();
    append_set_cookie(
        &mut response,
        build_cookie(
            ACCESS_COOKIE,
            &access_token,
            ACCESS_COOKIE_PATH,
            &state,
            Some(state.access_ttl),
        ),
    )?;
    append_set_cookie(
        &mut response,
        build_cookie(
            REFRESH_COOKIE,
            &next_refresh_token,
            REFRESH_COOKIE_PATH,
            &state,
            Some(state.refresh_ttl),
        ),
    )?;

    Ok(response)
}

pub async fn logout(State(state): State<ApiState>) -> Result<Response, AuthError> {
    let mut response = StatusCode::NO_CONTENT.into_response();
    append_set_cookie(
        &mut response,
        clear_cookie(ACCESS_COOKIE, ACCESS_COOKIE_PATH, &state),
    )?;
    append_set_cookie(
        &mut response,
        clear_cookie(REFRESH_COOKIE, REFRESH_COOKIE_PATH, &state),
    )?;

    Ok(response)
}

async fn validate_google_token(state: &ApiState, token: &str) -> Result<UserClaims, AuthError> {
    let header = jwt::decode_header(token)?;
    let kid = header.kid.ok_or(AuthError::InvalidKid)?;
    let token_data = get_google_token_data(state.clone(), token, kid).await?;
    Ok(token_data.claims)
}

fn encode_local_token(
    state: &ApiState,
    claims: &UserClaims,
    token_kind: TokenKind,
) -> Result<String, AuthError> {
    let now = chrono::Utc::now().timestamp() as u64;
    let ttl = match token_kind {
        TokenKind::Access => state.access_ttl,
        TokenKind::Refresh => state.refresh_ttl,
    };

    let token_claims = LocalTokenClaims {
        sub: claims.email.clone(),
        email: claims.email.clone(),
        name: claims.name.clone(),
        picture: claims.picture.clone(),
        token_type: token_kind,
        iss: state.jwt_issuer.clone(),
        aud: state.jwt_audience.clone(),
        iat: now,
        exp: now + ttl,
    };

    let secret = match token_kind {
        TokenKind::Access => &state.jwt_access_secret,
        TokenKind::Refresh => &state.jwt_refresh_secret,
    };

    Ok(jwt::encode(
        &jwt::Header::new(jwt::Algorithm::HS256),
        &token_claims,
        &jwt::EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

fn validate_local_token(
    state: &ApiState,
    token: &str,
    expected: TokenKind,
) -> Result<LocalTokenClaims, AuthError> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::HS256);
    validation.set_issuer(&[&state.jwt_issuer]);
    validation.set_audience(&[&state.jwt_audience]);

    let secret = match expected {
        TokenKind::Access => &state.jwt_access_secret,
        TokenKind::Refresh => &state.jwt_refresh_secret,
    };

    let token_data = jwt::decode::<LocalTokenClaims>(
        token,
        &jwt::DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    if token_data.claims.token_type != expected {
        return Err(AuthError::InvalidTokenType);
    }

    Ok(token_data.claims)
}

fn extract_cookie<'a>(req: &'a Request, name: &str) -> Option<&'a str> {
    let cookies = req.headers().get(COOKIE)?.to_str().ok()?;

    cookies.split(';').find_map(|part| {
        let trimmed = part.trim();
        trimmed
            .strip_prefix(name)
            .and_then(|value| value.strip_prefix('='))
    })
}

fn build_cookie(
    name: &str,
    token: &str,
    path: &str,
    state: &ApiState,
    max_age_seconds: Option<u64>,
) -> String {
    let mut cookie = format!("{name}={token}; HttpOnly; Path={path}; SameSite=Lax");

    if let Some(max_age) = max_age_seconds {
        cookie.push_str(&format!("; Max-Age={max_age}"));
    }

    if state.secure_env {
        cookie.push_str("; Secure");
    }

    cookie
}

fn clear_cookie(name: &str, path: &str, state: &ApiState) -> String {
    let mut cookie = format!("{name}=; HttpOnly; Path={path}; SameSite=Lax; Max-Age=0");
    if state.secure_env {
        cookie.push_str("; Secure");
    }
    cookie
}

fn append_set_cookie(response: &mut Response, cookie: String) -> Result<(), AuthError> {
    let header = HeaderValue::from_str(&cookie).map_err(|_| AuthError::InvalidCookie)?;
    response.headers_mut().append(SET_COOKIE, header);
    Ok(())
}

async fn get_google_token_data(
    state: ApiState,
    token: &str,
    kid: String,
) -> Result<TokenData<UserClaims>, AuthError> {
    let jwkset = state.google_keys.read().await;

    let audience = &state.audience;

    let jwk = match jwkset.find(&kid) {
        Some(k) => k,
        None => {
            drop(jwkset);
            let mut jwkset = state.google_keys.write().await;
            *jwkset = get_google_jwks().await?;

            let jwk = jwkset.find(&kid).ok_or(AuthError::InvalidKid)?;
            return Ok(decode_google_token_data(token, jwk, audience)?);
        }
    };

    Ok(decode_google_token_data(token, jwk, audience)?)
}

fn decode_google_token_data(
    token: &str,
    jwk: &Jwk,
    audience: &str,
) -> Result<TokenData<UserClaims>, Error> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::RS256);
    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);
    validation.set_audience(&[audience]);

    jwt::decode::<UserClaims>(token, &jwt::DecodingKey::from_jwk(jwk)?, &validation)
}

pub async fn get_google_jwks() -> Result<jwt::jwk::JwkSet, reqwest::Error> {
    let response = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;
    response.json().await
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TokenKind {
    Access,
    Refresh,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LocalTokenClaims {
    sub: String,
    email: String,
    name: String,
    picture: String,
    token_type: TokenKind,
    iss: String,
    aud: String,
    iat: u64,
    exp: u64,
}

impl LocalTokenClaims {
    fn to_user_claims(&self) -> UserClaims {
        UserClaims {
            email: self.email.clone(),
            name: self.name.clone(),
            picture: self.picture.clone(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Auth token not found on the request")]
    TokenNotPresent,
    #[error("Invalid KeyId ('kid') on token")]
    InvalidKid,
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("Invalid cookie header")]
    InvalidCookie,
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
