use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use venus_core::{config::types::Subscription, VenusSubscriptor};

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
    _claims: Claims,
    ValidatedJson(payload): ValidatedJson<SubPayload>,
) -> AppResult<impl IntoResponse> {
    let SubPayload { name, url } = payload;
    let core = &mut global_core().await.lock().await;
    core.add_subscription(name, url).await?;

    let res: RouteResponse<Option<()>> = RouteResponse {
        message: Some("ok".into()),
        ..RouteResponse::default()
    };

    Ok((StatusCode::OK, res))
}

pub async fn subscriptions(_claims: Claims) -> AppResult<impl IntoResponse> {
    let core = &global_core().await.lock().await;
    let res: RouteResponse<Option<Vec<Subscription>>> = RouteResponse {
        message: Some("ok".into()),
        data: Some(core.config.venus.subscriptions.clone()),
        ..RouteResponse::default()
    };

    Ok((StatusCode::OK, res))
}

pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_subscription))
        .route("/list", get(subscriptions))
}
