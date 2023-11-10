use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken as jwt;
use serde_json::json;

pub async fn auth<B>(
    cookie_jar: CookieJar,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, serde_json::Value)> {
    get_email(cookie_jar, &mut req).await?;

    Ok(next.run(req).await)
}

async fn get_email<B>(cookie_jar: CookieJar, req: &mut Request<B>) -> Result<(), AuthError> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value())
        .or_else(|| get_token_from_headers(req))
        .ok_or(AuthError::TokenNotPresent)?;

    let header = jwt::decode_header(token)?;

    let kid = header.kid.ok_or(AuthError::InvalidKid)?;

    let jwks = get_google_jwks().await?;

    let jwk = jwks.find(&kid).ok_or(AuthError::InvalidKid)?;

    let token_data = get_claims(token, jwk)?;

    req.extensions_mut().insert(token_data.claims.email);

    Ok(())
}

fn get_token_from_headers<B>(req: &Request<B>) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.starts_with("Bearer ").then(|| &value[7..]))
}

fn get_claims(
    token: &str,
    jwk: &jwt::jwk::Jwk,
) -> Result<jwt::TokenData<TokenClaims>, jwt::errors::Error> {
    let mut validation = jwt::Validation::default();

    validation.set_issuer(&["https://accounts.google.com"]);

    validation.set_audience(&[
        "824653628296-g4ij9785h9c1gkbimm5af42o4l7mket3.apps.googleusercontent.com",
    ]);

    jwt::decode::<TokenClaims>(token, &jwt::DecodingKey::from_jwk(jwk)?, &validation)
}

async fn get_google_jwks() -> Result<jwt::jwk::JwkSet, reqwest::Error> {
    let response = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;

    response.json().await
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Auth token not found on the request")]
    TokenNotPresent,
    #[error("Invalid KeyId ('kid') on token")]
    InvalidKid,
    #[error("Invalid token: (0)")]
    JwtValidation(#[from] jwt::errors::Error),
    #[error("Error during certificate retrieval: (0)")]
    IO(#[from] reqwest::Error),
}

impl From<AuthError> for (StatusCode, serde_json::Value) {
    fn from(value: AuthError) -> Self {
        (
            StatusCode::UNAUTHORIZED,
            json!({"error": value.to_string() }),
        )
    }
}

#[derive(serde::Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}
