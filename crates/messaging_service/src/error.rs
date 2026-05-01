//! 错误类型定义

use thiserror::Error;

/// 消息服务错误类型
#[derive(Debug, Error)]
pub enum MessageError {
    /// 消息发送失败
    #[error("消息发送失败: {0}")]
    SendFailed(String),
    
    /// 消息接收失败
    #[error("消息接收失败: {0}")]
    ReceiveFailed(String),
    
    /// 用户不存在
    #[error("用户不存在: {0}")]
    UserNotFound(String),
    
    /// 通知失败
    #[error("通知失败: {0}")]
    NotificationFailed(String),
    
    /// 内部错误
    #[error("内部错误: {0}")]
    Internal(String),
}

/// 消息结果类型
pub type MessageResult<T> = Result<T, MessageError>;
