//! 通知管理模块
//! 
//! 管理通知的发送和接收。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    /// 数据更新
    DataUpdate,
    /// 数据同步
    DataSync,
    /// 冲突检测
    ConflictDetected,
    /// 任务分配
    TaskAssignment,
    /// 任务完成
    TaskCompletion,
    /// 系统通知
    System,
}

/// 通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 通知 ID
    pub id: String,
    
    /// 通知类型
    pub notification_type: NotificationType,
    
    /// 标题
    pub title: String,
    
    /// 内容
    pub content: String,
    
    /// 创建时间
    pub created_at: DateTime<Utc>,
    
    /// 是否已读
    pub is_read: bool,
    
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl Notification {
    /// 创建新通知
    pub fn new(
        notification_type: NotificationType,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            notification_type,
            title: title.into(),
            content: content.into(),
            created_at: Utc::now(),
            is_read: false,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// 标记为已读
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }
}

/// 通知管理器
pub struct NotificationManager {
    /// 通知列表
    notifications: Vec<Notification>,
}

impl NotificationManager {
    /// 创建新的通知管理器
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    
    /// 添加通知
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }
    
    /// 获取未读通知
    pub fn get_unread(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| !n.is_read)
            .collect()
    }
    
    /// 获取通知数量
    pub fn count(&self) -> usize {
        self.notifications.len()
    }
    
    /// 获取未读数量
    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.is_read).count()
    }
    
    /// 标记通知为已读
    pub fn mark_as_read(&mut self, notification_id: &str) -> bool {
        if let Some(notification) = self.notifications.iter_mut().find(|n| n.id == notification_id) {
            notification.mark_as_read();
            true
        } else {
            false
        }
    }
    
    /// 清除已读通知
    pub fn clear_read(&mut self) {
        self.notifications.retain(|n| !n.is_read);
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notification = Notification::new(
            NotificationType::DataUpdate,
            "Test",
            "Hello",
        );
        assert!(!notification.is_read);
    }

    #[test]
    fn test_notification_manager() {
        let mut manager = NotificationManager::new();
        
        let notification = Notification::new(
            NotificationType::DataUpdate,
            "Test",
            "Hello",
        );
        manager.add_notification(notification);
        
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.unread_count(), 1);
        
        let unread = manager.get_unread();
        assert_eq!(unread.len(), 1);
    }
}
