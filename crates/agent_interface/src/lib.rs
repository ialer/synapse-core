//! # Agent Interface - Agent 接口模块
//!
//! 提供 MCP 协议和 CLI 接口，支持 AI Agent 访问个人资料库。

pub mod agent_service;
pub mod cli;
pub mod mcp;
pub mod tools;

use async_trait::async_trait;

pub use agent_service::{AgentAccess, AgentError, AgentService, AgentSession, MemoryAgentService};
pub use cli::CliInterface;
pub use mcp::McpServer;
pub use tools::{Tool, ToolRegistry};

/// 搜索结果项（DataProvider 返回类型）
#[derive(Debug, Clone)]
pub struct SearchEntry {
    /// 数据 ID
    pub id: String,
    /// 内容摘要
    pub content: String,
    /// 元数据（如 type, tags）
    pub metadata: std::collections::HashMap<String, String>,
}

/// 数据提供者 trait — 由 SynapseApp 实现
///
/// 定义 agent_interface 所需的数据操作接口，
/// 避免 agent_interface 直接依赖 synapse_service。
#[async_trait]
pub trait DataProvider: Send + Sync {
    /// 搜索数据（全文检索）
    ///
    /// # 参数
    /// * `query` - 搜索关键词
    /// * `limit` - 最大返回条数
    ///
    /// # 返回
    /// 匹配的索引条目列表
    async fn search_data(&self, query: &str, limit: usize) -> Vec<SearchEntry>;

    /// 获取并解密数据
    ///
    /// # 参数
    /// * `token` - 用户认证 token
    /// * `id` - 数据 ID
    ///
    /// # 返回
    /// (id, data_type_str, tags, decrypted_content_bytes)
    async fn get_data(
        &self,
        token: &str,
        id: &str,
    ) -> Result<(String, String, Vec<String>, Vec<u8>), String>;

    /// 存储加密数据
    ///
    /// # 参数
    /// * `token` - 用户认证 token
    /// * `data_type` - 数据类型字符串（如 "credential", "config" 等）
    /// * `content` - 原始明文内容
    /// * `tags` - 标签列表
    ///
    /// # 返回
    /// 新数据的 ID
    async fn store_data(
        &self,
        token: &str,
        data_type: &str,
        content: Vec<u8>,
        tags: Vec<String>,
    ) -> Result<String, String>;

    /// 更新已加密数据的内容
    async fn update_data(
        &self,
        token: &str,
        id: &str,
        content: Vec<u8>,
        tags: Vec<String>,
    ) -> Result<(), String>;

    /// 删除数据
    async fn delete_data(&self, token: &str, id: &str) -> Result<(), String>;

    /// 列出所有数据基本信息（不含加密内容）
    async fn list_all_data(&self) -> Vec<ListEntry>;
}

/// 列表条目（list_all_data 返回类型）
#[derive(Debug, Clone)]
pub struct ListEntry {
    pub id: String,
    pub data_type: String,
    pub tags: Vec<String>,
    pub created_at: String,
}

/// 空实现：用于测试和无数据后端场景
pub struct NullDataProvider;

#[async_trait]
impl DataProvider for NullDataProvider {
    async fn search_data(&self, _query: &str, _limit: usize) -> Vec<SearchEntry> {
        Vec::new()
    }

    async fn get_data(
        &self,
        _token: &str,
        id: &str,
    ) -> Result<(String, String, Vec<String>, Vec<u8>), String> {
        Err(format!("Data not found: {}", id))
    }

    async fn store_data(
        &self,
        _token: &str,
        _data_type: &str,
        _content: Vec<u8>,
        _tags: Vec<String>,
    ) -> Result<String, String> {
        Err("No storage backend available".to_string())
    }

    async fn update_data(
        &self,
        _token: &str,
        _id: &str,
        _content: Vec<u8>,
        _tags: Vec<String>,
    ) -> Result<(), String> {
        Err("No storage backend available".to_string())
    }

    async fn delete_data(&self, _token: &str, _id: &str) -> Result<(), String> {
        Err("No storage backend available".to_string())
    }

    async fn list_all_data(&self) -> Vec<ListEntry> {
        Vec::new()
    }
}
