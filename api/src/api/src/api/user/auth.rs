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
use lib::{Json, infra::UserClaims};
use openidconnect::{Nonce, core::CoreIdToken};
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
    let token = extract_cookie(&req, ACCESS_COOKIE)?;
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

    build_auth_response(state, claims, access_token, refresh_token)
}

pub async fn refresh(State(state): State<ApiState>, req: Request) -> Result<Response, AuthError> {
    let refresh_token = extract_cookie(&req, REFRESH_COOKIE)?;
    let claims = validate_local_token(&state, refresh_token, TokenKind::Refresh)?;

    let user_claims = claims.to_user_claims();
    let access_token = encode_local_token(&state, &user_claims, TokenKind::Access)?;
    let next_refresh_token = encode_local_token(&state, &user_claims, TokenKind::Refresh)?;

    build_auth_response(state, user_claims, access_token, next_refresh_token)
}

fn build_auth_response(
    state: ApiState,
    user_claims: UserClaims,
    access_token: String,
    refresh_token: String,
) -> Result<Response, AuthError> {
    let mut response = Json(user_claims).into_response();

    append_set_cookie(
        &mut response,
        build_cookie(
            ACCESS_COOKIE,
            &access_token,
            ACCESS_COOKIE_PATH,
            &state,
            state.access_ttl,
        ),
    )?;

    append_set_cookie(
        &mut response,
        build_cookie(
            REFRESH_COOKIE,
            &refresh_token,
            REFRESH_COOKIE_PATH,
            &state,
            state.refresh_ttl,
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
    let id_token: CoreIdToken = serde_json::from_value(serde_json::Value::String(token.into()))
        .map_err(|err| AuthError::OpenId(err.to_string()))?;

    let verifier = state.google_client.id_token_verifier();

    let claims = id_token
        .claims(&verifier, |_: Option<&Nonce>| Ok(()))
        .map_err(|err| AuthError::OpenId(err.to_string()))?;

    let raw_claims: GoogleProfileClaims = serde_json::from_value(
        serde_json::to_value(claims).map_err(|err| AuthError::OpenId(err.to_string()))?,
    )
    .map_err(|err| AuthError::OpenId(err.to_string()))?;

    raw_claims.try_into()
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

    let token_claims = AppTokenClaims {
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
) -> Result<AppTokenClaims, AuthError> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::HS256);
    validation.set_issuer(&[&state.jwt_issuer]);
    validation.set_audience(&[&state.jwt_audience]);

    let secret = match expected {
        TokenKind::Access => &state.jwt_access_secret,
        TokenKind::Refresh => &state.jwt_refresh_secret,
    };

    let token_data = jwt::decode::<AppTokenClaims>(
        token,
        &jwt::DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    if token_data.claims.token_type != expected {
        return Err(AuthError::InvalidTokenType);
    }

    Ok(token_data.claims)
}

fn extract_cookie<'a>(req: &'a Request, name: &str) -> Result<&'a str, AuthError> {
    let cookies = req
        .headers()
        .get(COOKIE)
        .and_then(|t| t.to_str().ok())
        .ok_or(AuthError::TokenNotPresent)?;

    cookies
        .split(';')
        .find_map(|part| {
            let trimmed = part.trim();
            trimmed
                .strip_prefix(name)
                .and_then(|value| value.strip_prefix('='))
        })
        .ok_or(AuthError::TokenNotPresent)
}

fn build_cookie(name: &str, token: &str, path: &str, state: &ApiState, max_age: u64) -> String {
    let mut cookie = format!("{name}={token}; HttpOnly; Path={path}; SameSite=Lax");

    cookie.push_str(&format!("; Max-Age={max_age}"));

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

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TokenKind {
    Access,
    Refresh,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AppTokenClaims {
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

impl AppTokenClaims {
    fn to_user_claims(&self) -> UserClaims {
        UserClaims {
            email: self.email.clone(),
            name: self.name.clone(),
            picture: self.picture.clone(),
        }
    }
}

#[derive(serde::Deserialize)]
struct GoogleProfileClaims {
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
}

impl TryFrom<GoogleProfileClaims> for UserClaims {
    type Error = AuthError;

    fn try_from(value: GoogleProfileClaims) -> Result<Self, Self::Error> {
        Ok(Self {
            email: value.email.ok_or(AuthError::MissingClaim("email"))?,
            name: value.name.ok_or(AuthError::MissingClaim("name"))?,
            picture: value.picture.ok_or(AuthError::MissingClaim("picture"))?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Auth token not found on the request")]
    TokenNotPresent,
    #[error("Missing claim on id_token: {0}")]
    MissingClaim(&'static str),
    #[error("OpenID validation failed: {0}")]
    OpenId(String),
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("Invalid cookie header")]
    InvalidCookie,
    #[error("Invalid token: ({0})")]
    JwtValidation(#[from] jwt::errors::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let err = self.to_string();
        tracing::error!("{err}");
        let body = Json(json!({"error":  err}));
        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}
