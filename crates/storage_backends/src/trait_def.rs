//! 存储后端 Trait 定义
//! 
//! 定义存储后端的抽象接口。

use async_trait::async_trait;

use crate::error::StorageResult;

/// 存储后端 Trait
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// 保存数据
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()>;
    
    /// 加载数据
    async fn load(&self, key: &str) -> StorageResult<Vec<u8>>;
    
    /// 列出数据
    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>>;
    
    /// 删除数据
    async fn delete(&self, key: &str) -> StorageResult<()>;
    
    /// 检查数据是否存在
    async fn exists(&self, key: &str) -> StorageResult<bool>;
    
    /// 获取数据大小
    async fn size(&self, key: &str) -> StorageResult<u64>;
    
    /// 复制数据
    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()>;
    
    /// 移动数据
    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()>;
}

/// 存储元数据
#[derive(Debug, Clone)]
pub struct StorageMetadata {
    /// 键
    pub key: String,
    
    /// 大小（字节）
    pub size: u64,
    
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// 修改时间
    pub modified_at: chrono::DateTime<chrono::Utc>,
    
    /// 内容类型
    pub content_type: Option<String>,
    
    /// 校验和
    pub checksum: Option<String>,
}

impl StorageMetadata {
    /// 创建新的存储元数据
    pub fn new(key: impl Into<String>, size: u64) -> Self {
        let now = chrono::Utc::now();
        Self {
            key: key.into(),
            size,
            created_at: now,
            modified_at: now,
            content_type: None,
            checksum: None,
        }
    }
    
    /// 设置内容类型
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
    
    /// 设置校验和
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }
}
