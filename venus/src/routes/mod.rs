use std::{borrow::Cow, time::Duration};

use axum::{
    http::{HeaderMap, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
};

use crate::{
    consts::VENUS_UI_PATH,
    error::{AppResult, ErrorCode},
    middlewares::{add_version, logging_route},
};

pub mod user;
pub mod version;

#[derive(Debug, Serialize, Default)]
pub struct RouteResponse<T>
where
    T: Serialize,
{
    #[serde(skip_serializing)]
    headers: HeaderMap,
    code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
    data: T,
}

#[derive(Debug, Serialize, Default)]
struct RouteInnerResponse<T>
where
    T: Serialize,
{
    code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
    data: T,
}

impl<T> IntoResponse for RouteResponse<T>
where
    T: Serialize + Default,
{
    fn into_response(self) -> Response {
        let Self {
            headers,
            code,
            message,
            data,
        } = self;
        let res = RouteInnerResponse {
            code,
            message,
            data,
        };
        (headers, Json(res)).into_response()
    }
}
pub type RouteResult<T> = AppResult<RouteResponse<T>>;

pub fn routes() -> Router {
    let ui_folder = &*VENUS_UI_PATH;
    let ui_index = format!("{ui_folder}/index.html");
    let ui_service = ServeDir::new(ui_folder.to_string()).fallback(ServeFile::new(ui_index));

    let router = Router::new()
        .nest_service("/", ui_service)
        .nest(
            "/api/",
            Router::new()
                .route("/version", get(version::version))
                .route("/user/register", post(user::register))
                .route("/user/login", post(user::login)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(add_version))
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::new(Duration::from_secs(15)))
                .layer(CompressionLayer::new()),
        );
    logging_route(router)
}
