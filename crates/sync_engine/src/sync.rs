//! 同步逻辑模块
//! 
//! 实现多设备数据同步与冲突解决。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{SyncError, SyncResult};

/// 同步操作类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    /// 创建
    Create,
    /// 更新
    Update,
    /// 删除
    Delete,
}

/// 同步记录
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncRecord {
    /// 记录 ID
    pub id: String,
    
    /// 数据 ID
    pub data_id: String,
    
    /// 操作类型
    pub operation: SyncOperation,
    
    /// 操作时间
    pub timestamp: DateTime<Utc>,
    
    /// 设备 ID
    pub device_id: String,
    
    /// 版本号
    pub version: u64,
    
    /// 操作数据
    pub data: Option<Vec<u8>>,
}

impl SyncRecord {
    /// 创建新的同步记录
    pub fn new(
        data_id: impl Into<String>,
        operation: SyncOperation,
        device_id: impl Into<String>,
        version: u64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            data_id: data_id.into(),
            operation,
            timestamp: Utc::now(),
            device_id: device_id.into(),
            version,
            data: None,
        }
    }
    
    /// 设置操作数据
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }
}

/// 同步状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncState {
    /// 设备 ID
    pub device_id: String,
    
    /// 最后同步时间
    pub last_sync: DateTime<Utc>,
    
    /// 最后同步版本
    pub last_version: u64,
    
    /// 同步状态
    pub status: SyncStatus,
}

/// 同步状态枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// 空闲
    Idle,
    /// 同步中
    Syncing,
    /// 冲突
    Conflict,
    /// 错误
    Error,
}

/// 同步引擎
pub struct SyncEngine {
    /// 设备 ID
    device_id: String,
    
    /// 同步状态
    state: SyncState,
}

impl SyncEngine {
    /// 创建新的同步引擎
    pub fn new(device_id: impl Into<String>) -> Self {
        let device_id = device_id.into();
        Self {
            device_id: device_id.clone(),
            state: SyncState {
                device_id,
                last_sync: Utc::now(),
                last_version: 0,
                status: SyncStatus::Idle,
            },
        }
    }
    
    /// 检测冲突
    pub fn detect_conflict(
        &self,
        local_version: u64,
        remote_version: u64,
    ) -> Option<SyncError> {
        if local_version > 0 && remote_version > 0 && local_version != remote_version {
            Some(SyncError::VersionConflict {
                local: local_version,
                remote: remote_version,
            })
        } else {
            None
        }
    }
    
    /// 解决冲突（最后写入者胜出）
    pub fn resolve_conflict_lww(
        &self,
        local_timestamp: DateTime<Utc>,
        remote_timestamp: DateTime<Utc>,
    ) -> bool {
        remote_timestamp > local_timestamp
    }
    
    /// 解决冲突（版本号更高者胜出）
    pub fn resolve_conflict_version(
        &self,
        local_version: u64,
        remote_version: u64,
    ) -> bool {
        remote_version > local_version
    }
    
    /// 生成同步记录
    pub fn create_sync_record(
        &self,
        data_id: &str,
        operation: SyncOperation,
        version: u64,
    ) -> SyncRecord {
        SyncRecord::new(data_id, operation, &self.device_id, version)
    }
    
    /// 更新同步状态
    pub fn update_state(&mut self, version: u64) {
        self.state.last_sync = Utc::now();
        self.state.last_version = version;
        self.state.status = SyncStatus::Idle;
    }
    
    /// 获取设备 ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    
    /// 获取同步状态
    pub fn state(&self) -> &SyncState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_engine_creation() {
        let engine = SyncEngine::new("device-1");
        assert_eq!(engine.device_id(), "device-1");
        assert_eq!(engine.state().status, SyncStatus::Idle);
    }

    #[test]
    fn test_conflict_detection() {
        let engine = SyncEngine::new("device-1");
        
        // 无冲突
        assert!(engine.detect_conflict(0, 0).is_none());
        assert!(engine.detect_conflict(1, 1).is_none());
        
        // 有冲突
        assert!(engine.detect_conflict(1, 2).is_some());
        assert!(engine.detect_conflict(2, 1).is_some());
    }

    #[test]
    fn test_conflict_resolution_lww() {
        let engine = SyncEngine::new("device-1");
        
        let local_time = Utc::now();
        let remote_time = local_time + chrono::Duration::hours(1);
        
        // 远程更新，应使用远程
        assert!(engine.resolve_conflict_lww(local_time, remote_time));
        
        // 本地更新，应使用本地
        assert!(!engine.resolve_conflict_lww(remote_time, local_time));
    }

    #[test]
    fn test_conflict_resolution_version() {
        let engine = SyncEngine::new("device-1");
        
        // 远程版本更高
        assert!(engine.resolve_conflict_version(1, 2));
        
        // 本地版本更高
        assert!(!engine.resolve_conflict_version(2, 1));
    }

    #[test]
    fn test_sync_record_creation() {
        let engine = SyncEngine::new("device-1");
        let record = engine.create_sync_record("data-1", SyncOperation::Create, 1);
        
        assert_eq!(record.data_id, "data-1");
        assert_eq!(record.operation, SyncOperation::Create);
        assert_eq!(record.device_id, "device-1");
        assert_eq!(record.version, 1);
    }
}
