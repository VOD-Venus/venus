use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;
use venus_core::VenusSubscriptor;

use crate::{
    core::global_core,
    error::AppResult,
    utils::{
        jwt::Claims,
        validator::{ValidatedJson, URL_REGEX},
    },
};

use super::RouteResponse;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SubPayload {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(regex(path = *URL_REGEX))]
    pub url: String,
}

/// Add subscription route
///
/// # Errors
///
/// Returns BadRequest if subscription already exists
#[axum::debug_handler]
pub async fn add_subscription(
    claims: Claims,
    ValidatedJson(payload): ValidatedJson<SubPayload>,
) -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<()>> = RouteResponse {
        ..RouteResponse::default()
    };

    let SubPayload { name, url } = payload;
    let core = &mut global_core().await.lock().await;
    core.add_subscription(name, url).await?;
    res.message = Some("ok".into());

    Ok((StatusCode::OK, res))
}

pub fn routes() -> Router {
    Router::new().route("/add", post(add_subscription))
}
