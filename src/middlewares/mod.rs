use axum::{
    body::Body, extract::Request, http::HeaderValue, middleware::Next, response::IntoResponse,
};

use crate::error::AppResult;

/// Middleware for adding version information to each response's headers.
///
/// This middleware takes an incoming `Request` and a `Next` handler, which represents the
/// subsequent middleware or route in the chain. It then asynchronously runs the next handler,
/// obtaining the response. After receiving the response, it appends two headers:
/// - "Server": The name of the server extracted from the Cargo package name.
/// - "S-Version": The version of the server extracted from the Cargo package version.
pub async fn add_version(req: Request<Body>, next: Next) -> AppResult<impl IntoResponse> {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.append("Server", HeaderValue::from_str(env!("CARGO_PKG_NAME"))?);
    headers.append(
        "Venus-Version",
        HeaderValue::from_str(env!("CARGO_PKG_VERSION"))?,
    );
    Ok(res)
}
