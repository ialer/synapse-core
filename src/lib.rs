//! # SynapseCore - 神经突触核心
//! 
//! 高性能、模块化的跨平台数据管理系统。
//! 
//! ## 核心功能
//! 
//! - 数据实体与加密 (data_core)
//! - 身份认证与授权 (iam_core)
//! - 本地存储 (storage_backends)
//! - 多设备同步 (sync_engine)
//! - 全文搜索 (search_indexer)
//! - 消息服务 (messaging_service)
//! - Agent 接口 (agent_interface)

/// 数据核心模块
pub mod data_core {
    pub use data_core::*;
}

/// 身份认证模块
pub mod iam_core {
    pub use iam_core::*;
}

/// 存储后端模块
pub mod storage_backends {
    pub use storage_backends::*;
}

/// 同步引擎模块
pub mod sync_engine {
    pub use sync_engine::*;
}

/// 搜索索引模块
pub mod search_indexer {
    pub use search_indexer::*;
}

/// 消息服务模块
pub mod messaging_service {
    pub use messaging_service::*;
}

/// Agent 接口模块
pub mod agent_interface {
    pub use agent_interface::*;
}

use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// SynapseCore 主 API
pub struct SynapseCore {
    /// 设备 ID
    device_id: String,
    
    /// 用户 ID
    user_id: String,
    
    /// 数据存储
    data_store: HashMap<String, data_core::DataEntity>,
    
    /// 索引器
    indexer: search_indexer::Indexer,
    
    /// 消息服务
    message_service: messaging_service::MessageService,
    
    /// 通知管理器
    notification_manager: messaging_service::NotificationManager,
}

impl SynapseCore {
    /// 创建新的 SynapseCore 实例
    pub fn new(device_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            user_id: user_id.into(),
            data_store: HashMap::new(),
            indexer: search_indexer::Indexer::new(),
            message_service: messaging_service::MessageService::new(),
            notification_manager: messaging_service::NotificationManager::new(),
        }
    }
    
    /// 存储数据
    pub fn store_data(
        &mut self,
        data_type: data_core::DataType,
        content: Vec<u8>,
        tags: Vec<String>,
    ) -> data_core::DataEntity {
        let mut entity = data_core::DataEntity::new(
            uuid::Uuid::parse_str(&self.user_id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            data_type,
            content,
        );
        entity = entity.with_tags(tags.clone());
        
        // 添加到索引
        let entry = search_indexer::IndexEntry {
            id: entity.id.to_string(),
            content: String::from_utf8_lossy(&entity.encrypted_content).to_string(),
            metadata: HashMap::from([
                ("type".to_string(), entity.data_type.to_string()),
                ("tags".to_string(), tags.join(",")),
            ]),
        };
        self.indexer.add_entry(entry);
        
        // 存储
        self.data_store.insert(entity.id.to_string(), entity.clone());
        
        entity
    }
    
    /// 获取数据
    pub fn get_data(&self, id: &str) -> Option<&data_core::DataEntity> {
        self.data_store.get(id)
    }
    
    /// 删除数据
    pub fn delete_data(&mut self, id: &str) -> bool {
        if let Some(entity) = self.data_store.remove(id) {
            self.indexer.remove_entry(id);
            true
        } else {
            false
        }
    }
    
    /// 搜索数据
    pub fn search_data(&self, query: &str, limit: usize) -> Vec<&search_indexer::IndexEntry> {
        self.indexer.search(query, limit)
    }
    
    /// 按元数据搜索
    pub fn search_by_metadata(&self, key: &str, value: &str, limit: usize) -> Vec<&search_indexer::IndexEntry> {
        self.indexer.search_by_metadata(key, value, limit)
    }
    
    /// 发送消息
    pub fn send_message(
        &mut self,
        recipient_id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) {
        let message = messaging_service::Message::new(
            &self.user_id,
            recipient_id,
            title,
            content,
        );
        self.message_service.send_message(message);
    }
    
    /// 获取用户消息
    pub fn get_messages(&self, limit: usize) -> Vec<&messaging_service::Message> {
        self.message_service.get_user_messages(&self.user_id, limit)
    }
    
    /// 获取未读消息
    pub fn get_unread_messages(&self) -> Vec<&messaging_service::Message> {
        self.message_service.get_unread_messages(&self.user_id)
    }
    
    /// 添加通知
    pub fn add_notification(
        &mut self,
        notification_type: messaging_service::NotificationType,
        title: impl Into<String>,
        content: impl Into<String>,
    ) {
        let notification = messaging_service::Notification::new(
            notification_type,
            title,
            content,
        );
        self.notification_manager.add_notification(notification);
    }
    
    /// 获取未读通知
    pub fn get_unread_notifications(&self) -> Vec<&messaging_service::Notification> {
        self.notification_manager.get_unread()
    }
    
    /// 获取设备 ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    
    /// 获取用户 ID
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    
    /// 获取数据数量
    pub fn data_count(&self) -> usize {
        self.data_store.len()
    }
    
    /// 获取统计信息
    pub fn stats(&self) -> SynapseStats {
        SynapseStats {
            device_id: self.device_id.clone(),
            user_id: self.user_id.clone(),
            data_count: self.data_store.len(),
            index_count: self.indexer.stats().total_entries,
            message_count: self.message_service.total_messages(),
            unread_messages: self.message_service.get_unread_messages(&self.user_id).len(),
            unread_notifications: self.notification_manager.unread_count(),
        }
    }
}

