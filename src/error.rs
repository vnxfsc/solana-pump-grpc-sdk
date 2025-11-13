use thiserror::Error;

/// SDK错误类型
#[derive(Error, Debug)]
pub enum Error {
    #[error("gRPC客户端错误: {0}")]
    GrpcClient(#[from] yellowstone_grpc_client::GeyserGrpcClientError),

    #[error("gRPC客户端构建错误: {0}")]
    GrpcBuilder(String),

    #[error("gRPC连接错误: {0}")]
    GrpcConnection(String),

    #[error("TLS配置错误: {0}")]
    TlsConfig(String),

    #[error("连接超时")]
    ConnectionTimeout,

    #[error("订阅错误: {0}")]
    SubscribeError(String),

    #[error("事件解析错误: {0}")]
    ParseError(String),

    #[error("Borsh反序列化错误: {0}")]
    BorshDeserialize(#[from] std::io::Error),

    #[error("签名解析错误")]
    SignatureParse,

    #[error("RPC错误: {0}")]
    RpcError(String),

    #[error("Pubkey解析错误: {0}")]
    PubkeyParse(#[from] solana_sdk::pubkey::PubkeyError),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// Result类型别名
pub type Result<T> = std::result::Result<T, Error>;
