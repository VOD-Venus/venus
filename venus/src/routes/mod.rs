use std::{borrow::Cow, time::Duration};

use axum::{
    http::{StatusCode, Uri},
    middleware,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use include_dir::{include_dir, Dir};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer};
use tower_serve_static::ServeDir;
use tracing::info;

use crate::{
    error::{AppResult, ErrorCode},
    middlewares::{add_version, logging_route},
};

pub mod version;

#[derive(Debug, Serialize, Default)]
pub struct RouteResponse<T>
where
    T: Serialize,
{
    code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
    data: T,
}
pub type RouteResult<T> = AppResult<Json<RouteResponse<T>>>;

static ASSERT_UI: Dir = include_dir!("./venus-ui/dist");

pub fn routes() -> Router {
    let service = ServeDir::new(&ASSERT_UI);

    let router = Router::new()
        .nest_service("/", service)
        .nest(
            "/api/",
            Router::new().route("/version", get(version::version)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(add_version))
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::new(Duration::from_secs(15)))
                .layer(CompressionLayer::new()),
        )
        .fallback(fallback);
    logging_route(router)
}

/// Fallback route handler for handling unmatched routes.
///
/// This asynchronous function takes a `Uri` as an argument, representing the unmatched route.
/// It logs a message indicating that the specified route is not found and returns a standard
/// "Not Found" response with a `StatusCode` of `404`.
///
/// # Arguments
///
/// - `uri`: The `Uri` representing the unmatched route.
///
/// # Returns
///
/// Returns a tuple `(StatusCode, &str)` where `StatusCode` is set to `NOT_FOUND` (404),
/// indicating that the route was not found, and the string "Not found" as the response body.
pub async fn fallback(uri: Uri) -> impl IntoResponse {
    info!("route {} not found", uri);
    (StatusCode::NOT_FOUND, "Not found")
}
