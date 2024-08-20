use core::{CORE, MSG};
use std::{env, error::Error, net::SocketAddr, thread};

use anyhow::Context;
use axum::Router;
use consts::DEFAULT_PORT;
use dotenvy::dotenv;
use routes::routes;
use tokio::net::TcpListener;
use tracing::{info, span, Level};
use utils::{init_logger, shutdown_signal};

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
            let child_rx = &MSG.lock().expect("cannot access global message").1;
            let core_span = span!(Level::INFO, "core").entered();
            while let Ok(msg) = child_rx.recv() {
                info!("{msg:?}");
            }
            core_span.exit();
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

#[derive(Debug, Clone)]
pub struct AppState {}

fn app() -> Router {
    Router::new().merge(routes())
}
