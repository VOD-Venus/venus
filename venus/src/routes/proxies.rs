use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;
use venus_core::config::types::Subscription;

use crate::{
    core::CORE,
    error::AppResult,
    utils::validator::{ValidatedJson, URL_REGEX},
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
pub async fn add_subscription(
    ValidatedJson(payload): ValidatedJson<SubPayload>,
) -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<Subscription>> = RouteResponse {
        ..RouteResponse::default()
    };

    let SubPayload { name, url } = payload;

    {
        let subscriptions = &mut CORE.lock()?.config.venus.subscriptions;
        let found = subscriptions.iter().find(|s| s.url == url);
        if found.is_some() {
            res.message = Some("Subscription already exists".into());
            return Ok((StatusCode::BAD_REQUEST, res));
        }
    }

    let subscription = Subscription {
        name: name.into(),
        url: url.into(),
        nodes: vec![],
    };
    let res_data = subscription.clone();
    {
        let config = &mut CORE.lock()?.config;
        config.venus.subscriptions.push(subscription);
        config.write_rua()?;
    }
    res.message = Some("ok".into());
    res.data = Some(res_data);

    Ok((StatusCode::OK, res))
}

pub fn routes() -> Router {
    Router::new().route("/add", post(add_subscription))
}
