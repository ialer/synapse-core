//! 本地文件系统 Provider

use async_trait::async_trait;
use crate::provider::*;

/// 本地文件系统 Provider
pub struct LocalProvider {
    root: std::path::PathBuf,
    connected: std::sync::atomic::AtomicBool,
}

impl LocalProvider {
    /// 创建新的本地文件系统 Provider
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            root: root.into(),
            connected: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// 将相对路径转换为绝对路径
    fn full_path(&self, key: &str) -> std::path::PathBuf {
        self.root.join(key)
    }
}

#[async_trait]
impl Provider for LocalProvider {
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "local".to_string(),
            display_name: "本地文件系统".to_string(),
            description: "本地磁盘文件系统".to_string(),
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
        // 验证目录存在
        if !self.root.exists() {
            return Err(ProviderError::ConnectionFailed(
                format!("目录不存在: {}", self.root.display())
            ));
        }
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

    async fn list(&self, prefix: &str) -> ProviderResult<Vec<String>> {
        let dir = self.root.join(prefix);
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();
        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| ProviderError::Internal(e.to_string()))?;

        while let Some(entry) = entries.next_entry()
            .await
            .map_err(|e| ProviderError::Internal(e.to_string()))? 
        {
            let path = entry.path();
            let relative = path.strip_prefix(&self.root)
                .unwrap_or(&path);
            results.push(relative.to_string_lossy().to_string());
        }

        Ok(results)
    }

    async fn read(&self, key: &str) -> ProviderResult<Vec<u8>> {
        let path = self.full_path(key);
        tokio::fs::read(&path)
            .await
            .map_err(|e| ProviderError::DataError(format!("读取失败 {}: {}", key, e)))
    }

    async fn write(&self, key: &str, data: &[u8]) -> ProviderResult<()> {
        let path = self.full_path(key);
        
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| ProviderError::Internal(e.to_string()))?;
        }

        tokio::fs::write(&path, data)
            .await
            .map_err(|e| ProviderError::DataError(format!("写入失败 {}: {}", key, e)))
    }

    async fn delete(&self, key: &str) -> ProviderResult<()> {
        let path = self.full_path(key);
        if path.exists() {
            if path.is_dir() {
                tokio::fs::remove_dir_all(&path)
                    .await
                    .map_err(|e| ProviderError::DataError(format!("删除失败 {}: {}", key, e)))?;
            } else {
                tokio::fs::remove_file(&path)
                    .await
                    .map_err(|e| ProviderError::DataError(format!("删除失败 {}: {}", key, e)))?;
            }
        }
        Ok(())
    }

    async fn exists(&self, key: &str) -> ProviderResult<bool> {
        let path = self.full_path(key);
        Ok(path.exists())
    }

    async fn size(&self, key: &str) -> ProviderResult<u64> {
        let path = self.full_path(key);
        let metadata = tokio::fs::metadata(&path)
            .await
            .map_err(|e| ProviderError::DataError(format!("获取大小失败 {}: {}", key, e)))?;
        Ok(metadata.len())
    }
}
