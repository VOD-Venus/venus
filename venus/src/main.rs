use core::{CORE, MSG};
use std::{env, error::Error, net::SocketAddr, thread};

use anyhow::Context;
use axum::Router;
use consts::{DEFAULT_PORT, RUA_COMPILER};
use dotenvy::dotenv;
use routes::routes;
use tokio::net::TcpListener;
use tracing::{error, info, span, Level};
use utils::{init_logger, shutdown_cb, shutdown_signal};
use venus_core::message::MessageType;

mod consts;
mod core;
mod error;
mod middlewares;
mod routes;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logger();

    info!("venus {RUA_COMPILER}");
    let venus = &*CORE;
    // register child message
    {
        info!("string core");
        let mut venus = venus.lock()?;
        venus
            .config
            .reload_rua()
            .with_context(|| "reading venus configuration failed")?;
        venus
            .config
            .reload_core()
            .with_context(|| "reading core configuration failed")?;
        venus
            .config
            .write_core()
            .with_context(|| "write core configuration failed")?;
        venus.spawn_core().with_context(|| "staring core failed")?;
        // global message handler
        thread::spawn(move || {
            let lock = &MSG.lock();
            let child_rx = match lock {
                Ok(msg) => &msg.1,
                Err(err) => {
                    error!("lock message failed {err}");
                    return;
                }
            };
            while let Ok(msg) = child_rx.recv() {
                match msg {
                    MessageType::Core(msg) => {
                        let core_span = span!(Level::INFO, "CORE").entered();
                        info!("{msg}");
                        core_span.exit();
                    }
                }
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
        .with_graceful_shutdown(shutdown_signal(shutdown_cb))
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {}

fn app() -> Router {
    Router::new().merge(routes())
}
