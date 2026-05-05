//! 阿里云 OSS 存储后端实现
//!
//! 基于 opendal 库实现阿里云 OSS 协议的存储后端。

use opendal::{services::Oss, Operator};

use crate::error::{StorageError, StorageResult};
use crate::opendal_backend::OpendalBackend;
use crate::trait_def::StorageBackend;

/// 阿里云 OSS 存储后端
///
/// This is a thin wrapper around [`OpendalBackend`] that configures the operator
/// for the Alibaba Cloud OSS protocol.
pub struct OssBackend {
    inner: OpendalBackend,
    endpoint: String,
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
            inner: OpendalBackend::new(op, "OSS"),
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
            inner: OpendalBackend::new(op, "OSS"),
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

#[async_trait::async_trait]
impl StorageBackend for OssBackend {
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
        )
        .unwrap();
        assert_eq!(backend.endpoint(), "oss-cn-hangzhou.aliyuncs.com");
        assert_eq!(backend.bucket(), "test-bucket");
    }
}
