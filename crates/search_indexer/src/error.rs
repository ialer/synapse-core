//! 错误类型定义

use thiserror::Error;

/// 搜索错误类型
#[derive(Debug, Error)]
pub enum SearchError {
    /// 索引不存在
    #[error("索引不存在: {0}")]
    IndexNotFound(String),
    
    /// 查询语法错误
    #[error("查询语法错误: {0}")]
    QuerySyntaxError(String),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(#[from] storage_backends::StorageError),
    
    /// 内部错误
    #[error("内部错误: {0}")]
    Internal(String),
}

/// 搜索结果类型
pub type SearchResult<T> = Result<T, SearchError>;
