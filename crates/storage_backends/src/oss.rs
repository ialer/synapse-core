//! 阿里云 OSS 存储后端实现
//! 
//! 基于 opendal 库实现阿里云 OSS 协议的存储后端。

use async_trait::async_trait;
use opendal::{services::Oss, Operator};

use crate::error::{StorageError, StorageResult};
use crate::trait_def::StorageBackend;

/// 阿里云 OSS 存储后端
pub struct OssBackend {
    /// opendal 操作器
    op: Operator,
    
    /// 服务端点
    endpoint: String,
    
    /// 存储桶
    bucket: String,
}

impl OssBackend {
    /// 创建新的 OSS 存储后端
    pub fn new(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> StorageResult<Self> {
        let builder = Oss::default()
            .endpoint(endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .access_key_secret(secret_key);
        
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create OSS operator: {}", e)))?
            .finish();
        
        Ok(Self {
            op,
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
        })
    }
    
    /// 创建带有自定义区域的 OSS 后端
    pub fn with_region(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: &str,
    ) -> StorageResult<Self> {
        let builder = Oss::default()
            .endpoint(endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .access_key_secret(secret_key)
            .root(&format!("/{}/", region));
        
        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create OSS operator: {}", e)))?
            .finish();
        
        Ok(Self {
            op,
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
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
}

#[async_trait]
impl StorageBackend for OssBackend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        let _ = self.op
            .write(key, data.to_vec())
            .await
            .map_err(|e| StorageError::Network(format!("OSS write failed: {}", e)))?;
        Ok(())
    }
    
    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        self.op
            .read(key)
            .await
            .map(|data| data.to_vec())
            .map_err(|e| StorageError::Network(format!("OSS read failed: {}", e)))
    }
    
    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let entries = self.op
            .list(prefix)
            .await
            .map_err(|e| StorageError::Network(format!("OSS list failed: {}", e)))?;
        
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
            .map_err(|e| StorageError::Network(format!("OSS delete failed: {}", e)))
    }
    
    async fn exists(&self, key: &str) -> StorageResult<bool> {
        match self.op.stat(key).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("not found") {
                    Ok(false)
                } else {
                    Err(StorageError::Network(format!("OSS exists check failed: {}", e)))
                }
            }
        }
    }
    
    async fn size(&self, key: &str) -> StorageResult<u64> {
        let meta = self.op
            .stat(key)
            .await
            .map_err(|e| StorageError::Network(format!("OSS stat failed: {}", e)))?;
        
        Ok(meta.content_length())
    }
    
    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .copy(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("OSS copy failed: {}", e)))
    }
    
    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .rename(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("OSS rename failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oss_backend_creation() {
        let result = OssBackend::new(
            "oss-cn-hangzhou.aliyuncs.com",
            "test-bucket",
            "access-key",
            "secret-key",
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_oss_backend_endpoint() {
        let backend = OssBackend::new(
            "oss-cn-hangzhou.aliyuncs.com",
            "test-bucket",
            "access-key",
            "secret-key",
        ).unwrap();
        assert_eq!(backend.endpoint(), "oss-cn-hangzhou.aliyuncs.com");
        assert_eq!(backend.bucket(), "test-bucket");
    }
}
