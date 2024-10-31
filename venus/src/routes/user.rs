use serde::Deserialize;
use validator::Validate;

use crate::utils::validator::ValidatedJson;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(length(min = 6, message = "Can not be empty"))]
    pub password: String,
}

pub async fn register(ValidatedJson(input): ValidatedJson<RegisterInput>) {}
