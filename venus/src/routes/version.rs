use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{core::CORE, error::ErrorCode};

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct Versions {
    pub core: String,
    pub venus: String,
}
pub async fn version() -> RouteResult<Versions> {
    let core = CORE.lock()?;
    let v = Versions {
        core: core.version.clone(),
        venus: env!("CARGO_PKG_VERSION").to_string(),
    };
    let res = RouteResponse {
        code: ErrorCode::Normal,
        data: v,
    };

    Ok(Json(res))
}
