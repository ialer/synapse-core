//! 错误类型定义
//! 
//! 定义 data_core 模块的所有错误类型。

use thiserror::Error;

/// 数据模块错误类型
#[derive(Debug, Error)]
pub enum DataError {
    /// 加密错误
    #[error("加密错误: {0}")]
    Cipher(#[from] crate::crypto::CipherError),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// MessagePack 序列化错误
    #[error("MessagePack 序列化错误: {0}")]
    MsgpackEncode(#[from] rmp_serde::encode::Error),
    
    /// MessagePack 反序列化错误
    #[error("MessagePack 反序列化错误: {0}")]
    MsgpackDecode(#[from] rmp_serde::decode::Error),
    
    /// 数据不存在
    #[error("数据不存在: {0}")]
    NotFound(String),
    
    /// 权限不足
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    /// 数据已损坏
    #[error("数据已损坏: {0}")]
    Corrupted(String),
    
    /// 版本冲突
    #[error("版本冲突: 期望版本 {expected}, 实际版本 {actual}")]
    VersionConflict {
        expected: u64,
        actual: u64,
    },
    
    /// 数据类型不匹配
    #[error("数据类型不匹配: 期望 {expected}, 实际 {actual}")]
    TypeMismatch {
        expected: String,
        actual: String,
    },
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(String),
    
    /// 同步错误
    #[error("同步错误: {0}")]
    Sync(String),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),
}

/// 数据操作结果类型
pub type DataResult<T> = Result<T, DataError>;
