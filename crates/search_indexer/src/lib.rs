//! # Search Indexer - 搜索索引模块
//!
//! 提供全文搜索与分类检索功能。

pub mod error;
pub mod indexer;
pub mod query;

pub use error::{SearchError, SearchResult};
pub use indexer::{IndexEntry, IndexStats, Indexer};
pub use query::{ParsedQuery, QueryParser, QueryType};
