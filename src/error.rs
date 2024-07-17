#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("venus core error {0}")]
    VenusCore(#[from] venus_core::error::VenusError),
    #[error("venus config error {0}")]
    VenusConfig(#[from] venus_core::config::error::ConfigError),
}

pub type AppResult<T, E = AppError> = Result<T, E>;
