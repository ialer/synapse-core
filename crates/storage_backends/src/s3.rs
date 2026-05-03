//! S3/MinIO 存储后端实现
//!
//! 基于 opendal 库实现 S3 协议的存储后端。

use opendal::{services::S3, Operator};

use crate::error::{StorageError, StorageResult};
use crate::opendal_backend::OpendalBackend;
use crate::trait_def::StorageBackend;

/// S3/MinIO 存储后端
///
/// This is a thin wrapper around [`OpendalBackend`] that configures the operator
/// for the S3 protocol. All `StorageBackend` methods are delegated to the
/// inner generic implementation.
pub struct S3Backend {
    inner: OpendalBackend,
    endpoint: String,
    bucket: String,
}

impl S3Backend {
    /// 创建新的 S3 存储后端
    pub fn new(
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
            .map_err(|e| StorageError::Config(format!("Failed to create S3 operator: {}", e)))?
            .finish();

        Ok(Self {
            inner: OpendalBackend::new(op, "S3"),
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
        })
    }

    /// 创建带有自定义区域和根目录的 S3 后端
    pub fn with_config(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: &str,
        root: &str,
    ) -> StorageResult<Self> {
        let builder = S3::default()
            .endpoint(endpoint)
            .bucket(bucket)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .region(region)
            .root(root);

        let op = Operator::new(builder)
            .map_err(|e| StorageError::Config(format!("Failed to create S3 operator: {}", e)))?
            .finish();

        Ok(Self {
            inner: OpendalBackend::new(op, "S3"),
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
impl StorageBackend for S3Backend {
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
    fn test_s3_backend_creation() {
        let result = S3Backend::new(
            "http://localhost:9000",
            "test-bucket",
            "minioadmin",
            "minio12345",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_s3_backend_endpoint() {
        let backend = S3Backend::new(
            "http://localhost:9000",
            "test-bucket",
            "minioadmin",
            "minio12345",
        )
        .unwrap();
        assert_eq!(backend.endpoint(), "http://localhost:9000");
        assert_eq!(backend.bucket(), "test-bucket");
    }
}
