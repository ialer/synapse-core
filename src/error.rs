//! 错误类型定义

use thiserror::Error;

/// 同步错误类型
#[derive(Debug, Error)]
pub enum SyncError {
    /// 冲突检测失败
    #[error("冲突检测失败: {0}")]
    ConflictDetected(String),
    
    /// 同步超时
    #[error("同步超时: {0}")]
    Timeout(String),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(#[from] storage_backends::StorageError),
    
    /// 数据不存在
    #[error("数据不存在: {0}")]
    NotFound(String),
    
    /// 版本冲突
    #[error("版本冲突: 本地版本 {local}, 远程版本 {remote}")]
    VersionConflict {
        /// 本地版本
        local: u64,
        /// 远程版本
        remote: u64,
    },
    
    /// 内部错误
    #[error("内部错误: {0}")]
    Internal(String),
}

/// 同步结果类型
pub type SyncResult<T> = Result<T, SyncError>;
