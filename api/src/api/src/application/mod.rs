use openidconnect::{
    ClientId, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl,
    core::{CoreClient, CoreProviderMetadata},
};

use crate::expect_env;

pub mod extractors;
pub mod model;

#[derive(Clone)]
pub struct ApiState {
    pub google_client: GoogleClient,
    pub secure_env: bool,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_ttl: u64,
    pub refresh_ttl: u64,
}

impl ApiState {
    pub async fn from_env() -> Self {
        let google_client = load_google_auth().await;

        let secure_env = expect_env!("SECURE_ENV") == "true";

        let jwt_access_secret = expect_env!("JWT_ACCESS_SECRET");
        let jwt_refresh_secret = expect_env!("JWT_REFRESH_SECRET");

        let access_ttl = expect_env!("JWT_ACCESS_TTL")
            .parse()
            .expect("numeric value");
        let refresh_ttl = expect_env!("JWT_REFRESH_TTL")
            .parse()
            .expect("numeric value");

        ApiState {
            google_client,
            secure_env,
            jwt_access_secret,
            jwt_refresh_secret,
            jwt_issuer: "pocket-planner-api".to_string(),
            jwt_audience: "pocket-planner-clients".to_string(),
            access_ttl,
            refresh_ttl,
        }
    }
}

pub type GoogleClient = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

async fn load_google_auth() -> GoogleClient {
    let audience = expect_env!("G_CLIENT_ID");

    let issuer =
        IssuerUrl::new("https://accounts.google.com".to_string()).expect("valid issuer url");

    let oidc_http_client = openidconnect::reqwest::ClientBuilder::new()
        .redirect(openidconnect::reqwest::redirect::Policy::none())
        .build()
        .expect("OIDC client should build");

    let google_provider = CoreProviderMetadata::discover_async(issuer, &oidc_http_client)
        .await
        .expect("Failed to discover OpenID configuration");

    CoreClient::from_provider_metadata(google_provider, ClientId::new(audience), None)
}
