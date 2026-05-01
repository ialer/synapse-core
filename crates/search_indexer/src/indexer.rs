//! 索引构建器模块
//! 
//! 构建和管理搜索索引。

use std::collections::HashMap;

/// 索引条目
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// 条目 ID
    pub id: String,
    
    /// 条目内容
    pub content: String,
    
    /// 条目元数据
    pub metadata: HashMap<String, String>,
}

/// 索引构建器
pub struct Indexer {
    /// 索引数据
    data: HashMap<String, IndexEntry>,
    
    /// 索引统计
    stats: IndexStats,
}

/// 索引统计
#[derive(Debug, Clone)]
pub struct IndexStats {
    /// 总条目数
    pub total_entries: usize,
    
    /// 总内容大小
    pub total_size: usize,
}

impl Default for IndexStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            total_size: 0,
        }
    }
}

impl Indexer {
    /// 创建新的索引构建器
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            stats: IndexStats::default(),
        }
    }
    
    /// 添加条目
    pub fn add_entry(&mut self, entry: IndexEntry) {
        let size = entry.content.len();
        self.data.insert(entry.id.clone(), entry);
        self.stats.total_entries += 1;
        self.stats.total_size += size;
    }
    
    /// 删除条目
    pub fn remove_entry(&mut self, id: &str) -> bool {
        if let Some(entry) = self.data.remove(id) {
            self.stats.total_entries -= 1;
            self.stats.total_size -= entry.content.len();
            true
        } else {
            false
        }
    }
    
    /// 获取条目
    pub fn get_entry(&self, id: &str) -> Option<&IndexEntry> {
        self.data.get(id)
    }
    
    /// 搜索条目
    pub fn search(&self, query: &str, limit: usize) -> Vec<&IndexEntry> {
        let query_lower = query.to_lowercase();
        
        let mut results: Vec<&IndexEntry> = self.data
            .values()
            .filter(|entry| entry.content.to_lowercase().contains(&query_lower))
            .collect();
        
        results.sort_by(|a, b| {
            let a_score = self.calculate_score(&a.content, &query_lower);
            let b_score = self.calculate_score(&b.content, &query_lower);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        results.truncate(limit);
        results
    }
    
    /// 按元数据搜索
    pub fn search_by_metadata(&self, key: &str, value: &str, limit: usize) -> Vec<&IndexEntry> {
        let results: Vec<&IndexEntry> = self.data
            .values()
            .filter(|entry| {
                entry.metadata.get(key)
                    .map(|v| v.to_lowercase().contains(&value.to_lowercase()))
                    .unwrap_or(false)
            })
            .collect();
        
        results.into_iter().take(limit).collect()
    }
    
    /// 计算相关度分数
    fn calculate_score(&self, content: &str, query: &str) -> f64 {
        let content_lower = content.to_lowercase();
        let query_len = query.len();
        let content_len = content_lower.len();
        
        if content_len == 0 {
            return 0.0;
        }
        
        // 简单的 TF 分数
        let matches = content_lower.matches(query).count();
        let tf = matches as f64 / (content_len as f64 / query_len as f64);
        
        // 基础分数
        let base_score = if content_lower.contains(query) { 1.0 } else { 0.0 };
        
        base_score + tf
    }
    
    /// 获取统计信息
    pub fn stats(&self) -> &IndexStats {
        &self.stats
    }
    
    /// 清空索引
    pub fn clear(&mut self) {
        self.data.clear();
        self.stats = IndexStats::default();
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexer_creation() {
        let indexer = Indexer::new();
        assert_eq!(indexer.stats().total_entries, 0);
    }

    #[test]
    fn test_add_and_get_entry() {
        let mut indexer = Indexer::new();
        
        let entry = IndexEntry {
            id: "1".to_string(),
            content: "Hello, World!".to_string(),
            metadata: HashMap::new(),
        };
        
        indexer.add_entry(entry);
        assert_eq!(indexer.stats().total_entries, 1);
        
        let retrieved = indexer.get_entry("1").unwrap();
        assert_eq!(retrieved.content, "Hello, World!");
    }

    #[test]
    fn test_search() {
        let mut indexer = Indexer::new();
        
        indexer.add_entry(IndexEntry {
            id: "1".to_string(),
            content: "Hello, World!".to_string(),
            metadata: HashMap::new(),
        });
        
        indexer.add_entry(IndexEntry {
            id: "2".to_string(),
            content: "Goodbye, World!".to_string(),
            metadata: HashMap::new(),
        });
        
        let results = indexer.search("Hello", 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "1");
    }

    #[test]
    fn test_search_by_metadata() {
        let mut indexer = Indexer::new();
        
        let mut metadata1 = HashMap::new();
        metadata1.insert("type".to_string(), "credential".to_string());
        
        indexer.add_entry(IndexEntry {
            id: "1".to_string(),
            content: "GitHub Token".to_string(),
            metadata: metadata1,
        });
        
        let mut metadata2 = HashMap::new();
        metadata2.insert("type".to_string(), "config".to_string());
        
        indexer.add_entry(IndexEntry {
            id: "2".to_string(),
            content: "App Config".to_string(),
            metadata: metadata2,
        });
        
        let results = indexer.search_by_metadata("type", "credential", 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "1");
    }
}
