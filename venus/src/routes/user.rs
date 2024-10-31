use std::borrow::Cow;

use serde::Deserialize;
use validator::Validate;
use venus_core::config::types::RUAUser;

use crate::{
    core::CORE,
    utils::{password, validator::ValidatedJson},
};

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 6, message = "Can not be empty or less than 6"))]
    pub password: String,
}

#[axum::debug_handler]
pub async fn register(
    ValidatedJson(input): ValidatedJson<RegisterInput>,
) -> RouteResult<Cow<'static, str>> {
    let mut res = RouteResponse {
        ..RouteResponse::default()
    };
    let RegisterInput { username, password } = input;

    let hashed = password::hash(password).await?;
    {
        let config = &mut CORE.lock()?.config;
        if config.venus.user.is_some() {
            res.data = "admin user already exist".into();
            return Ok(res);
        } else {
            config.venus.user = Some(RUAUser {
                username: username.into(),
                password: hashed.into(),
            })
        }
        config.write_rua()?;
    }
    res.data = "ok".into();
    Ok(res)
}
