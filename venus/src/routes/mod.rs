use std::{borrow::Cow, collections::HashMap, time::Duration};

use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, RequestPartsExt, Router,
};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer};
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

// static ASSERT_UI: Dir = include_dir!("./public");

pub fn routes() -> Router {
    // let service = ServeDir::new(&ASSERT_UI);

    let router = Router::new()
        .route("/", get(hello).post(hello))
        // .nest_service("/app", service)
        .route("/:version/version", get(version::version))
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
