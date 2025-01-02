use core::{global_core, global_message};
use std::{env, error::Error, net::SocketAddr};

use axum::Router;
use consts::{DEFAULT_PORT, RUA_COMPILER};
use dotenvy::dotenv;
use routes::routes;
use tokio::net::TcpListener;
use tracing::{info, span, Level};
use utils::{init_logger, shutdown_cb, shutdown_signal};
use venus_core::{message::MessageType, VenusCore};

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

    tokio::spawn(async move {
        info!("venus {RUA_COMPILER}");
        let venus = &mut global_core().await.lock().await;
        info!("string core");
        venus
            .config
            .reload_rua()
            .expect("reading venus configuration failed");
        venus
            .config
            .reload_core()
            .expect("reading core configuration failed");
        venus
            .config
            .write_core()
            .expect("write core configuration failed");
        venus.spawn_core().expect("staring core failed");

        // global message handler
        let child_rx = &global_message().await.lock().await.1;
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
