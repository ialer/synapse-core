//! 统一网关服务模块
//! 
//! 整合 IAM、Storage 和 Data 核心，对外提供统一的 API 接口。

pub mod error;
pub mod sharing;
pub mod pipeline;

use std::collections::HashMap;
use async_trait::async_trait;

use crate::error::{SynapseError, SynapseResult};
use data_core::{DataEntity, DataType, Cipher};
use iam_core::{AuthService, JwtConfig, DiskAuthService, MemoryAuthService};
use storage_backends::{StorageBackend, LocalBackend};
use search_indexer::Indexer;
use messaging_service::{MessageService, NotificationManager};

/// 存储后端类型
pub enum StorageType {
    /// 本地存储
    Local {
        /// 存储路径
        path: String,
    },
    /// WebDAV 存储
    Webdav {
        /// 服务端点
        endpoint: String,
        /// 用户名
        username: String,
        /// 密码
        password: String,
        /// 根目录（可选）
        root: Option<String>,
    },
    /// S3/MinIO 存储
    S3 {
        /// 服务端点
        endpoint: String,
        /// 存储桶
        bucket: String,
        /// Access Key
        access_key: String,
        /// Secret Key
        secret_key: String,
        /// 区域（可选）
        region: Option<String>,
        /// 根目录（可选）
        root: Option<String>,
    },
    /// 阿里云 OSS 存储
    Oss {
        /// 服务端点
        endpoint: String,
        /// 存储桶
        bucket: String,
        /// Access Key
        access_key: String,
        /// Secret Key
        secret_key: String,
        /// 区域（可选）
        region: Option<String>,
    },
    /// Cloudflare R2 存储
    R2 {
        /// 账户 ID
        account_id: String,
        /// 存储桶
        bucket: String,
        /// Access Key
        access_key: String,
        /// Secret Key
        secret_key: String,
        /// 根目录（可选）
        root: Option<String>,
    },
}

/// SynapseApp - 统一应用入口
pub struct SynapseApp {
    /// 认证服务
    auth: Box<dyn AuthService>,
    
    /// 存储后端
    storage: Box<dyn StorageBackend>,
    
    /// 加密器
    cipher: Cipher,
    
    /// 索引器
    indexer: Indexer,
    
    /// 消息服务
    message_service: MessageService,
    
    /// 通知管理器
    notification_manager: NotificationManager,
    
    /// 数据存储（内存版本，内部使用）
    data_store: HashMap<String, DataEntity>,
}

impl SynapseApp {
    /// 创建新的 SynapseApp（本地存储）
    pub async fn new_local(storage_path: &str) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(DiskAuthService::new(jwt_config, "synapse-secret-key", storage_path).await);
        let storage = Box::new(LocalBackend::new(storage_path));
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 创建新的 SynapseApp（WebDAV 存储）
    pub fn new_webdav(
        endpoint: &str,
        username: &str,
        password: &str,
        root: Option<&str>,
    ) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(MemoryAuthService::new(jwt_config, "synapse-secret-key"));
        let storage: Box<dyn StorageBackend> = match root {
            Some(r) => Box::new(storage_backends::WebdavBackend::with_root(
                endpoint, username, password, r,
            )?),
            None => Box::new(storage_backends::WebdavBackend::new(
                endpoint, username, password,
            )?),
        };
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 创建新的 SynapseApp（S3/MinIO 存储）
    pub fn new_s3(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: Option<&str>,
        root: Option<&str>,
    ) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(MemoryAuthService::new(jwt_config, "synapse-secret-key"));
        
        let storage: Box<dyn StorageBackend> = match (region, root) {
            (Some(r), Some(rt)) => Box::new(storage_backends::S3Backend::with_config(
                endpoint, bucket, access_key, secret_key, r, rt,
            )?),
            (Some(r), None) => Box::new(storage_backends::S3Backend::with_config(
                endpoint, bucket, access_key, secret_key, r, "",
            )?),
            (None, Some(rt)) => Box::new(storage_backends::S3Backend::with_config(
                endpoint, bucket, access_key, secret_key, "auto", rt,
            )?),
            (None, None) => Box::new(storage_backends::S3Backend::new(
                endpoint, bucket, access_key, secret_key,
            )?),
        };
        
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 创建新的 SynapseApp（阿里云 OSS 存储）
    pub fn new_oss(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: Option<&str>,
    ) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(MemoryAuthService::new(jwt_config, "synapse-secret-key"));
        
