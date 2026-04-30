//! 错误类型定义

use thiserror::Error;

/// Agent 接口错误
#[derive(Debug, Error)]
pub enum AgentError {
    /// 工具未找到
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    
    /// 执行失败
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    /// 权限不足
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// 参数错误
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    /// 数据不存在
    #[error("Data not found: {0}")]
    DataNotFound(String),
    
    /// MCP 错误
    #[error("MCP error: {0}")]
    McpError(String),
    
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON 错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Agent 结果类型
pub type AgentResult<T> = Result<T, AgentError>;
