use axum::{routing, Json, Router};
use serde_json::{json, Value};

pub fn asset_links_router() -> Router {
    Router::new().route("/assetlinks.json", routing::get(handler))
}

async fn handler() -> Json<Value> {
    Json(json!([{
      "relation": ["delegate_permission/common.handle_all_urls"],
      "target": {
        "namespace": "android_app",
        "package_name": "com.example.pocket_planner_front",
        "sha256_cert_fingerprints": ["29:6F:64:43:1F:17:B8:45:87:C3:AA:97:3B:6D:CD:EC:DB:93:1A:BA"]
      }
    }]))
}
