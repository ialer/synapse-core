//! Cloudflare R2 存储后端实现
//! 
//! Cloudflare R2 是 S3 兼容的对象存储服务。
//! 此模块提供针对 R2 优化的存储后端实现。

use async_trait::async_trait;
use opendal::{services::S3, Operator};

use crate::error::{StorageError, StorageResult};
use crate::trait_def::StorageBackend;

/// Cloudflare R2 存储后端
/// 
/// R2 是 S3 兼容的，使用 S3 协议但有一些配置差异：
/// - 使用 Cloudflare 账户 ID 作为端点的一部分
/// - 需要启用路径风格访问
/// - 区域固定为 "auto"
pub struct R2Backend {
    /// opendal 操作器
    op: Operator,
    
    /// 服务端点
    endpoint: String,
    
    /// 存储桶
    bucket: String,
    
    /// 账户 ID
    account_id: String,
}

impl R2Backend {
    /// 创建新的 R2 存储后端
    /// 
    /// # 参数
    /// * `account_id` - Cloudflare 账户 ID
    /// * `bucket` - R2 存储桶名称
    /// * `access_key` - R2 API Token 的 Access Key ID
    /// * `secret_key` - R2 API Token 的 Secret Access Key
    /// 
    /// # 示例
    /// ```ignore
    /// let backend = R2Backend::new(
    ///     "your-account-id",
    ///     "my-bucket",
    ///     "your-access-key",
    ///     "your-secret-key",
    /// )?;
    /// ```
    pub fn new(
        account_id: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> StorageResult<Self> {
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", account_id);
        
        let builder = S3::default()
            .endpoint(&endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .region("auto");
        
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create R2 operator: {}", e)))?
            .finish();
        
        Ok(Self {
            op,
            endpoint,
            bucket: bucket.to_string(),
            account_id: account_id.to_string(),
        })
    }
    
    /// 创建带有自定义根目录的 R2 后端
    pub fn with_root(
        account_id: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        root: &str,
    ) -> StorageResult<Self> {
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", account_id);
        
        let builder = S3::default()
            .endpoint(&endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .region("auto")
            .root(root);
        
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create R2 operator: {}", e)))?
            .finish();
        
        Ok(Self {
            op,
            endpoint,
            bucket: bucket.to_string(),
            account_id: account_id.to_string(),
        })
    }
    
    /// 创建使用自定义端点的 R2 后端（用于开发或测试）
    pub fn with_endpoint(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> StorageResult<Self> {
        let builder = S3::default()
            .endpoint(endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .region("auto");
        
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create R2 operator: {}", e)))?
            .finish();
        
        Ok(Self {
            op,
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
            account_id: String::new(),
        })
    }
    
    /// 获取服务端点
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
    
    /// 获取存储桶名称
    pub fn bucket(&self) -> &str {
        &self.bucket
    }
    
    /// 获取账户 ID
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
}

#[async_trait]
impl StorageBackend for R2Backend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        let _ = self.op
            .write(key, data.to_vec())
            .await
            .map_err(|e| StorageError::Network(format!("R2 write failed: {}", e)))?;
        Ok(())
    }
    
    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        self.op
            .read(key)
            .await
            .map(|data| data.to_vec())
            .map_err(|e| StorageError::Network(format!("R2 read failed: {}", e)))
    }
    
    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let entries = self.op
            .list(prefix)
            .await
            .map_err(|e| StorageError::Network(format!("R2 list failed: {}", e)))?;
        
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
            .map_err(|e| StorageError::Network(format!("R2 delete failed: {}", e)))
    }
    
    async fn exists(&self, key: &str) -> StorageResult<bool> {
        match self.op.stat(key).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("not found") {
                    Ok(false)
                } else {
                    Err(StorageError::Network(format!("R2 exists check failed: {}", e)))
                }
            }
        }
    }
    
    async fn size(&self, key: &str) -> StorageResult<u64> {
        let meta = self.op
            .stat(key)
            .await
            .map_err(|e| StorageError::Network(format!("R2 stat failed: {}", e)))?;
        
        Ok(meta.content_length())
    }
    
    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .copy(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("R2 copy failed: {}", e)))
    }
    
    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        // R2 不支持原生 rename，使用 copy + delete 实现
        self.copy(src, dst).await?;
        self.delete(src).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_r2_backend_creation() {
        let result = R2Backend::new(
            "test-account-id",
            "test-bucket",
            "access-key",
            "secret-key",
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_r2_backend_endpoint() {
        let backend = R2Backend::new(
            "test-account-id",
            "test-bucket",
            "access-key",
            "secret-key",
        ).unwrap();
        assert_eq!(backend.endpoint(), "https://test-account-id.r2.cloudflarestorage.com");
        assert_eq!(backend.bucket(), "test-bucket");
        assert_eq!(backend.account_id(), "test-account-id");
    }
    
    #[test]
    fn test_r2_backend_custom_endpoint() {
        let backend = R2Backend::with_endpoint(
            "http://localhost:9000",
            "test-bucket",
            "access-key",
            "secret-key",
        ).unwrap();
        assert_eq!(backend.endpoint(), "http://localhost:9000");
    }
}
