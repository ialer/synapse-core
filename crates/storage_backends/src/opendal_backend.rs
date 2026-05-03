//! Generic opendal-based storage backend
//!
//! Provides a single, unified `StorageBackend` implementation backed by `opendal::Operator`.
//! All protocol-specific backends (S3, R2, OSS, WebDAV) delegate their trait methods here.

use async_trait::async_trait;
use opendal::Operator;

use crate::error::{StorageError, StorageResult};
use crate::trait_def::StorageBackend;

/// A generic storage backend that wraps an `opendal::Operator`.
///
/// This eliminates code duplication across S3, R2, OSS, and WebDAV backends
/// by centralising the `StorageBackend` trait implementation.
pub struct OpendalBackend {
    op: Operator,
    name: String,
    /// When `true`, `rename` is implemented as `copy` + `delete` instead of
    /// using the native rename operation (needed for backends like R2 that
    /// don't support native rename).
    copy_delete_rename: bool,
}

impl OpendalBackend {
    /// Create a new `OpendalBackend` with native rename support.
    ///
    /// # Arguments
    /// * `op` – a fully configured `opendal::Operator`
    /// * `name` – human-readable name used in error messages (e.g. "S3", "R2")
    pub fn new(op: Operator, name: impl Into<String>) -> Self {
        Self {
            op,
            name: name.into(),
            copy_delete_rename: false,
        }
    }

    /// Create a new `OpendalBackend` with copy+delete rename semantics.
    ///
    /// Use this for backends that do not support native rename (e.g. R2).
    pub fn with_copy_delete_rename(op: Operator, name: impl Into<String>) -> Self {
        Self {
            op,
            name: name.into(),
            copy_delete_rename: true,
        }
    }

    /// Get a reference to the underlying `opendal::Operator`.
    pub fn operator(&self) -> &Operator {
        &self.op
    }
}

#[async_trait]
impl StorageBackend for OpendalBackend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        let _ = self
            .op
            .write(key, data.to_vec())
            .await
            .map_err(|e| StorageError::Network(format!("{} write failed: {}", self.name, e)))?;
        Ok(())
    }

    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        self.op
            .read(key)
            .await
            .map(|data| data.to_vec())
            .map_err(|e| StorageError::Network(format!("{} read failed: {}", self.name, e)))
    }

    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let entries = self
            .op
            .list(prefix)
            .await
            .map_err(|e| StorageError::Network(format!("{} list failed: {}", self.name, e)))?;

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
            .map_err(|e| StorageError::Network(format!("{} delete failed: {}", self.name, e)))
    }

    async fn exists(&self, key: &str) -> StorageResult<bool> {
        match self.op.stat(key).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("not found") {
                    Ok(false)
                } else {
                    Err(StorageError::Network(format!(
                        "{} exists check failed: {}",
                        self.name,
                        e
                    )))
                }
            }
        }
    }

    async fn size(&self, key: &str) -> StorageResult<u64> {
        let meta = self
            .op
            .stat(key)
            .await
            .map_err(|e| StorageError::Network(format!("{} stat failed: {}", self.name, e)))?;

        Ok(meta.content_length())
    }

    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        self.op
            .copy(src, dst)
            .await
            .map_err(|e| StorageError::Network(format!("{} copy failed: {}", self.name, e)))
    }

    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        if self.copy_delete_rename {
            self.copy(src, dst).await?;
            self.delete(src).await?;
            Ok(())
        } else {
            self.op
                .rename(src, dst)
                .await
                .map_err(|e| StorageError::Network(format!("{} rename failed: {}", self.name, e)))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_opendal_backend_creation() {
        // We can't easily create an Operator without a real backend, but we can
        // verify the struct compiles. The real value comes from integration tests.
    }
}
