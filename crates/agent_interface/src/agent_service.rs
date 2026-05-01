//! Agent 服务模块
//! 
//! 提供 MCP 协议的 Agent 交互功能。

use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mcp::{McpRequest, McpResponse, McpResult, McpError};
use crate::tools::{ToolRegistry, ToolResult};
use crate::mcp::ToolInfo;

/// Agent 访问权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAccess {
    /// 允许访问的标签
    pub allowed_tags: Vec<String>,
    
    /// 允许的数据类型
    pub allowed_types: Vec<String>,
    
    /// 最大读取数量
    pub max_read: usize,
    
    /// 最大写入数量
    pub max_write: usize,
}

impl Default for AgentAccess {
    fn default() -> Self {
        Self {
            allowed_tags: vec!["agent".to_string()],
            allowed_types: vec!["generic".to_string()],
            max_read: 100,
            max_write: 10,
        }
    }
}

/// Agent 会话
#[derive(Debug, Clone)]
pub struct AgentSession {
    /// 会话 ID
    pub session_id: String,
    
    /// Agent ID
    pub agent_id: String,
    
    /// 访问权限
    pub access: AgentAccess,
    
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// 最后活动时间
    pub last_active: chrono::DateTime<chrono::Utc>,
}

/// Agent 服务 Trait
#[async_trait]
pub trait AgentService: Send + Sync {
    /// 创建会话
    async fn create_session(&self, agent_id: &str, access: AgentAccess) -> AgentSession;
    
    /// 获取会话
    async fn get_session(&self, session_id: &str) -> Option<AgentSession>;
    
    /// 处理 MCP 请求
    async fn handle_mcp_request(&self, session_id: &str, request: McpRequest) -> McpResponse;
    
    /// 搜索数据（带权限检查）
    async fn search_with_access(
        &self,
        session_id: &str,
        query: &str,
        data_type: Option<&str>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, AgentError>;
    
    /// 获取数据（带权限检查）
    async fn get_with_access(
        &self,
        session_id: &str,
        data_id: &str,
    ) -> Result<DataItem, AgentError>;
}

/// 搜索结果项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    /// 数据 ID
    pub id: String,
    
    /// 数据类型
    pub data_type: String,
    
    /// 标签
    pub tags: Vec<String>,
    
    /// 相关度分数
    pub score: f64,
    
    /// 摘要
    pub summary: String,
}

/// 数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    /// 数据 ID
    pub id: String,
    
    /// 数据类型
    pub data_type: String,
    
    /// 标签
    pub tags: Vec<String>,
    
    /// 内容摘要
    pub summary: String,
    
    /// 创建时间
    pub created_at: String,
}

/// Agent 错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentError {
    /// 会话不存在
    SessionNotFound,
    
    /// 权限不足
    PermissionDenied(String),
    
    /// 数据不存在
    DataNotFound(String),
    
    /// 会话已过期
    SessionExpired,
    
    /// 内部错误
    Internal(String),
}

/// 内存 Agent 服务实现
pub struct MemoryAgentService {
    /// 会话存储
    sessions: HashMap<String, AgentSession>,
    
    /// 工具注册表
    tool_registry: ToolRegistry,
}

impl MemoryAgentService {
    /// 创建新的 Agent 服务
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            tool_registry: ToolRegistry::new(),
        }
    }
    
    /// 检查标签访问权限
    fn check_tag_access(&self, access: &AgentAccess, tags: &[String]) -> bool {
        tags.iter().any(|tag| access.allowed_tags.contains(tag))
    }
    
    /// 检查类型访问权限
    fn check_type_access(&self, access: &AgentAccess, data_type: &str) -> bool {
        access.allowed_types.is_empty() || access.allowed_types.contains(&data_type.to_string())
    }
}

#[async_trait]
impl AgentService for MemoryAgentService {
    async fn create_session(&self, agent_id: &str, access: AgentAccess) -> AgentSession {
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        AgentSession {
            session_id,
            agent_id: agent_id.to_string(),
            access,
            created_at: now,
            last_active: now,
        }
    }
    
    async fn get_session(&self, session_id: &str) -> Option<AgentSession> {
        self.sessions.get(session_id).cloned()
    }
    
