#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use synapse_connect::{Provider, ProviderRegistry, ProviderInfo, ProviderResult};
    use async_trait::async_trait;

    // Mock Provider for testing
    struct MockProvider {
        name: String,
    }

    #[async_trait]
    impl Provider for MockProvider {
        fn info(&self) -> ProviderInfo {
            ProviderInfo {
                name: self.name.clone(),
                display_name: format!("Mock {}", self.name),
                description: "测试用 Mock Provider".to_string(),
                version: "0.1.0".to_string(),
                capabilities: vec!["read".to_string()],
            }
        }

        async fn connect(&self) -> ProviderResult<()> { Ok(()) }
        async fn disconnect(&self) -> ProviderResult<()> { Ok(()) }
        async fn is_connected(&self) -> bool { true }
        async fn list(&self, _prefix: &str) -> ProviderResult<Vec<String>> { Ok(vec![]) }
        async fn read(&self, _key: &str) -> ProviderResult<Vec<u8>> { Ok(b"mock".to_vec()) }
        async fn write(&self, _key: &str, _data: &[u8]) -> ProviderResult<()> { Ok(()) }
        async fn delete(&self, _key: &str) -> ProviderResult<()> { Ok(()) }
        async fn exists(&self, _key: &str) -> ProviderResult<bool> { Ok(false) }
        async fn size(&self, _key: &str) -> ProviderResult<u64> { Ok(0) }
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = ProviderRegistry::new();
        let provider = Arc::new(MockProvider { name: "test".to_string() });
        
        registry.register("test".to_string(), provider.clone());
        
        let retrieved = registry.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().info().name, "test");
    }

    #[test]
    fn test_registry_list_providers() {
        let mut registry = ProviderRegistry::new();
        registry.register("a".to_string(), Arc::new(MockProvider { name: "a".to_string() }));
        registry.register("b".to_string(), Arc::new(MockProvider { name: "b".to_string() }));
        
        let providers = registry.list_providers();
        assert_eq!(providers.len(), 2);
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = ProviderRegistry::new();
        registry.register("test".to_string(), Arc::new(MockProvider { name: "test".to_string() }));
        
        let removed = registry.remove("test");
        assert!(removed.is_some());
        assert!(registry.get("test").is_none());
    }

    #[test]
    fn test_registry_provider_names() {
        let mut registry = ProviderRegistry::new();
        registry.register("x".to_string(), Arc::new(MockProvider { name: "x".to_string() }));
        registry.register("y".to_string(), Arc::new(MockProvider { name: "y".to_string() }));
        
        let names = registry.provider_names();
        assert!(names.contains(&"x".to_string()));
        assert!(names.contains(&"y".to_string()));
    }

    #[test]
    fn test_provider_info() {
        let provider = MockProvider { name: "test".to_string() };
        let info = provider.info();
        assert_eq!(info.name, "test");
        assert_eq!(info.version, "0.1.0");
    }
}

#[cfg(test)]
mod local_tests {
    use tempfile::tempdir;
    use synapse_connect::{Provider, LocalProvider};

    #[tokio::test]
    async fn test_local_provider_connect() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        
        assert!(!provider.is_connected().await);
        provider.connect().await.unwrap();
        assert!(provider.is_connected().await);
    }

    #[tokio::test]
    async fn test_local_provider_write_and_read() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        provider.connect().await.unwrap();
        
        // 写入数据
        provider.write("test.txt", b"hello world").await.unwrap();
        
        // 读取数据
        let data = provider.read("test.txt").await.unwrap();
        assert_eq!(data, b"hello world");
    }

    #[tokio::test]
    async fn test_local_provider_exists_and_size() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        provider.connect().await.unwrap();
        
        assert!(!provider.exists("test.txt").await.unwrap());
        
        provider.write("test.txt", b"hello").await.unwrap();
        
        assert!(provider.exists("test.txt").await.unwrap());
        assert_eq!(provider.size("test.txt").await.unwrap(), 5);
    }

    #[tokio::test]
    async fn test_local_provider_list() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        provider.connect().await.unwrap();
        
        provider.write("a.txt", b"a").await.unwrap();
        provider.write("b.txt", b"b").await.unwrap();
        
        let files = provider.list("").await.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_local_provider_delete() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        provider.connect().await.unwrap();
        
        provider.write("test.txt", b"data").await.unwrap();
        assert!(provider.exists("test.txt").await.unwrap());
        
        provider.delete("test.txt").await.unwrap();
        assert!(!provider.exists("test.txt").await.unwrap());
    }

    #[tokio::test]
    async fn test_local_provider_subdirectories() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        provider.connect().await.unwrap();
        
        provider.write("sub/nested/file.txt", b"data").await.unwrap();
        let data = provider.read("sub/nested/file.txt").await.unwrap();
        assert_eq!(data, b"data");
    }

    #[test]
    fn test_local_provider_info() {
        let dir = tempdir().unwrap();
        let provider = LocalProvider::new(dir.path());
        let info = provider.info();
        
        assert_eq!(info.name, "local");
        assert!(info.capabilities.contains(&"read".to_string()));
        assert!(info.capabilities.contains(&"write".to_string()));
    }
}
