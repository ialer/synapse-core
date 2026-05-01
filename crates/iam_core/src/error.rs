//! 错误类型定义模块
//! 
//! 定义身份认证模块的所有错误类型。

use thiserror::Error;

/// 认证错误类型
#[derive(Debug, Error)]
pub enum AuthError {
    /// 用户名或密码错误
    #[error("用户名或密码错误")]
    InvalidCredentials,
    
    /// 用户不存在
    #[error("用户不存在: {0}")]
    UserNotFound(String),
    
    /// 用户已被禁用
    #[error("用户已被禁用: {0}")]
    UserDisabled(String),
    
    /// Token 无效
    #[error("Token 无效: {0}")]
    InvalidToken(String),
    
    /// Token 已过期
    #[error("Token 已过期")]
    TokenExpired,
    
    /// Token 已撤销
    #[error("Token 已撤销")]
    TokenRevoked,
    
    /// 权限不足
    #[error("权限不足: 需要 {required:?}，当前 {current:?}")]
    PermissionDenied {
        /// 所需权限
        required: Vec<String>,
        /// 当前权限
        current: Vec<String>,
    },
    
    /// 角色不匹配
    #[error("角色不匹配: 需要 {required:?}，当前 {current:?}")]
    RoleMismatch {
        /// 所需角色
        required: String,
        /// 当前角色
        current: String,
    },
    
    /// 会话无效
    #[error("会话无效: {0}")]
    InvalidSession(String),
    
    /// 会话已过期
    #[error("会话已过期")]
    SessionExpired,
    
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    StorageError(String),
    
    /// 内部错误
    #[error("内部错误: {0}")]
    InternalError(String),
}

/// 认证结果类型
pub type AuthResult<T> = Result<T, AuthError>;
