//! # Sync Engine - 同步引擎模块
//!
//! 提供多设备数据同步与冲突解决功能。

pub mod conflict;
pub mod error;
pub mod sync;

pub use conflict::{ConflictDetector, ConflictResolution, ConflictResolver, ConflictStrategy};
pub use error::{SyncError, SyncResult};
pub use sync::{SyncEngine, SyncOperation, SyncRecord, SyncState, SyncStatus};
