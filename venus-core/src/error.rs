use std::{fmt::Display, sync::mpsc::SendError};

use log::error;
use thiserror::Error;

use crate::{config::error::ConfigError, message::MessageType};

#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Subscription already exists: {0}")]
    AlreadyExist(String),

    #[error("Invalid node format in subscription: {0}")]
    InvalidFormat(String),

    #[error("Base64 decode failed for subscription {0}: {1}")]
    Base64Decode(String, #[source] base64::DecodeError),

    #[error("UTF-8 conversion failed for subscription {0}: {1}")]
    Utf8Conversion(String, #[source] std::string::FromUtf8Error),

    #[error("JSON parse error in subscription {0}")]
    JsonParse(#[source] serde_json::Error),

    #[error("Empty subscription content: {0}")]
    EmptyContent(String),
}

#[derive(Debug, Error)]
pub enum VenusError {
    // 配置相关错误
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    // 核心进程错误
    #[error("Failed to launch core process: {0}")]
    CoreLaunch(String),

    #[error("Core process not running")]
    CoreNotRunning,

    #[error("Failed to terminate core process: {0}")]
    ProcessTermination(String),

    // 版本相关错误
    #[error("Failed to parse version from: {0}")]
    VersionParse(String),

    // IO/系统错误
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // 网络请求错误
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    // 订阅处理错误
    #[error("Subscription error: {0}")]
    Subscription(#[from] SubscriptionError),

    // 线程/通道错误
    #[error("Thread join failed: {0}")]
    ThreadJoin(String),

    #[error("Channel send error: {0}")]
    ChannelSend(#[from] SendError<MessageType>),

    // 子进程流错误
    #[error("Child process stream unavailable")]
    ChildStream,

    // 通用错误
    #[error("Runtime error: {0}")]
    Runtime(#[from] anyhow::Error),
}

pub fn log_err<E: Display>(err: E) -> E {
    error!("[VENUS ERROR] {}", err);
    err
}

pub type VenusResult<T> = Result<T, VenusError>;
