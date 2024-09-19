use std::borrow::Cow;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{consts::VERSION, core::CORE, error::ErrorCode};

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct Versions {
    pub core: Cow<'static, str>,
    pub venus: Cow<'static, str>,
}
pub async fn version() -> RouteResult<Versions> {
    let core = CORE.lock()?;
    let v = Versions {
        core: core.version.clone().into(),
        venus: VERSION.into(),
    };
    let res = RouteResponse {
        code: ErrorCode::Normal,
        message: None,
        data: v,
    };

    Ok(Json(res))
}
