//! 错误类型定义

use thiserror::Error;

/// 存储错误类型
#[derive(Debug, Error)]
pub enum StorageError {
    /// 文件未找到
    #[error("文件未找到: {0}")]
    NotFound(String),
    
    /// 文件已存在
    #[error("文件已存在: {0}")]
    AlreadyExists(String),
    
    /// 权限不足
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    /// 磁盘空间不足
    #[error("磁盘空间不足")]
    InsufficientSpace,
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),
    
    /// 内部错误
    #[error("内部错误: {0}")]
    Internal(String),
}

/// 存储结果类型
pub type StorageResult<T> = Result<T, StorageError>;
