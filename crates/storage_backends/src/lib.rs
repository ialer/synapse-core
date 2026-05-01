//! # Storage Backends - 存储后端模块
//! 
//! 提供存储后端的抽象接口和多种实现。

pub mod trait_def;
pub mod local;
pub mod error;

pub use trait_def::{StorageBackend, StorageMetadata};
pub use local::LocalBackend;
pub use error::{StorageError, StorageResult};
