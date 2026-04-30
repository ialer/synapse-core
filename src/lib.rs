//! # SynapseCore - 神经突触核心
//! 
//! 高性能、模块化的跨平台数据管理系统。
//! 
//! ## 架构概览
//! 
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    SynapseCore Architecture                 │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                             │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
//! │  │  data_core  │  │  iam_core   │  │   messaging │        │
//! │  │  (数据核心) │  │  (身份认证) │  │   (消息服务)│        │
//! │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘        │
//! │         │                │                │                │
//! │         ▼                ▼                ▼                │
//! │  ┌─────────────────────────────────────────────────────┐  │
//! │  │              storage_backends (存储后端)              │  │
//! │  └─────────────────────────────────────────────────────┘  │
//! │                          │                                │
//! │         ┌────────────────┼────────────────┐               │
//! │         ▼                ▼                ▼               │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
//! │  │sync_engine  │  │search_index │  │agent_interface│     │
//! │  │ (同步引擎)  │  │ (搜索索引)  │  │ (Agent接口)  │     │
//! │  └─────────────┘  └─────────────┘  └─────────────┘      │
//! │                                                             │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//! 
//! ## 模块说明
//! 
//! - `data_core`: 数据实体定义与加密核心
//! - `iam_core`: 身份认证与访问控制
//! - `storage_backends`: 存储后端抽象
//! - `sync_engine`: 多设备同步引擎
//! - `search_indexer`: 全文搜索索引
//! - `agent_interface`: MCP协议Agent接口
//! - `messaging_service`: 消息服务与通知

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

/// Agent接口模块
pub mod agent_interface {
    pub use agent_interface::*;
}

/// 消息服务模块
pub mod messaging_service {
    pub use messaging_service::*;
}
