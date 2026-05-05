//! # Messaging Service - 消息服务模块
//!
//! 提供消息处理与通知管理功能。

pub mod error;
pub mod message;
pub mod notification;

pub use error::{MessageError, MessageResult};
pub use message::{Message, MessagePriority, MessageService, MessageType};
pub use notification::{Notification, NotificationManager, NotificationType};
