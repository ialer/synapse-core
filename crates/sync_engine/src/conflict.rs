//! 冲突解决模块
//! 
//! 提供多种冲突解决策略。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::sync::{SyncRecord, SyncOperation};

/// 冲突解决策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// 最后写入者胜出 (Last Writer Wins)
    LastWriterWins,
    /// 版本号更高者胜出
    HigherVersionWins,
    /// 手动解决
    Manual,
    /// 合并
    Merge,
}

/// 冲突解决方案
#[derive(Debug, Clone)]
pub struct ConflictResolution {
    /// 是否使用远程数据
    pub use_remote: bool,
    
    /// 合并后的数据（如果需要合并）
    pub merged_data: Option<Vec<u8>>,
    
    /// 解决策略
    pub strategy: ConflictStrategy,
}

/// 冲突解决器
pub struct ConflictResolver {
    /// 默认策略
    default_strategy: ConflictStrategy,
}

impl ConflictResolver {
    /// 创建新的冲突解决器
    pub fn new(strategy: ConflictStrategy) -> Self {
        Self {
            default_strategy: strategy,
        }
    }
    
    /// 解决冲突
    pub fn resolve(
        &self,
        local_record: &SyncRecord,
        remote_record: &SyncRecord,
        strategy: Option<ConflictStrategy>,
    ) -> ConflictResolution {
        let strategy = strategy.unwrap_or_else(|| self.default_strategy.clone());
        
        match strategy {
            ConflictStrategy::LastWriterWins => {
                let use_remote = remote_record.timestamp > local_record.timestamp;
                ConflictResolution {
                    use_remote,
                    merged_data: None,
                    strategy,
                }
            }
            ConflictStrategy::HigherVersionWins => {
                let use_remote = remote_record.version > local_record.version;
                ConflictResolution {
                    use_remote,
                    merged_data: None,
                    strategy,
                }
            }
            ConflictStrategy::Manual => {
                ConflictResolution {
                    use_remote: false,
                    merged_data: None,
                    strategy,
                }
            }
            ConflictStrategy::Merge => {
                // 简化版本：使用远程数据
                ConflictResolution {
                    use_remote: true,
                    merged_data: remote_record.data.clone(),
                    strategy,
                }
            }
        }
    }
    
    /// 批量解决冲突
    pub fn resolve_batch(
        &self,
        conflicts: &[(SyncRecord, SyncRecord)],
        strategy: Option<ConflictStrategy>,
    ) -> Vec<ConflictResolution> {
        conflicts
            .iter()
            .map(|(local, remote)| self.resolve(local, remote, strategy.clone()))
            .collect()
    }
}

/// 冲突检测器
pub struct ConflictDetector;

impl ConflictDetector {
    /// 检测两个记录是否冲突
    pub fn detect_conflict(record1: &SyncRecord, record2: &SyncRecord) -> bool {
        record1.data_id == record2.data_id
            && record1.device_id != record2.device_id
            && record1.version != record2.version
    }
    
    /// 检测记录列表中的冲突
    pub fn detect_conflicts(records: &[SyncRecord]) -> Vec<(&SyncRecord, &SyncRecord)> {
        let mut conflicts = Vec::new();
        
        for i in 0..records.len() {
            for j in (i + 1)..records.len() {
                if Self::detect_conflict(&records[i], &records[j]) {
                    conflicts.push((&records[i], &records[j]));
                }
            }
        }
        
        conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{SyncOperation, SyncEngine};

    #[test]
    fn test_conflict_resolver_lww() {
        let resolver = ConflictResolver::new(ConflictStrategy::LastWriterWins);
        
        let engine = SyncEngine::new("device-1");
        let local_record = engine.create_sync_record("data-1", SyncOperation::Update, 1);
        
        let engine2 = SyncEngine::new("device-2");
        let mut remote_record = engine2.create_sync_record("data-1", SyncOperation::Update, 2);
        remote_record.timestamp = Utc::now() + chrono::Duration::hours(1);
        
        let resolution = resolver.resolve(&local_record, &remote_record, None);
        assert!(resolution.use_remote);
    }

    #[test]
    fn test_conflict_resolver_version() {
        let resolver = ConflictResolver::new(ConflictStrategy::HigherVersionWins);
        
        let engine = SyncEngine::new("device-1");
        let local_record = engine.create_sync_record("data-1", SyncOperation::Update, 1);
        
        let engine2 = SyncEngine::new("device-2");
        let remote_record = engine2.create_sync_record("data-1", SyncOperation::Update, 2);
        
        let resolution = resolver.resolve(&local_record, &remote_record, None);
        assert!(resolution.use_remote);
    }

    #[test]
    fn test_conflict_detector() {
        let engine = SyncEngine::new("device-1");
        let record1 = engine.create_sync_record("data-1", SyncOperation::Update, 1);
        
        let engine2 = SyncEngine::new("device-2");
        let record2 = engine2.create_sync_record("data-1", SyncOperation::Update, 2);
        
        assert!(ConflictDetector::detect_conflict(&record1, &record2));
        
        let record3 = engine.create_sync_record("data-1", SyncOperation::Update, 1);
        assert!(!ConflictDetector::detect_conflict(&record1, &record3));
    }
}
