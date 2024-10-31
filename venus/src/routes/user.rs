use serde::Deserialize;
use validator::Validate;

use crate::utils::validator::ValidatedJson;

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 6, message = "Can not be empty or less than 6"))]
    pub password: String,
}

pub async fn register(ValidatedJson(input): ValidatedJson<RegisterInput>) -> RouteResult<()> {
    Ok(RouteResponse {
        ..RouteResponse::default()
    })
}
