use core::get_core;

use dotenvy::dotenv;
use error::AppResult;
use tracing::info;
use utils::init_logger;

mod consts;
mod core;
mod error;
mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    init_logger();

    let venus = get_core();

    {
        let mut venus = venus.lock()?;
        venus.config.reload_rua()?;
        venus.config.reload_core()?;
        info!("Hello, world!");
        let inbounds = if let Some(c) = venus.config.core.as_ref() {
            c.inbounds.as_ref()
        } else {
            &vec![]
        };
        info!("core config {:?}", inbounds);
        venus.config.write_core()?;
    }
    Ok(())
}
