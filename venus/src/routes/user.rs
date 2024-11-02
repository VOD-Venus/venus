use std::borrow::Cow;

use serde::Deserialize;
use validator::Validate;
use venus_core::config::types::RUAUser;

use crate::{
    core::CORE,
    error::ErrorCode,
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

pub async fn login(ValidatedJson(input): ValidatedJson<RegisterInput>) -> RouteResult<String> {
    let mut res = RouteResponse {
        code: ErrorCode::ParameterIncorrect,
        message: Some("User not exist or password incorrect".into()),
        ..Default::default()
    };
    let RegisterInput {
        username: _,
        password,
    } = input;

    let user = {
        let core = &CORE.lock()?;
        let config = &core.config;
        if let Some(user) = &config.venus.user {
            user.clone()
        } else {
            return Ok(res);
        }
    };

    let validated = password::verify(password, user.password.into()).await?;
    if !validated {
        return Ok(res);
    }

    res.message = Some("ok".into());
    res.code = ErrorCode::default();
    Ok(res)
}
