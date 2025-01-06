use axum::{response::IntoResponse, routing::get, Router};
use venus_core::grpc::stats::{query_stats, StatsResponse};

use crate::error::AppResult;

use super::RouteResponse;

/// Query http and socks traffic stats
/// 3   0           inbound>>>http>>>traffic>>>downlink
/// 4   0           inbound>>>http>>>traffic>>>uplink
/// 5   0           inbound>>>socks>>>traffic>>>downlink
/// 6   0           inbound>>>socks>>>traffic>>>uplink
pub async fn stats() -> AppResult<impl IntoResponse> {
    let mut res: RouteResponse<Option<StatsResponse>> = RouteResponse {
        ..RouteResponse::default()
    };

    let stats_res = query_stats("http://localhost:10086".to_string()).await?;

    res.message = Some("ok".into());
    res.data = Some(stats_res);
    Ok(res)
}

pub fn routes() -> Router {
    Router::new().route("/all", get(stats))
}
