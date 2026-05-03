//! 数据源连接器模块
//! 
//! 定义 Provider 插件接口，支持多种数据源接入。

pub mod provider;
pub mod registry;
pub mod local;
pub mod webdav;

pub use provider::{Provider, ProviderInfo, ProviderError, ProviderResult};
pub use registry::ProviderRegistry;
pub use local::LocalProvider;
