use core::get_core;
use std::{env, error::Error, net::SocketAddr, thread};

use anyhow::anyhow;
use axum::Router;
use consts::DEFAULT_PORT;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;
use utils::{init_logger, shutdown_signal};

mod consts;
mod core;
mod error;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logger();

    let venus = get_core();
    // register child message
    {
        info!("string core");
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

    let port = env::var("VENUS_PORT")
        .map(|port| port.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn app() -> Router {
    Router::new()
}