    async fn handle_mcp_request(&self, session_id: &str, request: McpRequest) -> McpResponse {
        // 简化版本：处理基本请求
        match request {
            McpRequest::Initialize(_) => McpResponse {
                id: "1".to_string(),
                result: Some(McpResult::Initialize(crate::mcp::InitializeResult {
                    server_info: crate::mcp::ServerInfo {
                        name: "synapse-agent-service".to_string(),
                        version: "0.1.0".to_string(),
                    },
                    capabilities: vec!["tools".to_string(), "search".to_string()],
                })),
                error: None,
            },
            McpRequest::ListTools => McpResponse {
                id: "1".to_string(),
                result: Some(McpResult::ToolList(vec![
                    ToolInfo {
                        name: "search_data".to_string(),
                        description: "Search personal data".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "query": {"type": "string"}
                            }
                        }),
                    },
                ])),
                error: None,
            },
            _ => McpResponse {
                id: "1".to_string(),
                result: None,
                error: Some(McpError {
                    code: -1,
                    message: "Not implemented".to_string(),
                }),
            },
        }
    }
    
    async fn search_with_access(
        &self,
        session_id: &str,
        query: &str,
        data_type: Option<&str>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, AgentError> {
        // 获取会话
        let session = self.get_session(session_id)
            .await
            .ok_or(AgentError::SessionNotFound)?;
        
        // 检查类型权限
        if let Some(dt) = data_type {
            if !self.check_type_access(&session.access, dt) {
                return Err(AgentError::PermissionDenied(
                    format!("Access denied for type: {}", dt)
                ));
            }
        }
        
        // 检查标签权限
        if let Some(t) = tags {
            if !self.check_tag_access(&session.access, t) {
                return Err(AgentError::PermissionDenied(
                    "Access denied for tags".to_string()
                ));
            }
        }
        
        // 限制结果数量
        let limited_limit = limit.min(session.access.max_read);
        
        // 返回模拟结果
        Ok(vec![
            SearchResultItem {
                id: "example-1".to_string(),
                data_type: "credential".to_string(),
                tags: vec!["github".to_string()],
                score: 0.95,
                summary: "GitHub Token for API access".to_string(),
            },
        ])
    }
    
    async fn get_with_access(
        &self,
        session_id: &str,
        data_id: &str,
    ) -> Result<DataItem, AgentError> {
        // 获取会话
        let _session = self.get_session(session_id)
            .await
            .ok_or(AgentError::SessionNotFound)?;
        
        // 返回模拟数据
        Ok(DataItem {
            id: data_id.to_string(),
            data_type: "credential".to_string(),
            tags: vec!["github".to_string()],
            summary: "GitHub Token".to_string(),
            created_at: "2026-05-01T00:00:00Z".to_string(),
        })
    }
}

impl Default for MemoryAgentService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session() {
        let service = MemoryAgentService::new();
        let access = AgentAccess::default();
        
        let session = service.create_session("test-agent", access).await;
        assert_eq!(session.agent_id, "test-agent");
        assert!(!session.session_id.is_empty());
    }

    #[tokio::test]
    async fn test_get_session() {
        let mut service = MemoryAgentService::new();
        let access = AgentAccess::default();
        
        let session = service.create_session("test-agent", access).await;
        let session_id = session.session_id.clone();
        
        // 手动添加会话
        service.sessions.insert(session_id.clone(), session);
        
        let retrieved = service.get_session(&session_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().agent_id, "test-agent");
    }

    #[tokio::test]
    async fn test_search_with_access() {
        let mut service = MemoryAgentService::new();
        let access = AgentAccess {
            allowed_tags: vec!["github".to_string()],
            allowed_types: vec!["credential".to_string()],
            max_read: 10,
            max_write: 5,
        };
        
        let session = service.create_session("test-agent", access).await;
        let session_id = session.session_id.clone();
        service.sessions.insert(session_id.clone(), session);
        
        let results = service.search_with_access(
            &session_id,
            "github",
            Some("credential"),
            None,
            10,
        ).await.unwrap();
        
        assert_eq!(results.len(), 1);
    }
}
