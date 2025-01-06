use log::error;

use crate::grpc::error::GrpcError;
use crate::v2ray_core::stats_service_client::StatsServiceClient;
use crate::v2ray_core::GetStatsRequest;

const TEST: &str = "inbound>>>api>>>traffic>>>uplink";

pub async fn stats_test(url: String) -> Result<(), GrpcError> {
    let mut client = StatsServiceClient::connect(url).await?;
    let request = GetStatsRequest {
        name: TEST.to_string(),
        reset: false,
    };
    error!("test");
    let response = client.get_stats(request).await?;
    error!("RESPONSE={:?}", response);
    Ok(())
}
