//! 本地存储后端实现
//! 
//! 基于文件系统的本地存储实现。

use async_trait::async_trait;
use std::path::PathBuf;

use crate::error::{StorageError, StorageResult};
use crate::trait_def::{StorageBackend, StorageMetadata};

/// 本地存储后端
pub struct LocalBackend {
    /// 根目录
    root: PathBuf,
}

impl LocalBackend {
    /// 创建新的本地存储后端
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
        }
    }
    
    /// 获取完整路径
    fn full_path(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }
    
    /// 确保目录存在
    async fn ensure_dir(&self, path: &PathBuf) -> StorageResult<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl StorageBackend for LocalBackend {
    async fn save(&self, key: &str, data: &[u8]) -> StorageResult<()> {
        let path = self.full_path(key);
        self.ensure_dir(&path).await?;
        tokio::fs::write(&path, data).await?;
        Ok(())
    }
    
    async fn load(&self, key: &str) -> StorageResult<Vec<u8>> {
        let path = self.full_path(key);
        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }
        Ok(tokio::fs::read(&path).await?)
    }
    
    async fn list(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let mut results = Vec::new();
        let search_dir = if prefix.is_empty() {
            self.root.clone()
        } else {
            self.root.join(prefix)
        };
        
        if search_dir.exists() && search_dir.is_dir() {
            self.list_recursive(&search_dir, &mut results).await?;
        }
        
        Ok(results)
    }
    
    async fn delete(&self, key: &str) -> StorageResult<()> {
        let path = self.full_path(key);
        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }
        tokio::fs::remove_file(&path).await?;
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> StorageResult<bool> {
        let path = self.full_path(key);
        Ok(path.exists())
    }
    
    async fn size(&self, key: &str) -> StorageResult<u64> {
        let path = self.full_path(key);
        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }
        let metadata = tokio::fs::metadata(&path).await?;
        Ok(metadata.len())
    }
    
    async fn copy(&self, src: &str, dst: &str) -> StorageResult<()> {
        let src_path = self.full_path(src);
        let dst_path = self.full_path(dst);
        
        if !src_path.exists() {
            return Err(StorageError::NotFound(src.to_string()));
        }
        
        self.ensure_dir(&dst_path).await?;
        tokio::fs::copy(&src_path, &dst_path).await?;
        Ok(())
    }
    
    async fn rename(&self, src: &str, dst: &str) -> StorageResult<()> {
        let src_path = self.full_path(src);
        let dst_path = self.full_path(dst);
        
        if !src_path.exists() {
            return Err(StorageError::NotFound(src.to_string()));
        }
        
        self.ensure_dir(&dst_path).await?;
        tokio::fs::rename(&src_path, &dst_path).await?;
        Ok(())
    }
}

impl LocalBackend {
    /// 递归列出目录下的所有文件
    async fn list_recursive(&self, dir: &PathBuf, results: &mut Vec<String>) -> StorageResult<()> {
        let mut entries = tokio::fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                Box::pin(self.list_recursive(&path, results)).await?;
            } else {
                // 返回相对于 root 的路径
                if let Ok(relative) = path.strip_prefix(&self.root) {
                    results.push(relative.to_string_lossy().to_string());
                }
            }
        }
        Ok(())
    }
    
    /// 获取文件元数据
    pub async fn metadata(&self, key: &str) -> StorageResult<StorageMetadata> {
        let path = self.full_path(key);
        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }
        
        let metadata = tokio::fs::metadata(&path).await?;
        let created = metadata.created()
            .map(|t| chrono::DateTime::from(t))
            .unwrap_or_else(|_| chrono::Utc::now());
        let modified = metadata.modified()
            .map(|t| chrono::DateTime::from(t))
            .unwrap_or_else(|_| chrono::Utc::now());
        
        Ok(StorageMetadata {
            key: key.to_string(),
            size: metadata.len(),
            created_at: created,
            modified_at: modified,
            content_type: None,
            checksum: None,
        })
    }
    
    /// 清理空目录
    pub async fn cleanup_empty_dirs(&self) -> StorageResult<u32> {
        // 简化版本：只清理根目录下的空目录
        let mut count = 0;
        if self.root.exists() {
            let mut entries = tokio::fs::read_dir(&self.root).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.is_dir() {
                    let mut sub_entries = tokio::fs::read_dir(&path).await?;
                    if sub_entries.next_entry().await?.is_none() {
                        tokio::fs::remove_dir(&path).await?;
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_local_backend() {
        let temp_dir = env::temp_dir().join("synapse-test");
        let backend = LocalBackend::new(&temp_dir);
        
        // 保存数据
        let data = b"Hello, World!";
        backend.save("test.txt", data).await.unwrap();
        
        // 加载数据
        let loaded = backend.load("test.txt").await.unwrap();
        assert_eq!(data.to_vec(), loaded);
        
        // 检查存在
        assert!(backend.exists("test.txt").await.unwrap());
        assert!(!backend.exists("nonexistent.txt").await.unwrap());
        
        // 获取大小
        let size = backend.size("test.txt").await.unwrap();
        assert_eq!(size, data.len() as u64);
        
        // 复制数据
        backend.copy("test.txt", "test2.txt").await.unwrap();
        assert!(backend.exists("test2.txt").await.unwrap());
        
        // 移动数据
        backend.rename("test2.txt", "test3.txt").await.unwrap();
        assert!(!backend.exists("test2.txt").await.unwrap());
        assert!(backend.exists("test3.txt").await.unwrap());
        
        // 删除数据
        backend.delete("test.txt").await.unwrap();
        assert!(!backend.exists("test.txt").await.unwrap());
        
        // 清理
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
    
    #[tokio::test]
    async fn test_local_backend_list_recursive() {
        let temp_dir = env::temp_dir().join("synapse-test-list");
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
        let backend = LocalBackend::new(&temp_dir);
        
        // 创建嵌套目录结构
        backend.save("file1.txt", b"data1").await.unwrap();
        backend.save("dir1/file2.txt", b"data2").await.unwrap();
        backend.save("dir1/sub/file3.txt", b"data3").await.unwrap();
        backend.save("dir2/file4.txt", b"data4").await.unwrap();
        
        // 列出所有文件
        let all = backend.list("").await.unwrap();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&"file1.txt".to_string()));
        assert!(all.contains(&"dir1/file2.txt".to_string()));
        assert!(all.contains(&"dir1/sub/file3.txt".to_string()));
        assert!(all.contains(&"dir2/file4.txt".to_string()));
        
        // 按前缀过滤
        let dir1_files = backend.list("dir1/").await.unwrap();
        assert_eq!(dir1_files.len(), 2);
        assert!(dir1_files.contains(&"dir1/file2.txt".to_string()));
        assert!(dir1_files.contains(&"dir1/sub/file3.txt".to_string()));
        
        // 空前缀返回空
        let empty = backend.list("nonexistent/").await.unwrap();
        assert!(empty.is_empty());
        
        // 清理
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
