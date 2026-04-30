//! 元数据管理模块
//! 
//! 定义数据实体的元数据结构。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据元数据
/// 
/// 包含数据的附加信息，用于索引、搜索和管理。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// 自定义键值对
    pub properties: HashMap<String, String>,
    
    /// 数据来源 (设备/应用标识)
    pub source: Option<String>,
    
    /// 数据大小 (字节)
    pub size: Option<u64>,
    
    /// MIME 类型
    pub mime_type: Option<String>,
    
    /// 最后访问时间
    pub last_accessed: Option<DateTime<Utc>>,
    
    /// 访问次数
    pub access_count: u64,
    
    /// 优先级 (0-100)
    pub priority: u8,
    
    /// 是否收藏
    pub is_favorite: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            properties: HashMap::new(),
            source: None,
            size: None,
            mime_type: None,
            last_accessed: None,
            access_count: 0,
            priority: 50,
            is_favorite: false,
        }
    }
}

/// 元数据构建器
/// 
/// 提供流式 API 构建元数据。
/// 
/// # 示例
/// 
/// ```rust
/// use data_core::MetadataBuilder;
/// 
/// let metadata = MetadataBuilder::new()
///     .source("android-phone")
///     .mime_type("application/json")
///     .priority(80)
///     .favorite(true)
///     .property("app", "chrome")
///     .build();
/// ```
pub struct MetadataBuilder {
    metadata: Metadata,
}

impl MetadataBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            metadata: Metadata::default(),
        }
    }

    /// 设置数据来源
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.metadata.source = Some(source.into());
        self
    }

    /// 设置数据大小
    pub fn size(mut self, size: u64) -> Self {
        self.metadata.size = Some(size);
        self
    }

    /// 设置 MIME 类型
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.metadata.mime_type = Some(mime_type.into());
        self
    }

    /// 设置优先级
    pub fn priority(mut self, priority: u8) -> Self {
        self.metadata.priority = priority.min(100);
        self
    }

    /// 设置收藏状态
    pub fn favorite(mut self, favorite: bool) -> Self {
        self.metadata.is_favorite = favorite;
        self
    }

    /// 添加自定义属性
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.properties.insert(key.into(), value.into());
        self
    }

    /// 批量添加属性
    pub fn properties(mut self, props: HashMap<String, String>) -> Self {
        self.metadata.properties.extend(props);
        self
    }

    /// 构建元数据
    pub fn build(self) -> Metadata {
        self.metadata
    }
}

impl Metadata {
    /// 记录访问
    pub fn record_access(&mut self) {
        self.last_accessed = Some(Utc::now());
        self.access_count += 1;
    }

    /// 添加属性
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }

    /// 获取属性
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    /// 检查是否有指定属性
    pub fn has_property(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Metadata {{ source: {:?}, size: {:?}, priority: {}, favorite: {} }}",
            self.source, self.size, self.priority, self.is_favorite
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_builder() {
        let metadata = MetadataBuilder::new()
            .source("test-device")
            .size(1024)
            .mime_type("text/plain")
            .priority(90)
            .favorite(true)
            .property("env", "production")
            .build();

        assert_eq!(metadata.source.as_deref(), Some("test-device"));
        assert_eq!(metadata.size, Some(1024));
        assert_eq!(metadata.priority, 90);
        assert!(metadata.is_favorite);
        assert_eq!(metadata.get_property("env").map(String::as_str), Some("production"));
    }

    #[test]
    fn test_metadata_access_tracking() {
        let mut metadata = Metadata::default();
        
        assert_eq!(metadata.access_count, 0);
        assert!(metadata.last_accessed.is_none());
        
        metadata.record_access();
        
        assert_eq!(metadata.access_count, 1);
        assert!(metadata.last_accessed.is_some());
    }
}
