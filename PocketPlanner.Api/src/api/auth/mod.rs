use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken as jwt;
use jwt::{errors::Error, jwk::Jwk, TokenData};
use reqwest::Client;
use serde_json::json;

use crate::ResponseResult;

pub async fn auth(
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    get_email(cookie_jar, &mut req).await?;

    Ok(next.run(req).await)
}

#[derive(serde::Deserialize)]
pub struct Params {
    refresh_token: String,
    client_id: String,
}

pub async fn refresh(Json(params): Json<Params>) -> ResponseResult<()> {
    let client = Client::new();

    let secret = std::env::var("G_CLIENT_SECRET")?;

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
        .await?;

    println!("{}", response.text().await?);

    //let body = response.json().await?;

    Ok(Json(()))
}

async fn get_email(cookie_jar: CookieJar, req: &mut Request) -> Result<(), AuthError> {
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

    req.extensions_mut().insert(token_data.claims);

    Ok(())
}

fn get_token_from_headers(req: &Request) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.starts_with("Bearer ").then(|| &value[7..]))
}

fn get_claims(token: &str, jwk: &Jwk) -> Result<TokenData<UserClaims>, Error> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::RS256);

    validation.set_issuer(&["https://accounts.google.com"]);

    validation.set_audience(&[
        "824653628296-g4ij9785h9c1gkbimm5af42o4l7mket3.apps.googleusercontent.com",
        "824653628296-ahr9jr3aqgr367mul4p359dj4plsl67a.apps.googleusercontent.com",
    ]);

    jwt::decode::<UserClaims>(token, &jwt::DecodingKey::from_jwk(jwk)?, &validation)
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

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserClaims {
    pub email: String,
    pub name: String,
    pub picture: String,
}
