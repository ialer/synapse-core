//! 流式数据处理管道
//!
//! Pipeline pattern: Source → Transform → Encrypt → Store → Index
//!
//! Uses `tokio::sync::mpsc` for event notification and supports
//! concurrent batch processing with configurable stages.

use tokio::sync::mpsc;
use tokio::time::Instant;
use std::sync::Arc;

use crate::error::{SynapseError, SynapseResult};

// ---------------------------------------------------------------------------
// Pipeline events
// ---------------------------------------------------------------------------

/// 管道事件
#[derive(Debug, Clone)]
pub enum PipelineEvent {
    /// 数据到达
    DataReceived { key: String, size: usize },
    /// 转换完成
    Transformed { key: String, original_size: usize, transformed_size: usize },
    /// 加密完成
    Encrypted { key: String, original_size: usize, encrypted_size: usize },
    /// 存储完成
    Stored { key: String },
    /// 索引更新
    Indexed { key: String },
    /// 处理完成
    Completed { key: String, duration_ms: u64 },
    /// 处理失败
    Failed { key: String, error: String },
}

// ---------------------------------------------------------------------------
// Pipeline configuration
// ---------------------------------------------------------------------------

/// 数据管道配置
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// 是否启用加密
    pub encrypt: bool,
    /// 是否更新索引
    pub index: bool,
    /// 并发处理数
    pub concurrency: usize,
    /// 是否发送事件通知
    pub notify: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            encrypt: true,
            index: true,
            concurrency: 4,
            notify: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Pipeline stages (function types)
// ---------------------------------------------------------------------------

/// Transform function: raw data → transformed data
pub type TransformFn = Arc<dyn Fn(&[u8]) -> SynapseResult<Vec<u8>> + Send + Sync>;

/// Encrypt function: data → encrypted data
pub type EncryptFn = Arc<dyn Fn(&[u8]) -> SynapseResult<Vec<u8>> + Send + Sync>;

/// Store function: (key, data) → ()
pub type StoreFn = Arc<dyn Fn(&str, &[u8]) -> SynapseResult<()> + Send + Sync>;

/// Index function: (key, data) → ()
pub type IndexFn = Arc<dyn Fn(&str, &[u8]) -> SynapseResult<()> + Send + Sync>;

// ---------------------------------------------------------------------------
// DataPipeline
// ---------------------------------------------------------------------------

/// 数据处理管道
pub struct DataPipeline {
    config: PipelineConfig,
    event_tx: Option<mpsc::UnboundedSender<PipelineEvent>>,
}

impl DataPipeline {
    /// 创建新的数据管道
    pub fn new(config: PipelineConfig) -> Self {
        Self {
            config,
            event_tx: None,
        }
    }

