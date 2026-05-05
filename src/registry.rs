//! Provider 注册表

use std::collections::HashMap;
use std::sync::Arc;
use crate::provider::{Provider, ProviderInfo};

/// Provider 注册表 - 管理所有已注册的 Provider
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// 注册 Provider
    pub fn register(&mut self, name: String, provider: Arc<dyn Provider>) {
        self.providers.insert(name, provider);
    }

    /// 获取 Provider
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.get(name).cloned()
    }

    /// 列出所有已注册 Provider 的信息
    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers.values().map(|p| p.info()).collect()
    }

    /// 移除 Provider
    pub fn remove(&mut self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.remove(name)
    }

    /// 获取已注册 Provider 名称列表
    pub fn provider_names(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
