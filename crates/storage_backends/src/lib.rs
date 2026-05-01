//! # Storage Backends - 存储后端模块
//! 
//! 提供存储后端的抽象接口和多种实现。

pub mod trait_def;
pub mod local;
pub mod webdav;
pub mod s3;
pub mod oss;
pub mod r2;
pub mod error;

pub use trait_def::{StorageBackend, StorageMetadata};
pub use local::LocalBackend;
pub use webdav::WebdavBackend;
pub use s3::S3Backend;
pub use oss::OssBackend;
pub use r2::R2Backend;
pub use error::{StorageError, StorageResult};
