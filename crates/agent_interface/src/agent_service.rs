//! Agent 服务模块
//! 
//! 提供 MCP 协议的 Agent 交互功能。

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mcp::{McpRequest, McpResponse, McpResult, McpError};
use crate::tools::{ToolRegistry, ToolResult};
use crate::mcp::ToolInfo;
use crate::{DataProvider, SearchEntry, ListEntry};

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

    /// 数据提供者
    provider: Arc<dyn DataProvider>,
}

impl MemoryAgentService {
    /// 创建新的 Agent 服务（无数据后端，使用 NullDataProvider）
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            tool_registry: ToolRegistry::new(),
            provider: Arc::new(crate::NullDataProvider),
        }
    }

    /// 创建新的 Agent 服务（使用指定的 DataProvider）
    pub fn with_provider(provider: Arc<dyn DataProvider>) -> Self {
        Self {
            sessions: HashMap::new(),
            tool_registry: ToolRegistry::new(),
            provider,
        }
    }

    /// 获取数据提供者的引用
    pub fn provider(&self) -> &Arc<dyn DataProvider> {
        &self.provider
    }

    /// 创建会话并存储（使用 &mut self 以便写入 HashMap）
    pub fn create_and_store_session(
        &mut self,
        agent_id: &str,
        access: AgentAccess,
    ) -> AgentSession {
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        let session = AgentSession {
            session_id: session_id.clone(),
            agent_id: agent_id.to_string(),
            access,
            created_at: now,
            last_active: now,
        };
        self.sessions.insert(session_id, session.clone());
        session
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
            McpRequest::SearchData(req) => {
                match self.search_with_access(
                    session_id,
                    &req.query,
                    req.data_type.as_deref(),
                    req.tags.as_deref(),
                    req.limit.unwrap_or(10),
                ).await {
                    Ok(items) => {
                        let result_items: Vec<crate::mcp::SearchResultItem> = items.into_iter().map(|i| {
                            crate::mcp::SearchResultItem {
                                id: i.id.clone(),
                                data_type: i.data_type.clone(),
                                tags: i.tags.clone(),
                                score: i.score,
                                summary: i.summary.clone(),
                            }
                        }).collect();
                            let total = result_items.len();
                            McpResponse {
                                id: "1".to_string(),
                                result: Some(McpResult::SearchResult(crate::mcp::SearchResult {
                                    results: result_items,
                                    total,
                                })),
                                error: None,
                            }
                        }
                    Err(e) => McpResponse {
                        id: "1".to_string(),
                        result: None,
                        error: Some(McpError {
                            code: -1,
                            message: format!("{:?}", e),
                        }),
                    },
                }
            }
            McpRequest::GetData(req) => {
                match self.get_with_access(session_id, &req.id).await {
                    Ok(item) => McpResponse {
                        id: "1".to_string(),
                        result: Some(McpResult::DataDetail(crate::mcp::DataDetail {
                            id: item.id,
                            owner_id: "agent".to_string(),
                            data_type: item.data_type,
                            tags: item.tags,
                            created_at: item.created_at.clone(),
                            updated_at: item.created_at,
                            version: 1,
                        })),
                        error: None,
                    },
                    Err(e) => McpResponse {
                        id: "1".to_string(),
                        result: None,
                        error: Some(McpError {
                            code: -1,
                            message: format!("{:?}", e),
                        }),
                    },
                }
            }
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
        
        // 调用 DataProvider 进行真实搜索
        let entries = self.provider.search_data(query, limited_limit).await;
        
        let results: Vec<SearchResultItem> = entries.into_iter().map(|entry| {
            let data_type_str = entry.metadata.get("type")
                .cloned()
                .unwrap_or_else(|| "generic".to_string());
            let tags_vec: Vec<String> = entry.metadata.get("tags")
                .map(|t| t.split(',').map(String::from).collect())
                .unwrap_or_default();
            
            SearchResultItem {
                id: entry.id,
                data_type: data_type_str,
                tags: tags_vec,
                score: 1.0, // 简化分数
                summary: entry.content,
            }
        }).collect();
        
        Ok(results)
    }
    
    async fn get_with_access(
        &self,
        session_id: &str,
        data_id: &str,
    ) -> Result<DataItem, AgentError> {
        // 获取会话
        let session = self.get_session(session_id)
            .await
            .ok_or(AgentError::SessionNotFound)?;
        
        // 调用 DataProvider 获取真实数据
        // 使用 agent 的 token 进行认证（Agent 场景下使用 session_id 作为 token）
        let (id, data_type_str, tags, _content) = self.provider
            .get_data(&session.session_id, data_id)
            .await
            .map_err(|e| AgentError::DataNotFound(e))?;
        
        Ok(DataItem {
            id,
            data_type: data_type_str,
            tags,
            summary: format!("Data retrieved by agent {}", session.agent_id),
            created_at: chrono::Utc::now().to_rfc3339(),
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
    use crate::NullDataProvider;

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
        
        // 使用 create_and_store_session 来存储会话
        let session = service.create_and_store_session("test-agent", access);
        let session_id = session.session_id.clone();
        
        let retrieved = service.get_session(&session_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().agent_id, "test-agent");
    }

    #[tokio::test]
    async fn test_search_with_access_no_provider() {
        let mut service = MemoryAgentService::with_provider(Arc::new(NullDataProvider));
        let access = AgentAccess {
            allowed_tags: vec!["github".to_string()],
            allowed_types: vec!["credential".to_string()],
            max_read: 10,
            max_write: 5,
        };
        
        let session = service.create_and_store_session("test-agent", access);
        let session_id = session.session_id.clone();
        
        // NullDataProvider 返回空结果
        let results = service.search_with_access(
            &session_id,
            "github",
            Some("credential"),
            None,
            10,
        ).await.unwrap();
        
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_search_with_access_permission_denied() {
        let mut service = MemoryAgentService::new();
        let access = AgentAccess {
            allowed_tags: vec!["github".to_string()],
            allowed_types: vec!["credential".to_string()],
            max_read: 10,
            max_write: 5,
        };
        
        let session = service.create_and_store_session("test-agent", access);
        let session_id = session.session_id.clone();
        
        // 尝试搜索不允许的类型
        let result = service.search_with_access(
            &session_id,
            "test",
            Some("config"), // 不在 allowed_types 中
            None,
            10,
        ).await;
        
        assert!(result.is_err());
        match result {
            Err(AgentError::PermissionDenied(_)) => {},
            _ => panic!("Expected PermissionDenied"),
        }
    }

    #[tokio::test]
    async fn test_search_with_access_session_not_found() {
        let service = MemoryAgentService::new();
        
        let result = service.search_with_access(
            "nonexistent-session",
            "test",
            None,
            None,
            10,
        ).await;
        
        assert!(result.is_err());
        match result {
            Err(AgentError::SessionNotFound) => {},
            _ => panic!("Expected SessionNotFound"),
        }
    }

    #[tokio::test]
    async fn test_with_provider_constructor() {
        let provider = Arc::new(NullDataProvider);
        let mut service = MemoryAgentService::with_provider(provider);
        
        let access = AgentAccess::default();
        let session = service.create_and_store_session("test-agent", access);
        assert!(!session.session_id.is_empty());
    }
}