    /// 创建事件接收器 — 调用后所有后续事件都会发送到返回的 channel
    pub fn subscribe(&mut self) -> mpsc::UnboundedReceiver<PipelineEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_tx = Some(tx);
        rx
    }

    /// 处理单条数据
    pub async fn process<F>(&self, key: String, data: Vec<u8>, handler: F) -> SynapseResult<()>
    where
        F: Fn(&str, &[u8]) -> SynapseResult<Vec<u8>> + Send + Sync,
    {
        let start = Instant::now();
        let original_size = data.len();

        self.emit(PipelineEvent::DataReceived {
            key: key.clone(),
            size: original_size,
        });

        // 1. Transform (via user-provided handler)
        let transformed = handler(&key, &data).map_err(|e| {
            self.emit(PipelineEvent::Failed {
                key: key.clone(),
                error: e.to_string(),
            });
            e
        })?;
        self.emit(PipelineEvent::Transformed {
            key: key.clone(),
            original_size,
            transformed_size: transformed.len(),
        });

        // 2. Encrypt (optional)
        let final_data = if self.config.encrypt {
            let encrypted = self.simulate_encrypt(&transformed)?;
            self.emit(PipelineEvent::Encrypted {
                key: key.clone(),
                original_size: transformed.len(),
                encrypted_size: encrypted.len(),
            });
            encrypted
        } else {
            transformed
        };

        // 3. Store
        self.simulate_store(&key, &final_data)?;
        self.emit(PipelineEvent::Stored { key: key.clone() });

        // 4. Index (optional)
        if self.config.index {
            self.simulate_index(&key, &final_data)?;
            self.emit(PipelineEvent::Indexed { key: key.clone() });
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        self.emit(PipelineEvent::Completed {
            key,
            duration_ms,
        });

        Ok(())
    }

    /// 批量处理
    pub async fn process_batch<F>(&self, items: Vec<(String, Vec<u8>)>, handler: F) -> Vec<SynapseResult<()>>
    where
        F: Fn(&str, &[u8]) -> SynapseResult<Vec<u8>> + Send + Sync + Clone + 'static,
    {
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(self.config.concurrency));
        let mut handles = Vec::with_capacity(items.len());

        for (key, data) in items {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let handler = handler.clone();
            let config = self.config.clone();
            let event_tx = self.event_tx.clone();

            let handle = tokio::spawn(async move {
                let pipeline = DataPipeline {
                    config,
                    event_tx,
                };
                let result = pipeline.process(key, data, handler).await;
                drop(permit); // release concurrency slot
                result
            });
            handles.push(handle);
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            match handle.await {
                Ok(r) => results.push(r),
                Err(e) => results.push(Err(SynapseError::Internal(format!("Task join error: {e}")))),
            }
        }
        results
    }

    /// Process with full custom stage functions
    pub async fn process_with_stages(
        &self,
        key: String,
        data: Vec<u8>,
        transform: &TransformFn,
        encrypt: &EncryptFn,
        store: &StoreFn,
        index: &IndexFn,
    ) -> SynapseResult<()> {
        let start = Instant::now();
        let original_size = data.len();

        self.emit(PipelineEvent::DataReceived {
            key: key.clone(),
            size: original_size,
        });

        // Transform
        let transformed = transform(&data).map_err(|e| {
            self.emit(PipelineEvent::Failed { key: key.clone(), error: e.to_string() });
            e
        })?;
        self.emit(PipelineEvent::Transformed {
            key: key.clone(),
            original_size,
            transformed_size: transformed.len(),
        });

        // Encrypt
        let encrypted = if self.config.encrypt {
            let enc = encrypt(&transformed).map_err(|e| {
                self.emit(PipelineEvent::Failed { key: key.clone(), error: e.to_string() });
                e
            })?;
            self.emit(PipelineEvent::Encrypted {
                key: key.clone(),
                original_size: transformed.len(),
                encrypted_size: enc.len(),
            });
            enc
        } else {
            transformed
        };

        // Store
        store(&key, &encrypted).map_err(|e| {
            self.emit(PipelineEvent::Failed { key: key.clone(), error: e.to_string() });
            e
        })?;
        self.emit(PipelineEvent::Stored { key: key.clone() });

        // Index
        if self.config.index {
            index(&key, &encrypted).map_err(|e| {
                self.emit(PipelineEvent::Failed { key: key.clone(), error: e.to_string() });
                e
            })?;
            self.emit(PipelineEvent::Indexed { key: key.clone() });
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        self.emit(PipelineEvent::Completed { key, duration_ms });

        Ok(())
    }

    /// Return a clone of the config
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }

    // ----- internal helpers (default "simulate" stages) -----

    fn simulate_encrypt(&self, data: &[u8]) -> SynapseResult<Vec<u8>> {
        // In production this would call data_core::Cipher.
        // Here we just prepend a simple marker to show encryption happened.
        let mut out = Vec::with_capacity(data.len() + 8);
        out.extend_from_slice(b"ENC:");
        out.extend_from_slice(data);
        Ok(out)
    }

    fn simulate_store(&self, _key: &str, _data: &[u8]) -> SynapseResult<()> {
        // In production this would call storage_backends::StorageBackend::save.
        Ok(())
    }

    fn simulate_index(&self, _key: &str, _data: &[u8]) -> SynapseResult<()> {
        // In production this would call search_indexer::Indexer.
        Ok(())
    }

    fn emit(&self, event: PipelineEvent) {
        if self.config.notify {
            if let Some(tx) = &self.event_tx {
                let _ = tx.send(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let config = PipelineConfig::default();
        let pipeline = DataPipeline::new(config.clone());
        assert!(pipeline.config().encrypt);
        assert!(pipeline.config().index);
        assert_eq!(pipeline.config().concurrency, 4);
        assert!(pipeline.config().notify);
    }

    #[test]
    fn test_config_defaults() {
        let cfg = PipelineConfig::default();
        assert!(cfg.encrypt);
        assert!(cfg.index);
        assert_eq!(cfg.concurrency, 4);
        assert!(cfg.notify);
    }

    #[test]
    fn test_event_subscription() {
        let config = PipelineConfig::default();
        let mut pipeline = DataPipeline::new(config);
        let mut rx = pipeline.subscribe();

        // Emit a test event
        pipeline.emit(PipelineEvent::DataReceived {
            key: "test".into(),
            size: 42,
        });

        let event = rx.try_recv().unwrap();
        match event {
            PipelineEvent::DataReceived { key, size } => {
                assert_eq!(key, "test");
                assert_eq!(size, 42);
            }
            _ => panic!("Unexpected event type"),
        }
    }

    #[tokio::test]
    async fn test_single_item_processing() {
        let config = PipelineConfig::default();
        let mut pipeline = DataPipeline::new(config);
        let mut rx = pipeline.subscribe();

        let result = pipeline
            .process("file-1".into(), b"hello".to_vec(), |_key, data| {
                Ok(data.to_vec())
            })
            .await;
        assert!(result.is_ok());

        // Collect all events
        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }

        // Should have: DataReceived, Transformed, Encrypted, Stored, Indexed, Completed
        assert_eq!(events.len(), 6);

        // Verify event sequence
        assert!(matches!(events[0], PipelineEvent::DataReceived { .. }));
        assert!(matches!(events[1], PipelineEvent::Transformed { .. }));
        assert!(matches!(events[2], PipelineEvent::Encrypted { .. }));
        assert!(matches!(events[3], PipelineEvent::Stored { .. }));
        assert!(matches!(events[4], PipelineEvent::Indexed { .. }));
        assert!(matches!(events[5], PipelineEvent::Completed { .. }));
    }

    #[tokio::test]
    async fn test_processing_without_encrypt() {
        let mut config = PipelineConfig::default();
        config.encrypt = false;
        config.index = false;
        let mut pipeline = DataPipeline::new(config);
        let mut rx = pipeline.subscribe();

        let result = pipeline
            .process("file-2".into(), b"data".to_vec(), |_key, data| {
                Ok(data.to_vec())
            })
            .await;
        assert!(result.is_ok());

        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }

        // Without encrypt/index: DataReceived, Transformed, Stored, Completed
        assert_eq!(events.len(), 4);
        assert!(matches!(events[0], PipelineEvent::DataReceived { .. }));
        assert!(matches!(events[1], PipelineEvent::Transformed { .. }));
        assert!(matches!(events[2], PipelineEvent::Stored { .. }));
        assert!(matches!(events[3], PipelineEvent::Completed { .. }));
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let config = PipelineConfig::default();
        let pipeline = DataPipeline::new(config);

        let items = vec![
            ("item-1".into(), b"aaa".to_vec()),
            ("item-2".into(), b"bbb".to_vec()),
            ("item-3".into(), b"ccc".to_vec()),
        ];

        let results = pipeline
            .process_batch(items, |_key, data| Ok(data.to_vec()))
            .await;

        assert_eq!(results.len(), 3);
        for r in results {
            assert!(r.is_ok());
        }
    }

    #[test]
    fn test_failed_event_on_handler_error() {
        let config = PipelineConfig::default();
        let mut pipeline = DataPipeline::new(config);
        let mut rx = pipeline.subscribe();

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(pipeline.process("bad".into(), vec![], |_k, _d| {
            Err(SynapseError::Internal("boom".into()))
        }));
        assert!(result.is_err());

        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }

        // DataReceived then Failed
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], PipelineEvent::DataReceived { .. }));
        assert!(matches!(&events[1], PipelineEvent::Failed { key, error }
            if key == "bad" && error.contains("boom")));
    }
}
