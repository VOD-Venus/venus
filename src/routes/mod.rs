use std::{collections::HashMap, time::Duration};

use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequestParts, Path, Request},
    http::{request::Parts, HeaderMap, HeaderValue, StatusCode, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, RequestPartsExt, Router,
};
use include_dir::{include_dir, Dir};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass, compression::CompressionLayer, cors::CorsLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tower_serve_static::ServeDir;
use tracing::{error, info, info_span, Span};

use crate::{
    error::{AppResult, ErrorCode},
    middlewares::add_version,
};

pub mod version;

#[derive(Debug, Serialize)]
pub struct RouteResponse<T>
where
    T: Serialize,
{
    code: ErrorCode,
    data: T,
}
pub type RouteResult<T> = AppResult<Json<RouteResponse<T>>>;

static ASSERT_UI: Dir = include_dir!("./public");

pub fn routes() -> Router {
    let service = ServeDir::new(&ASSERT_UI);

    Router::new()
        .route("/", get(hello).post(hello))
        .nest_service("/app", service)
        .route("/:version/version", get(version::version))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(add_version))
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::new(Duration::from_secs(15)))
                .layer(CompressionLayer::new()),
        )
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<_>| {
                    info_span!("HTTP", some_other_field = tracing::field::Empty,)
                })
                .on_request(|req: &Request<_>, _span: &Span| {
                    let unknown = &HeaderValue::from_static("Unknown");
                    let ua = req
                        .headers()
                        .get("User-Agent")
                        .unwrap_or(unknown)
                        .to_str()
                        .unwrap_or("Unknown");
                    info!("{} {} {}", req.method(), req.uri(), ua);
                })
                .on_response(|res: &Response, latency: Duration, _span: &Span| {
                    info!("{} {}ms", res.status(), latency.as_millis());
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        error!("{}", error);
                    },
                ),
        )
}

/// hello world
pub async fn hello() -> String {
    format!("hello {}", env!("CARGO_PKG_NAME"))
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

#[derive(Debug)]
enum Version {
    V1,
    V2,
    V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}