        let storage: Box<dyn StorageBackend> = match region {
            Some(r) => Box::new(storage_backends::OssBackend::with_region(
                endpoint, bucket, access_key, secret_key, r,
            )?),
            None => Box::new(storage_backends::OssBackend::new(
                endpoint, bucket, access_key, secret_key,
            )?),
        };
        
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 创建新的 SynapseApp（Cloudflare R2 存储）
    pub fn new_r2(
        account_id: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        root: Option<&str>,
    ) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(MemoryAuthService::new(jwt_config, "synapse-secret-key"));
        
        let storage: Box<dyn StorageBackend> = match root {
            Some(r) => Box::new(storage_backends::R2Backend::with_root(
                account_id, bucket, access_key, secret_key, r,
            )?),
            None => Box::new(storage_backends::R2Backend::new(
                account_id, bucket, access_key, secret_key,
            )?),
        };
        
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 创建新的 SynapseApp（通用存储后端）
    pub fn new_with_storage(
        auth: Box<dyn AuthService>,
        storage: Box<dyn StorageBackend>,
    ) -> SynapseResult<Self> {
        let cipher = Cipher::new()?;
        
        Ok(Self {
            auth,
            storage,
            cipher,
            indexer: Indexer::new(),
            message_service: MessageService::new(),
            notification_manager: NotificationManager::new(),
            data_store: HashMap::new(),
        })
    }
    
    /// 用户登录
    pub async fn login(&self, username: &str, password: &str) -> SynapseResult<String> {
        let result = self.auth.login(username, password).await?;
        Ok(result.access_token)
    }
    
    /// 存储加密数据
    pub async fn secure_store(
        &mut self,
        token: &str,
        data_type: DataType,
        data: Vec<u8>,
        tags: Vec<String>,
    ) -> SynapseResult<DataEntity> {
        // 1. 验证权限
        let claims = self.auth.verify_token(token).await?;
        
        // 2. 加密数据
        let encrypted = self.cipher.encrypt(&data, None)?;
        
        // 3. 创建数据实体
        let owner_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| SynapseError::Internal(e.to_string()))?;
        let mut entity = DataEntity::new(owner_id, data_type, encrypted);
        entity = entity.with_tags(tags.clone());
        
        // 4. 存储数据
        let key = format!("data/{}", entity.id);
        self.storage.save(&key, &entity.to_msgpack()?).await?;
        
        // 5. 更新索引
        let entry = search_indexer::IndexEntry {
            id: entity.id.to_string(),
            content: String::from_utf8_lossy(&data).to_string(),
            metadata: HashMap::from([
                ("type".to_string(), entity.data_type.to_string()),
                ("tags".to_string(), tags.join(",")),
            ]),
        };
        self.indexer.add_entry(entry);
        
        // 6. 存储到内存
        self.data_store.insert(entity.id.to_string(), entity.clone());
        
