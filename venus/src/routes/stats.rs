use axum::{response::IntoResponse, routing::get, Router};
use venus_core::grpc::stats::stats_test;

use crate::error::AppResult;

use super::RouteResponse;

pub async fn stats() -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<()>> = RouteResponse {
        ..RouteResponse::default()
    };

    stats_test("http://localhost:10086".to_string()).await?;

    res.message = Some("ok".into());
    Ok(res)
}

pub fn routes() -> Router {
    Router::new().route("/all", get(stats))
}
