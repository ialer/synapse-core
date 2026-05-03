//! WebDAV 存储后端实现
//!
//! 基于 opendal 库实现 WebDAV 协议的存储后端。

use opendal::{services::Webdav, Operator};
use url::Url;

use crate::error::{StorageError, StorageResult};
use crate::opendal_backend::OpendalBackend;
use crate::trait_def::StorageBackend;

/// WebDAV 存储后端
///
/// This is a thin wrapper around [`OpendalBackend`] that configures the operator
/// for the WebDAV protocol.
pub struct WebdavBackend {
    inner: OpendalBackend,
    endpoint: String,
}

impl WebdavBackend {
    /// 创建新的 WebDAV 存储后端
    pub fn new(endpoint: &str, username: &str, password: &str) -> StorageResult<Self> {
        // 验证 URL 格式
        Url::parse(endpoint)
            .map_err(|e| StorageError::Config(format!("Invalid URL: {}", e)))?;

        // 配置 WebDAV 服务（链式调用）
        let builder = Webdav::default()
            .endpoint(endpoint)
            .username(username)
            .password(password);

        // 创建操作器
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create operator: {}", e)))?
            .finish();

        Ok(Self {
            inner: OpendalBackend::new(op, "WebDAV"),
            endpoint: endpoint.to_string(),
        })
    }

    /// 创建带有自定义根目录的 WebDAV 后端
    pub fn with_root(
        endpoint: &str,
        username: &str,
        password: &str,
        root: &str,
    ) -> StorageResult<Self> {
        let builder = Webdav::default()
            .endpoint(endpoint)
            .username(username)
            .password(password)
            .root(root);

        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create operator: {}", e)))?
            .finish();

        Ok(Self {
            inner: OpendalBackend::new(op, "WebDAV"),
            endpoint: endpoint.to_string(),
        })
    }

    /// 获取服务端点
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[async_trait::async_trait]
impl StorageBackend for WebdavBackend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        self.inner.save(key, data).await
    }

    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        self.inner.load(key).await
    }

    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        self.inner.list(prefix).await
    }

    async fn delete(&self, key: &str) -> StorageResult<()> {
        self.inner.delete(key).await
    }

    async fn exists(&self, key: &str) -> StorageResult<bool> {
        self.inner.exists(key).await
    }

    async fn size(&self, key: &str) -> StorageResult<u64> {
        self.inner.size(key).await
    }

    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.inner.copy(src, dst).await
    }

    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.inner.rename(src, dst).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 注意：WebDAV 测试需要实际的 WebDAV 服务器
    // 以下为单元测试，验证接口正确性

    #[test]
    fn test_webdav_backend_creation() {
        // 测试 URL 验证
        let result = WebdavBackend::new("invalid-url", "user", "pass");
        assert!(result.is_err());

        let result = WebdavBackend::new("https://example.com/webdav", "user", "pass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webdav_backend_endpoint() {
        let backend =
            WebdavBackend::new("https://example.com/webdav", "user", "pass").unwrap();
        assert_eq!(backend.endpoint(), "https://example.com/webdav");
    }
}
