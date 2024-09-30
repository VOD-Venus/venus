use std::borrow::Cow;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{consts::VERSION, core::CORE};

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Serialize, Default)]
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
        data: v,
        ..RouteResponse::default()
    };
    Ok(Json(res))
}
