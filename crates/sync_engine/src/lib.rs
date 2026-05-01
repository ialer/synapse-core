//! # Sync Engine - 同步引擎模块
//! 
//! 提供多设备数据同步与冲突解决功能。

pub mod sync;
pub mod conflict;
pub mod error;

pub use sync::{SyncEngine, SyncRecord, SyncOperation, SyncState, SyncStatus};
pub use conflict::{ConflictResolver, ConflictDetector, ConflictStrategy, ConflictResolution};
pub use error::{SyncError, SyncResult};
