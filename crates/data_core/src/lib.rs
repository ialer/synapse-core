//! # Data Core - 数据核心模块
//! 
//! 定义数据实体结构与加密核心功能。
//! 
//! ## 核心功能
//! 
//! - 数据实体定义 (DataEntity)
//! - AES-256-GCM 加解密
//! - 元数据管理
//! - 序列化/反序列化

pub mod crypto;
pub mod entity;
pub mod error;
pub mod metadata;

pub use crypto::{Cipher, CipherError};
pub use entity::{DataEntity, DataType, DataId, OwnerId};
pub use error::DataError;
pub use metadata::{Metadata, MetadataBuilder};