        Ok(entity)
    }
    
    /// 获取数据
    pub async fn secure_get(&self, token: &str, id: &str) -> SynapseResult<DataEntity> {
        // 1. 验证权限
        let claims = self.auth.verify_token(token).await?;
        
        // 2. 获取数据
        let key = format!("data/{}", id);
        let data = self.storage.load(&key).await?;
        let entity = DataEntity::from_msgpack(&data)?;
        
        // 3. 检查所有权
        if entity.owner_id.to_string() != claims.sub {
            return Err(SynapseError::PermissionDenied("Not owner".to_string()));
        }
        
        Ok(entity)
    }

    /// 获取并解密数据
    pub async fn secure_get_decrypted(&self, token: &str, id: &str) -> SynapseResult<(DataEntity, Vec<u8>)> {
        let entity = self.secure_get(token, id).await?;
        let decrypted = self.cipher.decrypt(&entity.encrypted_content, None)?;
        Ok((entity, decrypted))
    }
    
    /// 搜索数据
    pub fn search(&self, query: &str, limit: usize) -> Vec<search_indexer::IndexEntry> {
        self.indexer.search(query, limit)
            .into_iter()
            .cloned()
            .collect()
    }
    
    /// 删除数据
    pub async fn secure_delete(&mut self, token: &str, id: &str) -> SynapseResult<()> {
        // 1. 验证权限
        let claims = self.auth.verify_token(token).await?;
        
        // 2. 检查数据是否存在
        let key = format!("data/{}", id);
        let data = self.storage.load(&key).await?;
        let entity = DataEntity::from_msgpack(&data)?;
        
        // 3. 检查所有权
        if entity.owner_id.to_string() != claims.sub {
            return Err(SynapseError::PermissionDenied("Not owner".to_string()));
        }
        
        // 4. 删除数据
        self.storage.delete(&key).await?;
        self.indexer.remove_entry(id);
        self.data_store.remove(id);
        
        Ok(())
    }
    
    /// 发送消息
    pub fn send_message(
        &mut self,
        _token: &str,
        recipient_id: &str,
        title: &str,
        content: &str,
    ) -> SynapseResult<()> {
        let message = messaging_service::Message::new(
            "system",
            recipient_id,
            title,
            content,
        );
        self.message_service.send_message(message);
        Ok(())
    }
    
    /// 获取用户消息
    pub fn get_messages(&self, user_id: &str, limit: usize) -> Vec<messaging_service::Message> {
        self.message_service.get_user_messages(user_id, limit)
            .into_iter()
            .cloned()
            .collect()
    }
    
    /// 注册新用户
    pub async fn register(&mut self, username: &str, password: &str) -> SynapseResult<String> {
        // 通过认证服务注册并登录
        let result = self.auth.register(username, password).await?;
        Ok(result.access_token)
    }

    /// 更新加密数据
    pub async fn secure_update(
        &mut self,
        token: &str,
        id: &str,
        data: Vec<u8>,
        tags: Vec<String>,
    ) -> SynapseResult<bool> {
        // 1. 验证权限
        let claims = self.auth.verify_token(token).await?;

        // 2. 获取现有数据
        let key = format!("data/{}", id);
        let raw = self.storage.load(&key).await?;
        let mut entity = DataEntity::from_msgpack(&raw)?;

        // 3. 检查所有权
        if entity.owner_id.to_string() != claims.sub {
            return Err(SynapseError::PermissionDenied("Not owner".to_string()));
        }

        // 4. 重新加密数据
        let encrypted = self.cipher.encrypt(&data, None)?;

        // 5. 更新实体
        entity.update_content(encrypted);
        entity.tags = tags.clone();

        // 6. 保存到存储
        self.storage.save(&key, &entity.to_msgpack()?).await?;

        // 7. 更新索引 - 先移除旧条目，再添加新条目
        self.indexer.remove_entry(id);
        let entry = search_indexer::IndexEntry {
            id: entity.id.to_string(),
            content: String::from_utf8_lossy(&data).to_string(),
            metadata: HashMap::from([
                ("type".to_string(), entity.data_type.to_string()),
                ("tags".to_string(), tags.join(",")),
            ]),
        };
        self.indexer.add_entry(entry);

        // 8. 更新内存存储
        self.data_store.insert(id.to_string(), entity);

        Ok(true)
    }

    /// 从磁盘加载所有数据到内存（data_store 和 indexer）
    pub async fn load_from_disk(&mut self) -> SynapseResult<usize> {
        let keys = self.storage.list("data/").await?;
        let mut count = 0;
        for key in &keys {
            if let Ok(data) = self.storage.load(key).await {
                if let Ok(entity) = DataEntity::from_msgpack(&data) {
                    // Add to indexer
                    let entry = search_indexer::IndexEntry {
                        id: entity.id.to_string(),
                        content: String::from_utf8_lossy(&entity.encrypted_content).to_string(),
                        metadata: HashMap::from([
                            ("type".to_string(), entity.data_type.to_string()),
                            ("tags".to_string(), entity.tags.join(",")),
                        ]),
                    };
                    self.indexer.add_entry(entry);
                    self.data_store.insert(entity.id.to_string(), entity);
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// 异步初始化：从磁盘恢复 data_store 和 indexer
    pub async fn init(&mut self) -> SynapseResult<()> {
        let count = self.load_from_disk().await?;
        println!("[synapse] 从磁盘加载了 {} 条数据", count);
        Ok(())
    }

    /// 验证 token
    pub async fn verify_token(&self, token: &str) -> Result<iam_core::Claims, iam_core::AuthError> {
        self.auth.verify_token(token).await
    }

    /// 列出所有数据的基本信息（不含加密内容）
    pub fn list_all_data(&self) -> Vec<DataItemInfo> {
        self.data_store
            .values()
            .map(|entity| DataItemInfo {
                id: entity.id.to_string(),
                data_type: entity.data_type.to_string(),
                tags: entity.tags.clone(),
                created_at: entity.created_at.to_rfc3339(),
            })
            .collect()
    }

    /// 获取数据总数
    pub fn get_data_count(&self) -> usize {
        self.data_store.len()
    }

    /// 获取数据实体（内部使用）
    pub(crate) fn get_data_item(&self, id: &str) -> Option<&DataEntity> {
        self.data_store.get(id)
    }

    /// 按标签搜索数据（返回基本信息，不含加密内容）
    pub fn search_by_tag(&self, tag: &str, limit: usize) -> Vec<DataItemInfo> {
        let tag_lower = tag.to_lowercase();
        self.data_store
            .values()
            .filter(|e| e.tags.iter().any(|t| t.to_lowercase().contains(&tag_lower)))
            .take(limit)
            .map(|entity| DataItemInfo {
                id: entity.id.to_string(),
                data_type: entity.data_type.to_string(),
                tags: entity.tags.clone(),
                created_at: entity.created_at.to_rfc3339(),
            })
            .collect()
    }

    /// 获取统计信息
    pub fn stats(&self) -> ServiceStats {
        ServiceStats {
            data_count: self.data_store.len(),
            index_count: self.indexer.stats().total_entries,
            message_count: self.message_service.total_messages(),
        }
    }
}

/// 服务统计信息
#[derive(Debug, Clone)]
pub struct ServiceStats {
    /// 数据数量
    pub data_count: usize,
    
    /// 索引数量
    pub index_count: usize,
    
    /// 消息数量
    pub message_count: usize,
}

/// 基本数据条目信息（不含加密内容）
#[derive(Debug, Clone)]
pub struct DataItemInfo {
    /// 数据 ID
    pub id: String,
    /// 数据类型
    pub data_type: String,
    /// 标签
    pub tags: Vec<String>,
    /// 创建时间
    pub created_at: String,
}

/// 服务 Trait
#[async_trait]
pub trait SynapseService: Send + Sync {
    /// 存储数据
    async fn store(&self, data_type: DataType, data: Vec<u8>, tags: Vec<String>) -> SynapseResult<String>;
    
    /// 获取数据
    async fn get(&self, id: &str) -> SynapseResult<Vec<u8>>;
    
    /// 删除数据
    async fn delete(&self, id: &str) -> SynapseResult<()>;
    
    /// 搜索数据
    fn search(&self, query: &str, limit: usize) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_synapse_app_creation() {
        let temp_dir = TempDir::new().unwrap();
        let app = SynapseApp::new_local(temp_dir.path().to_str().unwrap()).await.unwrap();
        
        let stats = app.stats();
        assert_eq!(stats.data_count, 0);
    }

    #[tokio::test]
    async fn test_search() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap()).await.unwrap();
        
        let entry = search_indexer::IndexEntry {
            id: "1".to_string(),
            content: "github token".to_string(),
            metadata: std::collections::HashMap::from([
                ("type".to_string(), "credential".to_string()),
            ]),
        };
        app.indexer.add_entry(entry);
        
        let results = app.search("github", 10);
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap()).await.unwrap();
        
        let entry = search_indexer::IndexEntry {
            id: "1".to_string(),
            content: "test".to_string(),
            metadata: std::collections::HashMap::new(),
        };
        app.indexer.add_entry(entry);
        
        let stats = app.stats();
        assert_eq!(stats.index_count, 1);
    }
}
