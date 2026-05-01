//! # Messaging Service - 消息服务模块
//! 
//! 提供消息处理与通知管理功能。

pub mod message;
pub mod notification;
pub mod error;

pub use message::{Message, MessageType, MessagePriority, MessageService};
pub use notification::{Notification, NotificationType, NotificationManager};
pub use error::{MessageError, MessageResult};
