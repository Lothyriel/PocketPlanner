use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{self as jwt};
use jwt::{errors::Error, jwk::Jwk, TokenData};
use lib::infra::UserClaims;
use reqwest::Client;
use serde_json::json;

use crate::{application::ApiState, ResponseResult};

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

pub async fn refresh(Json(params): Json<Params>) -> ResponseResult<()> {
    let client = Client::new();

    let secret = std::env::var("G_CLIENT_SECRET").expect("G_CLIENT_SECRET");

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

async fn insert_claims(state: ApiState, req: &mut Request) -> Result<(), AuthError> {
    let cookie = req
        .headers()
        .get(axum::http::header::COOKIE)
        .ok_or_else(|| AuthError::TokenNotPresent)?;

    let token = cookie.to_str().map_err(|_| AuthError::TokenNotPresent)?;

    let header = jwt::decode_header(token)?;

    let kid = header.kid.ok_or(AuthError::InvalidKid)?;

    let token_data = get_token_data(state, token, kid).await?;

    req.extensions_mut().insert(token_data.claims);

    Ok(())
}

async fn get_token_data(
    state: ApiState,
    token: &str,
    kid: String,
) -> Result<TokenData<UserClaims>, AuthError> {
    let jwkset = state.google_keys.read().await;

    let jwk = match jwkset.find(&kid) {
        Some(k) => k,
        None => {
            drop(jwkset);
            let mut jwkset = state.google_keys.write().await;
            *jwkset = get_google_jwks().await?;

            let jwk = jwkset.find(&kid).ok_or(AuthError::InvalidKid)?;
            return Ok(decode_token_data(token, jwk)?);
        }
    };

    Ok(decode_token_data(token, jwk)?)
}

fn decode_token_data(token: &str, jwk: &Jwk) -> Result<TokenData<UserClaims>, Error> {
    let mut validation = jwt::Validation::new(jwt::Algorithm::RS256);

    validation.set_issuer(&["https://accounts.google.com"]);

    validation.set_audience(&[
        "824653628296-g4ij9785h9c1gkbimm5af42o4l7mket3.apps.googleusercontent.com",
        "824653628296-ahr9jr3aqgr367mul4p359dj4plsl67a.apps.googleusercontent.com",
    ]);

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
