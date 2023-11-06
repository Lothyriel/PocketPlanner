use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken as jwt;

#[derive(thiserror::Error, serde::Serialize, Debug)]
pub enum AuthError {
    #[error("(0)")]
    Token(#[from] TokenError),
}

#[derive(thiserror::Error, serde::Serialize, Debug)]
pub enum TokenError {
    #[error("Auth token not found on the request")]
    NotPresent,
    #[error("Invalid KeyId ('kid')")]
    InvalidKid,
    #[error("Invalid token: (0)")]
    Validation(String),
    #[error("Error during certificate retrieval: (0)")]
    IO(String),
}

#[derive(serde::Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthError>)> {
    get_email(cookie_jar, &mut req)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, Json(e)))?;

    Ok(next.run(req).await)
}

async fn get_email<B>(cookie_jar: CookieJar, req: &mut Request<B>) -> Result<(), AuthError> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_owned())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
                .and_then(|value| value.starts_with("Bearer ").then(|| value[7..].to_owned()))
        });

    let token = token.ok_or(TokenError::NotPresent)?;

    let header = jwt::decode_header(token).map_err(|e| TokenError::Validation(e.to_string()))?;

    let kid = header.kid.ok_or(TokenError::InvalidKid)?;

    let jwks = get_google_jwks()
        .await
        .map_err(|e| TokenError::IO(e.to_string()))?;

    let jwk = jwks.find(&kid).ok_or(TokenError::InvalidKid)?;

    let token_data = get_claims(token, jwk).map_err(|e| TokenError::Validation(e.to_string()))?;

    req.extensions_mut().insert(token_data.claims.email);

    Ok(())
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
