#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("grpc error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("grpc error: {0}")]
    Status(#[from] tonic::Status),
}
