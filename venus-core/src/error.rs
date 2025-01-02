use std::{borrow::Cow, fmt::Display};

use log::error;

use crate::{config::error::ConfigError, message::MessageType};

#[derive(thiserror::Error, Debug)]
pub enum SubscriptionError {
    #[error("subscription already exist")]
    AlreadyExist,
}

#[derive(thiserror::Error, Debug)]
pub enum VenusError {
    #[error("config error {0}")]
    Config(#[from] ConfigError),
    #[error("core error {0}")]
    Core(Cow<'static, str>),

    // from
    #[error("venus io error {0}")]
    IO(#[from] std::io::Error),
    #[error("venus error {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("request error {0}")]
    Request(#[from] reqwest::Error),
    #[error("serialize json error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("decode base64 error {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("from utf8 error {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("subscription error {0:?}")]
    Channel(#[from] std::sync::mpsc::SendError<MessageType>),
    #[error("subscription error {0}")]
    Subscription(#[from] SubscriptionError),
}

pub fn log_err<T: Display>(err: T) -> T {
    error!("{err}");
    err
}

pub type VenusResult<T, E = VenusError> = Result<T, E>;
