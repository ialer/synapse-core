//! WebDAV 存储后端实现
//! 
//! 基于 opendal 库实现 WebDAV 协议的存储后端。

use async_trait::async_trait;
use opendal::{services::Webdav, Operator};
use url::Url;

use crate::error::{StorageError, StorageResult};
use crate::trait_def::StorageBackend;

/// WebDAV 存储后端
pub struct WebdavBackend {
    /// opendal 操作器
    op: Operator,
    
    /// 服务端点
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
            op,
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
            op,
            endpoint: endpoint.to_string(),
        })
    }
    
    /// 获取服务端点
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[async_trait]
impl StorageBackend for WebdavBackend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        // write() returns Metadata on success
        let _ = self.op
            .write(key, data.to_vec())
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV write failed: {}", e)))?;
        Ok(())
    }
    
    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        self.op
            .read(key)
            .await
            .map(|data| data.to_vec())
            .map_err(|e| StorageError::Network(format!("WebDAV read failed: {}", e)))
    }
    
    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let entries = self.op
            .list(prefix)
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV list failed: {}", e)))?;
        
        // 将 Entry 转换为 String
        let paths: Vec<String> = entries
            .into_iter()
            .map(|entry| entry.path().to_string())
            .collect();
        
        Ok(paths)
    }
    
    async fn delete(&self, key: &str) -> StorageResult<()> {
        self.op
            .delete(key)
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV delete failed: {}", e)))
    }
    
    async fn exists(&self, key: &str) -> StorageResult<bool> {
        match self.op.stat(key).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("not found") {
                    Ok(false)
                } else {
                    Err(StorageError::Network(format!("WebDAV exists check failed: {}", e)))
                }
            }
        }
    }
    
    async fn size(&self, key: &str) -> StorageResult<u64> {
        let meta = self.op
            .stat(key)
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV stat failed: {}", e)))?;
        
        // content_length() returns u64 directly
        Ok(meta.content_length())
    }
    
    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .copy(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV copy failed: {}", e)))
    }
    
    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .rename(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("WebDAV rename failed: {}", e)))
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
        let backend = WebdavBackend::new("https://example.com/webdav", "user", "pass").unwrap();
        assert_eq!(backend.endpoint(), "https://example.com/webdav");
    }
}