/// 统计信息
#[derive(Debug, Clone)]
pub struct SynapseStats {
    /// 设备 ID
    pub device_id: String,
    
    /// 用户 ID
    pub user_id: String,
    
    /// 数据数量
    pub data_count: usize,
    
    /// 索引数量
    pub index_count: usize,
    
    /// 消息数量
    pub message_count: usize,
    
    /// 未读消息
    pub unread_messages: usize,
    
    /// 未读通知
    pub unread_notifications: usize,
}

impl std::fmt::Display for SynapseStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SynapseStats {{ device: {}, user: {}, data: {}, messages: {}, unread: {} }}",
            self.device_id,
            self.user_id,
            self.data_count,
            self.message_count,
            self.unread_messages
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synapse_core_creation() {
        let core = SynapseCore::new("device-1", "user-1");
        assert_eq!(core.device_id(), "device-1");
        assert_eq!(core.user_id(), "user-1");
    }

    #[test]
    fn test_store_and_get_data() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        let entity = core.store_data(
            data_core::DataType::Credential,
            b"secret data".to_vec(),
            vec!["github".to_string()],
        );
        
        let retrieved = core.get_data(&entity.id.to_string());
        assert!(retrieved.is_some());
        assert_eq!(core.data_count(), 1);
    }

    #[test]
    fn test_delete_data() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        let entity = core.store_data(
            data_core::DataType::Config,
            b"config data".to_vec(),
            vec![],
        );
        
        assert!(core.delete_data(&entity.id.to_string()));
        assert_eq!(core.data_count(), 0);
        assert!(!core.delete_data("nonexistent"));
    }

    #[test]
    fn test_search_data() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        core.store_data(
            data_core::DataType::Credential,
            b"github token".to_vec(),
            vec!["github".to_string()],
        );
        
        let results = core.search_data("github", 10);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_messages() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        core.send_message("user-2", "Test", "Hello");
        
        assert_eq!(core.get_messages(10).len(), 1);
        assert_eq!(core.get_unread_messages().len(), 1);
    }

    #[test]
    fn test_notifications() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        core.add_notification(
            messaging_service::NotificationType::DataUpdate,
            "Update",
            "Data updated",
        );
        
        assert_eq!(core.get_unread_notifications().len(), 1);
    }

    #[test]
    fn test_stats() {
        let mut core = SynapseCore::new("device-1", "user-1");
        
        core.store_data(
            data_core::DataType::Generic,
            b"test".to_vec(),
            vec![],
        );
        
        let stats = core.stats();
        assert_eq!(stats.data_count, 1);
    }
}
