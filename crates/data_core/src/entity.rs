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

/// 权限级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// 只读
    View,
    /// 可编辑
    Edit,
    /// 管理员 (可分享/删除)
    Admin,
}

/// 共享权限条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharePermission {
    /// 被授权用户ID
    pub user_id: OwnerId,
    /// 权限级别
    pub level: PermissionLevel,
    /// 授权时间
    pub granted_at: DateTime<Utc>,
    /// 授权人
    pub granted_by: OwnerId,
}

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
    
    /// 共享权限列表
    pub shared_with: Vec<SharePermission>,
    
    /// 文件夹路径 (用于分类)
    pub folder: Option<String>,
    
    /// MIME 类型
    pub content_type: Option<String>,
    
    /// 数据校验和
    pub checksum: Option<String>,
    
    /// 扩展元数据
    pub metadata: std::collections::HashMap<String, String>,
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
            shared_with: Vec::new(),
            folder: None,
            content_type: None,
            checksum: None,
            metadata: std::collections::HashMap::new(),
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
            shared_with: Vec::new(),
            folder: None,
            content_type: None,
            checksum: None,
            metadata: std::collections::HashMap::new(),
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

    /// 设置文件夹路径
    pub fn with_folder(mut self, folder: impl Into<String>) -> Self {
        self.folder = Some(folder.into());
        self
    }

    /// 设置 MIME 类型
    pub fn with_content_type(mut self, ct: impl Into<String>) -> Self {
        self.content_type = Some(ct.into());
        self
    }

    /// 设置数据校验和
    pub fn with_checksum(mut self, cs: impl Into<String>) -> Self {
        self.checksum = Some(cs.into());
        self
    }

    /// 添加一条扩展元数据
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into());
        self
    }

    /// 批量设置扩展元数据
    pub fn with_meta(mut self, map: std::collections::HashMap<String, String>) -> Self {
        self.metadata.extend(map);
        self
    }

    /// 添加共享权限
    ///
    /// 如果该用户已有权限，则更新为新的级别。
    pub fn share_with(&mut self, user_id: OwnerId, level: PermissionLevel, granted_by: OwnerId) {
        // Remove existing permission for this user if present
        self.shared_with.retain(|p| p.user_id != user_id);
        self.shared_with.push(SharePermission {
            user_id,
            level,
            granted_at: Utc::now(),
            granted_by,
        });
    }

    /// 撤销对指定用户的共享权限
    ///
    /// 返回 true 表示成功移除，false 表示该用户原本没有权限。
    pub fn revoke_share(&mut self, user_id: &OwnerId) -> bool {
        let before = self.shared_with.len();
        self.shared_with.retain(|p| p.user_id != *user_id);
        self.shared_with.len() < before
    }

    /// 检查指定用户是否拥有至少指定级别的权限
    ///
    /// 所有者始终拥有 Admin 权限。
    pub fn has_permission(&self, user_id: &OwnerId, level: PermissionLevel) -> bool {
        // Owner always has full admin access
        if self.owner_id == *user_id {
            return true;
        }
        self.shared_with.iter().any(|p| {
            p.user_id == *user_id
                && match (p.level, level) {
                    (PermissionLevel::Admin, _) => true,
                    (PermissionLevel::Edit, PermissionLevel::Edit | PermissionLevel::View) => true,
                    (PermissionLevel::View, PermissionLevel::View) => true,
                    _ => false,
                }
        })
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

    /// 检查数据是否属于指定所有者或拥有共享权限
    pub fn belongs_to(&self, owner_id: &OwnerId) -> bool {
        self.owner_id == *owner_id
            || self.shared_with.iter().any(|p| p.user_id == *owner_id)
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
        // New fields initialized to defaults
        assert!(entity.shared_with.is_empty());
        assert!(entity.folder.is_none());
        assert!(entity.content_type.is_none());
        assert!(entity.checksum.is_none());
        assert!(entity.metadata.is_empty());
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

    // --- Sharing / Permission tests ---

    #[test]
    fn test_share_with_and_has_permission() {
        let owner_id = Uuid::new_v4();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();

        let mut entity = DataEntity::new(
            owner_id,
            DataType::File,
            b"file content".to_vec(),
        );

        // Initially no one else has permissions
        assert!(!entity.has_permission(&user_a, PermissionLevel::View));
        assert!(!entity.has_permission(&user_b, PermissionLevel::View));

        // Owner always has Admin
        assert!(entity.has_permission(&owner_id, PermissionLevel::Admin));

        // Share with user_a as View
        entity.share_with(user_a, PermissionLevel::View, owner_id);
        assert!(entity.has_permission(&user_a, PermissionLevel::View));
        assert!(!entity.has_permission(&user_a, PermissionLevel::Edit));
        assert!(!entity.has_permission(&user_a, PermissionLevel::Admin));

        // Share with user_b as Admin
        entity.share_with(user_b, PermissionLevel::Admin, owner_id);
        assert!(entity.has_permission(&user_b, PermissionLevel::View));
        assert!(entity.has_permission(&user_b, PermissionLevel::Edit));
        assert!(entity.has_permission(&user_b, PermissionLevel::Admin));
    }

    #[test]
    fn test_edit_permission_implies_view() {
        let owner_id = Uuid::new_v4();
        let user = Uuid::new_v4();

        let mut entity = DataEntity::new(
            owner_id,
            DataType::Config,
            b"data".to_vec(),
        );

        entity.share_with(user, PermissionLevel::Edit, owner_id);
        assert!(entity.has_permission(&user, PermissionLevel::View));
        assert!(entity.has_permission(&user, PermissionLevel::Edit));
        assert!(!entity.has_permission(&user, PermissionLevel::Admin));
    }

    #[test]
    fn test_revoke_share() {
        let owner_id = Uuid::new_v4();
        let user = Uuid::new_v4();

        let mut entity = DataEntity::new(
            owner_id,
            DataType::Generic,
            b"data".to_vec(),
        );

        entity.share_with(user, PermissionLevel::View, owner_id);
        assert!(entity.has_permission(&user, PermissionLevel::View));

        let removed = entity.revoke_share(&user);
        assert!(removed);
        assert!(!entity.has_permission(&user, PermissionLevel::View));

        // Revoking again returns false
        let removed_again = entity.revoke_share(&user);
        assert!(!removed_again);
    }

    #[test]
    fn test_share_with_updates_level() {
        let owner_id = Uuid::new_v4();
        let user = Uuid::new_v4();

        let mut entity = DataEntity::new(
            owner_id,
            DataType::Generic,
            b"data".to_vec(),
        );

        entity.share_with(user, PermissionLevel::View, owner_id);
        assert!(entity.has_permission(&user, PermissionLevel::View));
        assert!(!entity.has_permission(&user, PermissionLevel::Edit));

        // Upgrading to Edit
        entity.share_with(user, PermissionLevel::Edit, owner_id);
        assert!(entity.has_permission(&user, PermissionLevel::View));
        assert!(entity.has_permission(&user, PermissionLevel::Edit));
        assert!(!entity.has_permission(&user, PermissionLevel::Admin));

        // Should only have one entry
        let count = entity.shared_with.iter().filter(|p| p.user_id == user).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_belongs_to_includes_shared() {
        let owner_id = Uuid::new_v4();
        let user = Uuid::new_v4();

        let mut entity = DataEntity::new(
            owner_id,
            DataType::Generic,
            b"data".to_vec(),
        );

        assert!(entity.belongs_to(&owner_id));
        assert!(!entity.belongs_to(&user));

        entity.share_with(user, PermissionLevel::View, owner_id);
        assert!(entity.belongs_to(&user));
    }

    // --- Builder / metadata tests ---

    #[test]
    fn test_builder_with_folder_content_type_checksum() {
        let owner_id = Uuid::new_v4();
        let entity = DataEntity::new(
            owner_id,
            DataType::File,
            b"content".to_vec(),
        )
        .with_folder("documents/reports")
        .with_content_type("application/pdf")
        .with_checksum("sha256:abc123");

        assert_eq!(entity.folder.as_deref(), Some("documents/reports"));
        assert_eq!(entity.content_type.as_deref(), Some("application/pdf"));
        assert_eq!(entity.checksum.as_deref(), Some("sha256:abc123"));
    }

    #[test]
    fn test_builder_with_metadata() {
        let owner_id = Uuid::new_v4();
        let entity = DataEntity::new(
            owner_id,
            DataType::Config,
            b"config".to_vec(),
        )
        .with_metadata("author", "alice")
        .with_metadata("language", "en");

        assert_eq!(entity.metadata.get("author").map(|s| s.as_str()), Some("alice"));
        assert_eq!(entity.metadata.get("language").map(|s| s.as_str()), Some("en"));

        // with_meta (bulk)
        let mut extra = std::collections::HashMap::new();
        extra.insert("region".to_string(), "us-east-1".to_string());
        let entity = entity.with_meta(extra);
        assert_eq!(entity.metadata.get("region").map(|s| s.as_str()), Some("us-east-1"));
    }

    #[test]
    fn test_builder_with_id_has_new_fields() {
        let owner_id = Uuid::new_v4();
        let now = Utc::now();
        let entity = DataEntity::with_id(
            Uuid::new_v4(),
            owner_id,
            DataType::Generic,
            b"data".to_vec(),
            now,
            now,
            1,
        );

        assert!(entity.shared_with.is_empty());
        assert!(entity.folder.is_none());
        assert!(entity.content_type.is_none());
        assert!(entity.checksum.is_none());
        assert!(entity.metadata.is_empty());
    }

    #[test]
    fn test_serialization_with_new_fields() {
        let owner_id = Uuid::new_v4();
        let user = Uuid::new_v4();
        let mut entity = DataEntity::new(
            owner_id,
            DataType::File,
            b"file data".to_vec(),
        )
        .with_folder("photos/2024")
        .with_content_type("image/jpeg")
        .with_checksum("md5:deadbeef")
        .with_metadata("camera", "iPhone 15");

        entity.share_with(user, PermissionLevel::Edit, owner_id);

        // JSON roundtrip
        let json = entity.to_json().unwrap();
        let restored = DataEntity::from_json(&json).unwrap();
        assert_eq!(entity.id, restored.id);
        assert_eq!(restored.folder, Some("photos/2024".to_string()));
        assert_eq!(restored.content_type, Some("image/jpeg".to_string()));
        assert_eq!(restored.checksum, Some("md5:deadbeef".to_string()));
        assert_eq!(restored.metadata.get("camera").map(|s| s.as_str()), Some("iPhone 15"));
        assert_eq!(restored.shared_with.len(), 1);
        assert_eq!(restored.shared_with[0].level, PermissionLevel::Edit);

        // MessagePack roundtrip
        let msgpack = entity.to_msgpack().unwrap();
        let restored = DataEntity::from_msgpack(&msgpack).unwrap();
        assert_eq!(entity.id, restored.id);
        assert_eq!(restored.folder, Some("photos/2024".to_string()));
        assert_eq!(restored.shared_with.len(), 1);
    }
}
