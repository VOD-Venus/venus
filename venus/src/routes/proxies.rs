use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
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

pub async fn add_subscription(
    ValidatedJson(payload): ValidatedJson<SubPayload>,
) -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<()>> = RouteResponse {
        ..RouteResponse::default()
    };
    Ok(res)
}
