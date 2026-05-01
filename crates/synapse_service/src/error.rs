//! 错误类型定义

use thiserror::Error;

/// Synapse 服务错误类型
#[derive(Debug, Error)]
pub enum SynapseError {
    /// 认证错误
    #[error("认证错误: {0}")]
    Auth(#[from] iam_core::AuthError),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(#[from] storage_backends::StorageError),
    
    /// 加密错误
    #[error("加密错误: {0}")]
    Cipher(#[from] data_core::CipherError),
    
    /// 数据错误
    #[error("数据错误: {0}")]
    Data(#[from] data_core::DataError),
    
    /// 同步错误
    #[error("同步错误: {0}")]
    Sync(#[from] sync_engine::SyncError),
    
    /// 搜索错误
    #[error("搜索错误: {0}")]
    Search(#[from] search_indexer::SearchError),
    
    /// 消息错误
    #[error("消息错误: {0}")]
    Message(#[from] messaging_service::MessageError),
    
    /// 权限不足
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    /// 数据不存在
    #[error("数据不存在: {0}")]
    NotFound(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),
    
    /// 内部错误
    #[error("内部错误: {0}")]
    Internal(String),
}

/// Synapse 服务结果类型
pub type SynapseResult<T> = Result<T, SynapseError>;

// 添加 rmp_serde 错误转换
impl From<rmp_serde::encode::Error> for SynapseError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        SynapseError::Internal(err.to_string())
    }
}

impl From<rmp_serde::decode::Error> for SynapseError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        SynapseError::Internal(err.to_string())
    }
}
