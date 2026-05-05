//! 数据源连接器模块
//!
//! 定义 Provider 插件接口，支持多种数据源接入。

pub mod local;
pub mod provider;
pub mod registry;
pub mod webdav;

pub use local::LocalProvider;
pub use provider::{Provider, ProviderError, ProviderInfo, ProviderResult};
pub use registry::ProviderRegistry;
