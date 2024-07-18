use dotenvy::dotenv;
use error::AppResult;
use tracing::info;
use utils::init_logger;
use venus_core::Venus;

mod consts;
mod core;
mod error;
mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    init_logger();

    let mut venus = Venus::new()?;
    venus.config.reload_rua()?;
    venus.config.reload_core()?;

    info!("Hello, world!");
    let inbounds = if let Some(c) = venus.config.core {
        c.inbounds
    } else {
        vec![]
    };
    info!("core config {:?}", inbounds);
    Ok(())
}
