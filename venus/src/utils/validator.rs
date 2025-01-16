use std::sync::LazyLock;

use axum::{
    extract::{
        rejection::{FormRejection, JsonRejection},
        FromRequest, Request,
    },
    Form, Json,
};
use regex::Regex;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

pub static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^((https?|ftp):\/\/(([\w-]+\.)+[\w-]+|localhost)(:\d+)?(\/[-\w@:%\+.~#?&//=]*)?)$")
        .expect("create url regex failed")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        let valid_urls = vec![
            "https://example.com",
            "http://example.com",
            "ftp://example.com",
            "http://localhost",
            "https://example.com:8080/path?query=1",
            "http://sub.example.com",
        ];

        for url in valid_urls {
            assert!(
                URL_REGEX.is_match(url),
                "Expected '{}' to be a valid URL",
                url
            );
        }
    }

    #[test]
    fn test_invalid_urls() {
        let invalid_urls = vec![
            "example.com",
            "http//example.com",
            "://example.com",
            "http:/example.com",
            "ftp//example.com",
            "just-a-string",
        ];

        for url in invalid_urls {
            assert!(
                !URL_REGEX.is_match(url),
                "Expected '{}' to be an invalid URL",
                url
            );
        }
    }
}
