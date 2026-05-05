//! Provider trait 定义

use async_trait::async_trait;

/// 数据源 Provider 信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
    pub capabilities: Vec<String>,
}

/// Provider 错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("连接失败: {0}")]
    ConnectionFailed(String),
    #[error("认证失败: {0}")]
    AuthenticationFailed(String),
    #[error("数据错误: {0}")]
    DataError(String),
    #[error("配置错误: {0}")]
    ConfigError(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

pub type ProviderResult<T> = Result<T, ProviderError>;

/// 数据源 Provider trait
///
/// 所有数据源连接器必须实现此 trait。
/// Provider 是无状态的，配置通过 new() 传入。
#[async_trait]
pub trait Provider: Send + Sync {
    /// 获取 Provider 信息
    fn info(&self) -> ProviderInfo;
    
    /// 连接到数据源
    async fn connect(&self) -> ProviderResult<()>;
    
    /// 断开连接
    async fn disconnect(&self) -> ProviderResult<()>;
    
    /// 检查连接状态
    async fn is_connected(&self) -> bool;
    
    /// 列出所有数据项
    async fn list(&self, prefix: &str) -> ProviderResult<Vec<String>>;
    
    /// 读取数据
    async fn read(&self, key: &str) -> ProviderResult<Vec<u8>>;
    
    /// 写入数据
    async fn write(&self, key: &str, data: &[u8]) -> ProviderResult<()>;
    
    /// 删除数据
    async fn delete(&self, key: &str) -> ProviderResult<()>;
    
    /// 检查数据是否存在
    async fn exists(&self, key: &str) -> ProviderResult<bool>;
    
    /// 获取数据大小
    async fn size(&self, key: &str) -> ProviderResult<u64>;
}
