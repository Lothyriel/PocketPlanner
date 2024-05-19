use axum::Json;
use reqwest::Client;
use serde_json::json;

use crate::api::ResponseResult;

#[derive(serde::Deserialize)]
pub struct Params {
    cpf: String,
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    access: String,
    refresh: String,
}

pub async fn handler(Json(params): Json<Params>) -> ResponseResult<Response> {
    let client = Client::new();

    let id = std::env::var("BELVO_ID")?;
    let password = std::env::var("BELVO_PASSWORD")?;
    let belvo_url = std::env::var("BELVO_URL")?;

    let body = json! ({
      "id": id,
      "password": password,
      "scopes": "read_institutions,write_links,read_consents,write_consents,write_consent_callback,delete_consents",
      "credentials_storage": "365d",
      "stale_in": "300d",
      "fetch_resources": ["ACCOUNTS", "TRANSACTIONS", "OWNERS"],
      "widget": {
          "openfinance_feature": "consent_link_creation",
          "callback_urls": {
              "success": "https://pocket-planner.click://success",
              "exit": "https://pocket-planner.click://exit",
              "event": "https://pocket-planner.click://event"
          },
          "consent": {
              "terms_and_conditions_url": "https://www.belvo.com",
              "permissions": ["REGISTER", "ACCOUNTS", "CREDIT_CARDS","CREDIT_OPERATIONS"],
              "identification_info": [{
                  "type": "CPF",
                  "number": params.cpf,
                  "name": params.name
              }]
          }
      }
    });

    let response = client
        .post(belvo_url + "/api/token/")
        .json(&body)
        .send()
        .await?;

    Ok(Json(response.json().await?))
}
