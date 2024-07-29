use core::get_core;
use std::thread;

use anyhow::anyhow;
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
    // register child message
    {
        let mut venus = venus.lock()?;
        venus.config.reload_rua()?;
        venus.config.reload_core()?;
        venus.config.write_core()?;
        venus.spawn_core()?;
        let child_rx = venus
            .child_rx
            .take()
            .ok_or(anyhow!("get child rx failed"))?;
        thread::spawn(move || {
            while let Ok(msg) = child_rx.recv() {
                info!("{msg}");
            }
        });
    }

    Ok(())
}
