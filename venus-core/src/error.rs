use std::{borrow::Cow, fmt::Display};

use log::error;

use crate::config::error::ConfigError;

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
}

pub fn log_err<T: Display>(err: T) -> T {
    error!("{err}");
    err
}

pub type VenusResult<T, E = VenusError> = Result<T, E>;
