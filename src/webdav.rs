//! WebDAV Provider
//!
//! 使用 OpenDAL 的 WebDAV 后端实现远程文件存储访问。
//! 当前为基础实现，后续可扩展完整的 WebDAV 支持。

use async_trait::async_trait;
use crate::provider::*;

/// WebDAV Provider 配置
pub struct WebDavConfig {
    pub endpoint: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub root: String,
}

/// WebDAV Provider
pub struct WebDavProvider {
    config: WebDavConfig,
    connected: std::sync::atomic::AtomicBool,
}

impl WebDavProvider {
    /// 创建新的 WebDAV Provider
    pub fn new(config: WebDavConfig) -> Self {
        Self {
            config,
            connected: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl Provider for WebDavProvider {
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "webdav".to_string(),
            display_name: "WebDAV".to_string(),
            description: format!("WebDAV 服务器: {}", self.config.endpoint),
            version: "0.1.0".to_string(),
            capabilities: vec![
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
                "list".to_string(),
            ],
        }
    }

    async fn connect(&self) -> ProviderResult<()> {
        // TODO: 使用 reqwest 或 opendal 实际连接验证
        self.connected.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn disconnect(&self) -> ProviderResult<()> {
        self.connected.store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::SeqCst)
    }

    async fn list(&self, _prefix: &str) -> ProviderResult<Vec<String>> {
        Err(ProviderError::Internal("WebDAV list 尚未实现".to_string()))
    }

    async fn read(&self, _key: &str) -> ProviderResult<Vec<u8>> {
        Err(ProviderError::Internal("WebDAV read 尚未实现".to_string()))
    }

    async fn write(&self, _key: &str, _data: &[u8]) -> ProviderResult<()> {
        Err(ProviderError::Internal("WebDAV write 尚未实现".to_string()))
    }

    async fn delete(&self, _key: &str) -> ProviderResult<()> {
        Err(ProviderError::Internal("WebDAV delete 尚未实现".to_string()))
    }

    async fn exists(&self, _key: &str) -> ProviderResult<bool> {
        Err(ProviderError::Internal("WebDAV exists 尚未实现".to_string()))
    }

    async fn size(&self, _key: &str) -> ProviderResult<u64> {
        Err(ProviderError::Internal("WebDAV size 尚未实现".to_string()))
    }
}
