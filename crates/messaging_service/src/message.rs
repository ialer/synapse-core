//! 消息处理模块
//! 
//! 定义消息类型和处理逻辑。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// 普通消息
    Text,
    /// 通知
    Notification,
    /// 系统消息
    System,
    /// 任务分配
    TaskAssignment,
    /// 任务完成
    TaskCompletion,
    /// 错误报告
    ErrorReport,
}

/// 消息优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    /// 低
    Low,
    /// 普通
    Normal,
    /// 高
    High,
    /// 紧急
    Urgent,
}

/// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息 ID
    pub id: String,
    
    /// 发送者 ID
    pub sender_id: String,
    
    /// 接收者 ID
    pub recipient_id: String,
    
    /// 消息类型
    pub message_type: MessageType,
    
    /// 消息优先级
    pub priority: MessagePriority,
    
    /// 消息标题
    pub title: String,
    
    /// 消息内容
    pub content: String,
    
    /// 发送时间
    pub sent_at: DateTime<Utc>,
    
    /// 已读时间
    pub read_at: Option<DateTime<Utc>>,
    
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl Message {
    /// 创建新消息
    pub fn new(
        sender_id: impl Into<String>,
        recipient_id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            sender_id: sender_id.into(),
            recipient_id: recipient_id.into(),
            message_type: MessageType::Text,
            priority: MessagePriority::Normal,
            title: title.into(),
            content: content.into(),
            sent_at: Utc::now(),
            read_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// 设置消息类型
    pub fn with_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }
    
    /// 设置优先级
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }
    
    /// 标记为已读
    pub fn mark_as_read(&mut self) {
        self.read_at = Some(Utc::now());
    }
    
    /// 检查是否已读
    pub fn is_read(&self) -> bool {
        self.read_at.is_some()
    }
}

/// 消息服务
pub struct MessageService {
    /// 消息存储
    messages: Vec<Message>,
}

impl MessageService {
    /// 创建新的消息服务
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    /// 发送消息
    pub fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    
    /// 获取用户消息
    pub fn get_user_messages(&self, user_id: &str, limit: usize) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|m| m.recipient_id == user_id)
            .take(limit)
            .collect()
    }
    
    /// 获取未读消息
    pub fn get_unread_messages(&self, user_id: &str) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|m| m.recipient_id == user_id && !m.is_read())
            .collect()
    }
    
    /// 标记消息为已读
    pub fn mark_as_read(&mut self, message_id: &str) -> bool {
        if let Some(message) = self.messages.iter_mut().find(|m| m.id == message_id) {
            message.mark_as_read();
            true
        } else {
            false
        }
    }
    
    /// 删除消息
    pub fn delete_message(&mut self, message_id: &str) -> bool {
        let len = self.messages.len();
        self.messages.retain(|m| m.id != message_id);
        self.messages.len() < len
    }
    
    /// 获取消息总数
    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new("user1", "user2", "Test", "Hello");
        assert_eq!(message.sender_id, "user1");
        assert_eq!(message.recipient_id, "user2");
        assert!(!message.is_read());
    }

    #[test]
    fn test_message_read() {
        let mut message = Message::new("user1", "user2", "Test", "Hello");
        message.mark_as_read();
        assert!(message.is_read());
    }

    #[test]
    fn test_message_service() {
        let mut service = MessageService::new();
        
        let message = Message::new("user1", "user2", "Test", "Hello");
        service.send_message(message);
        
        assert_eq!(service.total_messages(), 1);
        
        let messages = service.get_user_messages("user2", 10);
        assert_eq!(messages.len(), 1);
        
        let unread = service.get_unread_messages("user2");
        assert_eq!(unread.len(), 1);
    }
}
