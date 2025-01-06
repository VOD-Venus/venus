use log::debug;
use serde::{Deserialize, Serialize};

use crate::grpc::error::GrpcError;
use crate::v2ray_core::stats_service_client::StatsServiceClient;
use crate::v2ray_core::QueryStatsRequest;

const HTTP_DOWNLINK: &str = "inbound>>>http>>>traffic>>>downlink";
const HTTP_UPLINK: &str = "inbound>>>http>>>traffic>>>uplink";
const SOCKS_DOWNLINK: &str = "inbound>>>socks>>>traffic>>>downlink";
const SOCKS_UPLINK: &str = "inbound>>>socks>>>traffic>>>uplink";

/// HTTP and SOCKS traffic stats
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    /// bytes
    pub uplink: i64,
    /// bytes
    pub downlink: i64,
}

/// Get stats from grpc server
///
/// ❯ ./v2ray api stats  --server=127.0.0.1:10086
///     Value       Name
/// 1   758.00B     inbound>>>api>>>traffic>>>downlink
/// 2   704.00B     inbound>>>api>>>traffic>>>uplink
/// 3   0           inbound>>>http>>>traffic>>>downlink
/// 4   0           inbound>>>http>>>traffic>>>uplink
/// 5   0           inbound>>>socks>>>traffic>>>downlink
/// 6   0           inbound>>>socks>>>traffic>>>uplink
/// 7   0           outbound>>>blocked>>>traffic>>>downlink
/// 8   0           outbound>>>blocked>>>traffic>>>uplink
/// 9   0           outbound>>>direct>>>traffic>>>downlink
/// 10  0           outbound>>>direct>>>traffic>>>uplink
pub async fn query_stats(url: String) -> Result<StatsResponse, GrpcError> {
    let mut client = StatsServiceClient::connect(url).await?;
    let query = QueryStatsRequest {
        pattern: String::new(),
        reset: false,
        patterns: vec![
            HTTP_DOWNLINK.to_string(),
            HTTP_UPLINK.to_string(),
            SOCKS_DOWNLINK.to_string(),
            SOCKS_UPLINK.to_string(),
        ],
        regexp: false,
    };
    let response = client.query_stats(query).await?;
    let res: StatsResponse = response.into_parts().1.stat.into_iter().fold(
        StatsResponse::default(),
        |mut acc, state| {
            if state.name.contains("downlink") {
                acc.downlink += state.value;
            }
            if state.name.contains("uplink") {
                acc.uplink += state.value;
            }
            acc
        },
    );
    debug!("RESPONSE={:?}", res);
    Ok(res)
}
