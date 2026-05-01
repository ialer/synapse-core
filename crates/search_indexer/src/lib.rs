//! # Search Indexer - 搜索索引模块
//! 
//! 提供全文搜索与分类检索功能。

pub mod indexer;
pub mod query;
pub mod error;

pub use indexer::{Indexer, IndexEntry, IndexStats};
pub use query::{QueryParser, ParsedQuery, QueryType};
pub use error::{SearchError, SearchResult};
