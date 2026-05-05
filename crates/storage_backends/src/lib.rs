//! # Storage Backends - 存储后端模块
//!
//! 提供存储后端的抽象接口和多种实现。

pub mod error;
pub mod local;
pub mod opendal_backend;
pub mod oss;
pub mod r2;
pub mod s3;
pub mod trait_def;
pub mod webdav;

pub use error::{StorageError, StorageResult};
pub use local::LocalBackend;
pub use opendal_backend::OpendalBackend;
pub use oss::OssBackend;
pub use r2::R2Backend;
pub use s3::S3Backend;
pub use trait_def::{StorageBackend, StorageMetadata};
pub use webdav::WebdavBackend;
