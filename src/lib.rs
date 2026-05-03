//! # SynapseCore - 神经突触核心
//!
//! 高性能、模块化的跨平台数据管理系统。
//!
//! This crate is a thin re-export of `synapse_service` and its sub-crates,
//! providing a single entry point for the entire SynapseCore ecosystem.
//!
//! ## 核心功能
//!
//! - 数据实体与加密 (data_core)
//! - 身份认证与授权 (iam_core)
//! - 本地存储 (storage_backends)
//! - 多设备同步 (sync_engine)
//! - 全文搜索 (search_indexer)
//! - 消息服务 (messaging_service)
//! - 统一应用入口 (synapse_service → SynapseApp)

// Re-export synapse_service as the single high-level API
pub use synapse_service::SynapseApp;
pub use synapse_service::error;
pub use synapse_service::{
    DataItemInfo, ServiceStats, StorageType, SynapseService,
};

// Re-export sub-crates as modules for convenience
pub mod data_core {
    pub use ::data_core::*;
}

pub mod iam_core {
    pub use ::iam_core::*;
}

pub mod storage_backends {
    pub use ::storage_backends::*;
}

pub mod search_indexer {
    pub use ::search_indexer::*;
}

pub mod messaging_service {
    pub use ::messaging_service::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_synapse_app_creation() {
        let temp_dir = TempDir::new().unwrap();
        let app = SynapseApp::new_local(temp_dir.path().to_str().unwrap())
            .await
            .unwrap();

        let stats = app.stats();
        assert_eq!(stats.data_count, 0);
    }

    #[tokio::test]
    async fn test_secure_store_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap())
            .await
            .unwrap();

        // Register a user and get token
        let token = app.register("alice", "password123").await.unwrap();

        // Store data
        let entity = app
            .secure_store(
                &token,
                data_core::DataType::Credential,
                b"secret data".to_vec(),
                vec!["github".to_string()],
            )
            .await
            .unwrap();

        // Get data back
        let retrieved = app.secure_get(&token, &entity.id.to_string()).await.unwrap();
        assert_eq!(retrieved.id, entity.id);
    }

    #[tokio::test]
    async fn test_secure_delete() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap())
            .await
            .unwrap();

        let token = app.register("bob", "password123").await.unwrap();

        let entity = app
            .secure_store(
                &token,
                data_core::DataType::Config,
                b"config data".to_vec(),
                vec![],
            )
            .await
            .unwrap();

        app.secure_delete(&token, &entity.id.to_string())
            .await
            .unwrap();
        assert_eq!(app.get_data_count(), 0);
    }

    #[tokio::test]
    async fn test_search_via_indexer() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap())
            .await
            .unwrap();

        let token = app.register("carol", "password123").await.unwrap();

        app.secure_store(
            &token,
            data_core::DataType::Credential,
            b"github token".to_vec(),
            vec!["github".to_string()],
        )
        .await
        .unwrap();

        let results = app.search("github", 10);
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = SynapseApp::new_local(temp_dir.path().to_str().unwrap())
            .await
            .unwrap();

        let token = app.register("dave", "password123").await.unwrap();

        app.secure_store(
            &token,
            data_core::DataType::Generic,
            b"test".to_vec(),
            vec![],
        )
        .await
        .unwrap();

        let stats = app.stats();
        assert_eq!(stats.data_count, 1);
    }
}
