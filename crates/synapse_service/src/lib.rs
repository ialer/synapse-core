//! 统一网关服务模块
//! 
//! 整合 IAM、Storage 和 Data 核心，对外提供统一的 API 接口。

pub mod error;

use std::collections::HashMap;
use async_trait::async_trait;

use crate::error::{SynapseError, SynapseResult};
use data_core::{DataEntity, DataType, Cipher};
use iam_core::{AuthService, JwtConfig, MemoryAuthService, Role};
use storage_backends::{StorageBackend, LocalBackend};
use search_indexer::Indexer;
use messaging_service::{MessageService, NotificationManager};

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
    
    /// 数据存储（内存版本）
    data_store: HashMap<String, DataEntity>,
}

impl SynapseApp {
    /// 创建新的 SynapseApp
    pub fn new(storage_path: &str) -> SynapseResult<Self> {
        let jwt_config = JwtConfig::default();
        let auth = Box::new(MemoryAuthService::new(jwt_config, "synapse-secret-key"));
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
        token: &str,
        recipient_id: &str,
        title: &str,
        content: &str,
    ) -> SynapseResult<()> {
        // 简化版本：不验证 token
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
        let app = SynapseApp::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        let stats = app.stats();
        assert_eq!(stats.data_count, 0);
    }

    #[tokio::test]
    async fn test_search() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        // 手动添加数据到索引
        let entry = search_indexer::IndexEntry {
            id: "1".to_string(),
            content: "github token".to_string(),
            metadata: std::collections::HashMap::from([
                ("type".to_string(), "credential".to_string()),
            ]),
        };
        app.indexer.add_entry(entry);
        
        // 搜索数据
        let results = app.search("github", 10);
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        // 添加测试数据
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
