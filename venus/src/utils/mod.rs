use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
use venus_core::{error::log_err, VenusCore};

use crate::{core::global_core, error::AppResult};

pub mod jwt;
pub mod password;
pub mod validator;

/// Initializes the logger for tracing.
///
/// This function sets up the necessary layers for tracing using the `tracing_subscriber`
/// crate. It configures the formatting layer and environment filter based on the value
/// of the `LIMOS_LOG` environment variable (defaulting to "info" if not set).
///
/// # Example
///
/// ```rust
/// use utils::init_logger;
///
/// fn test() {
///     init_logger();
/// }
/// ```
pub fn init_logger() {
    let formatting_layer = fmt::layer()
        // .pretty()
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(std::io::stdout);

    let env_layer = EnvFilter::try_from_env("VENUS_LOG").unwrap_or_else(|_| "info".into());

    registry().with(env_layer).with(formatting_layer).init();
}

async fn stop_core() -> AppResult<()> {
    info!("stopping core");
    let venus = &mut global_core().await.lock().await;
    venus.config.write_core()?;
    venus.config.write_rua()?;
    venus.kill_core()?;
    Ok(())
}

pub fn shutdown_cb() {
    tokio::spawn(async move {
        stop_core().await.map_err(log_err).ok();
    });
}

/// Asynchronously waits for a shutdown signal and executes a callback function when a signal is received.
///
/// This function listens for shutdown signals in the form of `Ctrl+C` and termination signals. When one of
/// these signals is received, it invokes the provided callback function `shutdown_cb`.
///
/// The behavior of the signal handling depends on the operating system:
///
/// - On Unix-based systems (e.g., Linux, macOS), it listens for termination signals (such as SIGTERM).
/// - On non-Unix systems (e.g., Windows), it only listens for `Ctrl+C` and ignores termination signals.
///
/// The `shutdown_cb` callback function is executed when either signal is received. This function should
/// contain the logic needed to gracefully shut down the application or perform any necessary cleanup tasks.
/// # Parameters
///
/// - `shutdown_cb`: A closure or function to call when a shutdown signal is received. The function should
///   have the signature `Fn()`. This callback is executed without any parameters.
///
/// # Errors
///
/// - If setting up the signal handlers fails, the function will panic with an error message.
///
/// # Panics
///
/// - Panics if the setup for `Ctrl+C` or termination signal handlers fails.
///
/// # Platform-specific behavior
///
/// - On Unix-based systems, termination signals are handled using the `signal` crate for Unix signals.
/// - On non-Unix systems, only `Ctrl+C` signals are handled, and termination signals are not supported.
///
/// # Future
///
/// This function returns a future that resolves when either `Ctrl+C` or a termination signal is received
/// and the callback function has been executed.
pub async fn shutdown_signal<F>(shutdown_cb: F)
where
    F: Fn(),
{
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            shutdown_cb()
        },
        _ = terminate => {
            shutdown_cb()
        },
    }
}
