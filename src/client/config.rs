use std::time::Duration;

/// gRPC客户端配置
#[derive(Clone, Debug)]
pub struct Config {
    /// Yellowstone gRPC服务器URL
    pub url: String,
    /// 连接超时时间（秒）
    pub connect_timeout: Duration,
    /// 请求超时时间（秒）
    pub timeout: Duration,
    /// 保持连接活跃
    pub keep_alive_while_idle: bool,
    /// 承诺级别
    pub commitment: yellowstone_grpc_proto::geyser::CommitmentLevel,
}

impl Config {
    /// 创建新的配置
    pub fn new(url: String) -> Self {
        Self {
            url,
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(60),
            keep_alive_while_idle: true,
            commitment: yellowstone_grpc_proto::geyser::CommitmentLevel::Processed,
        }
    }

    /// 设置连接超时时间
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// 设置请求超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置是否保持连接活跃
    pub fn with_keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive_while_idle = keep_alive;
        self
    }

    /// 设置承诺级别
    pub fn with_commitment(
        mut self,
        commitment: yellowstone_grpc_proto::geyser::CommitmentLevel,
    ) -> Self {
        self.commitment = commitment;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("https://solana-yellowstone-grpc.publicnode.com".to_string())
    }
}
