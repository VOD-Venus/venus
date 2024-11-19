use axum::{http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;
use venus_core::config::types::RUAUser;

use crate::{
    core::CORE,
    error::{AppResult, ErrorCode},
    utils::{
        jwt::{self, Claims},
        password,
        validator::ValidatedJson,
    },
};

use super::RouteResponse;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 6, message = "Can not be empty or less than 6"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[axum::debug_handler]
pub async fn register(
    ValidatedJson(input): ValidatedJson<RegisterInput>,
) -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<AuthBody>> = RouteResponse {
        ..RouteResponse::default()
    };
    let RegisterInput { username, password } = input;

    let hashed = password::hash(password).await?;
    {
        let config = &mut CORE.lock()?.config;
        if config.venus.user.is_some() {
            res.message = Some("admin user already exist".into());
            return Ok((StatusCode::BAD_REQUEST, res));
        } else {
            config.venus.user = Some(RUAUser {
                username: username.clone().into(),
                password: hashed.into(),
            })
        }
        config.write_rua()?;
    }

    let iat = Utc::now().naive_utc();
    let exp = (iat + chrono::naive::Days::new(7)).and_utc().timestamp() as usize;
    let claims = Claims {
        exp,
        iat: iat.and_utc().timestamp() as usize,
        sub: username,
    };
    let token = jwt::encode_jwt(&claims)?;

    res.data = Some(AuthBody {
        access_token: token,
        token_type: "Bearer".into(),
    });
    res.message = Some("ok".into());
    Ok((StatusCode::OK, res))
}

pub async fn login(
    ValidatedJson(input): ValidatedJson<RegisterInput>,
) -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<AuthBody>> = RouteResponse {
        code: ErrorCode::ParameterIncorrect,
        message: Some("User not exist or password incorrect".into()),
        ..Default::default()
    };
    let RegisterInput { username, password } = input;

    let user = {
        let core = &CORE.lock()?;
        let config = &core.config;
        if let Some(user) = &config.venus.user {
            user.clone()
        } else {
            return Ok((StatusCode::UNAUTHORIZED, res));
        }
    };

    let validated = password::verify(password, user.password.into()).await?;
    if !validated {
        return Ok((StatusCode::UNAUTHORIZED, res));
    }

    let iat = Utc::now().naive_utc();
    let exp = (iat + chrono::naive::Days::new(7)).and_utc().timestamp() as usize;
    let claims = Claims {
        exp,
        iat: iat.and_utc().timestamp() as usize,
        sub: username,
    };
    let token = jwt::encode_jwt(&claims)?;

    res.data = Some(AuthBody {
        access_token: token,
        token_type: "Bearer".into(),
    });
    res.message = Some("ok".into());
    res.code = ErrorCode::default();
    Ok((StatusCode::OK, res))
}

pub async fn protected(claims: Claims) -> AppResult<impl IntoResponse> {
    let sub = claims.sub;
    Ok(sub)
}
