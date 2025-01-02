use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{consts::VERSION, core::global_core};

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Versions {
    pub core: Cow<'static, str>,
    pub venus: Cow<'static, str>,
}
pub async fn version() -> RouteResult<Versions> {
    let core = global_core().await.lock().await;
    let v = Versions {
        core: core.version.clone().into(),
        venus: VERSION.into(),
    };
    Ok(RouteResponse {
        data: v,
        ..RouteResponse::default()
    })
}
