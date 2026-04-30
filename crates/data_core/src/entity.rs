//! 数据实体定义模块
//! 
//! 定义系统中所有数据实体的核心结构。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 数据实体唯一标识
pub type DataId = Uuid;

/// 所有者唯一标识
pub type OwnerId = Uuid;

/// 数据类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// 凭证数据 (密码、API Key、Token等)
    Credential,
    /// 配置数据 (应用设置、参数等)
    Config,
    /// 文件数据 (文档、图片等)
    File,
    /// 联系人数据
    Contact,
    /// 通用数据
    Generic,
}

impl DataType {
    /// 获取数据类型的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Credential => "credential",
            DataType::Config => "config",
            DataType::File => "file",
            DataType::Contact => "contact",
            DataType::Generic => "generic",
        }
    }

    /// 从字符串解析数据类型
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "credential" => Some(DataType::Credential),
            "config" => Some(DataType::Config),
            "file" => Some(DataType::File),
            "contact" => Some(DataType::Contact),
            "generic" => Some(DataType::Generic),
            _ => None,
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 数据实体核心结构
/// 
/// 包含数据的所有必要信息，支持加密存储。
/// 
/// # 示例
/// 
/// ```rust
/// use data_core::{DataEntity, DataType};
/// use uuid::Uuid;
/// 
/// let entity = DataEntity::new(
///     Uuid::new_v4(),
///     DataType::Credential,
///     b"my secret data".to_vec(),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntity {
    /// 数据唯一标识
    pub id: DataId,
    
    /// 所有者标识
    pub owner_id: OwnerId,
    
    /// 数据类型
    pub data_type: DataType,
    
    /// 加密后的数据内容
    pub encrypted_content: Vec<u8>,
    
    /// 数据标签 (用于分类和搜索)
    pub tags: Vec<String>,
    
    /// 创建时间
    pub created_at: DateTime<Utc>,
    
    /// 最后修改时间
    pub updated_at: DateTime<Utc>,
    
    /// 版本号 (用于乐观并发控制)
    pub version: u64,
    
    /// 是否已删除 (软删除标记)
    pub is_deleted: bool,
}

impl DataEntity {
    /// 创建新的数据实体
    /// 
    /// # 参数
    /// 
    /// * `owner_id` - 所有者标识
    /// * `data_type` - 数据类型
    /// * `encrypted_content` - 加密后的内容
    /// 
    /// # 返回
    /// 
    /// 新创建的数据实体，自动设置ID、时间戳和版本号
    pub fn new(owner_id: OwnerId, data_type: DataType, encrypted_content: Vec<u8>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id,
            data_type,
            encrypted_content,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            version: 1,
            is_deleted: false,
        }
    }

    /// 创建带有ID的数据实体 (用于反序列化)
    pub fn with_id(
        id: DataId,
        owner_id: OwnerId,
        data_type: DataType,
        encrypted_content: Vec<u8>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        version: u64,
    ) -> Self {
        Self {
            id,
            owner_id,
            data_type,
            encrypted_content,
            tags: Vec::new(),
            created_at,
            updated_at,
            version,
            is_deleted: false,
        }
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 批量添加标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    /// 更新数据内容
    pub fn update_content(&mut self, new_encrypted_content: Vec<u8>) {
        self.encrypted_content = new_encrypted_content;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// 标记为已删除
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// 检查数据是否属于指定所有者
    pub fn belongs_to(&self, owner_id: &OwnerId) -> bool {
        self.owner_id == *owner_id
    }

    /// 获取数据大小 (字节)
    pub fn size(&self) -> usize {
        self.encrypted_content.len()
    }

    /// 序列化为 MessagePack 格式 (高效二进制)
    pub fn to_msgpack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(self)
    }

    /// 从 MessagePack 格式反序列化
    pub fn from_msgpack(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }

    /// 序列化为 JSON 格式 (可读性好)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从 JSON 格式反序列化
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl std::fmt::Display for DataEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataEntity {{ id: {}, type: {}, owner: {}, version: {}, size: {} bytes }}",
            self.id,
            self.data_type,
            self.owner_id,
            self.version,
            self.size()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_entity_creation() {
        let owner_id = Uuid::new_v4();
        let entity = DataEntity::new(
            owner_id,
            DataType::Credential,
            b"secret password".to_vec(),
        );

        assert_eq!(entity.owner_id, owner_id);
        assert_eq!(entity.data_type, DataType::Credential);
        assert_eq!(entity.version, 1);
        assert!(!entity.is_deleted);
    }

    #[test]
    fn test_data_type_conversion() {
        assert_eq!(DataType::Credential.as_str(), "credential");
        assert_eq!(DataType::from_str("config"), Some(DataType::Config));
        assert_eq!(DataType::from_str("invalid"), None);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let entity = DataEntity::new(
            Uuid::new_v4(),
            DataType::Config,
            b"test config".to_vec(),
        );

        // JSON roundtrip
        let json = entity.to_json().unwrap();
        let restored = DataEntity::from_json(&json).unwrap();
        assert_eq!(entity.id, restored.id);

        // MessagePack roundtrip
        let msgpack = entity.to_msgpack().unwrap();
        let restored = DataEntity::from_msgpack(&msgpack).unwrap();
        assert_eq!(entity.id, restored.id);
    }
}
