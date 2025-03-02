use anyhow::{anyhow, Result};

use crate::{fragments::*, ReqParts};

pub async fn render(req: ReqParts) -> Result<String> {
    match (req.method.as_str(), req.strip_route()) {
        ("GET", "/transaction") => transaction::view(),
        ("POST", "/transaction/add") => transaction::action(req.form),
        _ => Err(anyhow!("{} - {} route not found", req.method, req.route)),
    }
}
