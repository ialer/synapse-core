//! 工具注册模块
//! 
//! 定义和管理 Agent 可用的工具。

use std::collections::HashMap;
use async_trait::async_trait;

/// 工具定义
#[derive(Debug, Clone)]
pub struct Tool {
    /// 工具名称
    pub name: String,
    
    /// 工具描述
    pub description: String,
    
    /// 输入参数 schema
    pub input_schema: serde_json::Value,
    
    /// 所需权限
    pub permissions: Vec<String>,
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// 是否成功
    pub success: bool,
    
    /// 输出内容
    pub output: String,
    
    /// 元数据
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 工具执行器 trait
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// 执行工具
    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, String>;
}

/// 工具注册表
pub struct ToolRegistry {
    /// 已注册的工具
    tools: HashMap<String, Tool>,
    
    /// 工具执行器
    executors: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    /// 创建新的工具注册表
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            executors: HashMap::new(),
        }
    }
    
    /// 注册工具
    pub fn register(&mut self, tool: Tool, executor: Box<dyn ToolExecutor>) {
        let name = tool.name.clone();
        self.tools.insert(name.clone(), tool);
        self.executors.insert(name, executor);
    }
    
    /// 获取工具信息
    pub fn get_tool(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }
    
    /// 列出所有工具
    pub fn list_tools(&self) -> Vec<&Tool> {
        self.tools.values().collect()
    }
    
    /// 执行工具
    pub async fn execute_tool(&self, name: &str, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, String> {
        let tool = self.tools.get(name).ok_or_else(|| format!("Tool not found: {}", name))?;
        let executor = self.executors.get(name).ok_or_else(|| format!("Executor not found: {}", name))?;
        
        // 检查权限（简化版本）
        // TODO: 实现完整的权限检查
        
        executor.execute(args).await
    }
    
    /// 检查工具是否存在
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
    
    /// 获取工具数量
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 预定义工具：搜索个人数据
pub struct SearchDataTool;

#[async_trait]
impl ToolExecutor for SearchDataTool {
    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, String> {
        let query = args.get("query")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'query' argument")?;
        
        let data_type = args.get("data_type")
            .and_then(|v| v.as_str());
        
        let limit = args.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        
        // TODO: 实际搜索逻辑
        let results = vec![
            serde_json::json!({
                "id": "example-1",
                "type": "credential",
                "name": "GitHub Token",
                "score": 0.95
            }),
        ];
        
        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&serde_json::json!({
                "query": query,
                "data_type": data_type,
                "limit": limit,
                "results": results,
                "total": results.len()
            })).unwrap(),
            metadata: HashMap::new(),
        })
    }
}

/// 预定义工具：获取个人数据
pub struct GetDataTool;

#[async_trait]
impl ToolExecutor for GetDataTool {
    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, String> {
        let id = args.get("id")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'id' argument")?;
        
        // TODO: 实际获取逻辑
        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&serde_json::json!({
                "id": id,
                "type": "credential",
                "name": "GitHub Token",
                "content": "***encrypted***",
                "tags": ["github", "token"],
                "created_at": "2026-05-01T00:00:00Z",
                "version": 1
            })).unwrap(),
            metadata: HashMap::new(),
        })
    }
}

/// 预定义工具：创建个人数据
pub struct CreateDataTool;

#[async_trait]
impl ToolExecutor for CreateDataTool {
    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, String> {
        let data_type = args.get("data_type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'data_type' argument")?;
        
        let content = args.get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'content' argument")?;
        
        let tags: Vec<String> = args.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        // TODO: 实际创建逻辑
        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&serde_json::json!({
                "success": true,
                "id": "new-id",
                "message": format!("Created {} data with {} tags", data_type, tags.len())
            })).unwrap(),
            metadata: HashMap::new(),
        })
    }
}

/// 创建默认工具注册表
pub fn create_default_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    
    // 注册搜索工具
    registry.register(
        Tool {
            name: "search_data".to_string(),
            description: "Search personal data by query, type, or tags".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    },
                    "data_type": {
                        "type": "string",
                        "description": "Data type filter (credential, config, file, contact, generic)"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results",
                        "default": 10
                    }
                },
                "required": ["query"]
            }),
            permissions: vec!["read".to_string()],
        },
        Box::new(SearchDataTool),
    );
    
    // 注册获取工具
    registry.register(
        Tool {
            name: "get_data".to_string(),
            description: "Get personal data by ID".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Data ID"
                    }
                },
                "required": ["id"]
            }),
            permissions: vec!["read".to_string()],
        },
        Box::new(GetDataTool),
    );
    
    // 注册创建工具
    registry.register(
        Tool {
            name: "create_data".to_string(),
            description: "Create new personal data".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "data_type": {
                        "type": "string",
                        "description": "Data type (credential, config, file, contact, generic)"
                    },
                    "content": {
                        "type": "string",
                        "description": "Data content"
                    },
                    "tags": {
                        "type": "array",
                        "items": {
                            "type": "string"
                        },
                        "description": "Tags for the data"
                    }
                },
                "required": ["data_type", "content"]
            }),
            permissions: vec!["write".to_string()],
        },
        Box::new(CreateDataTool),
    );
    
    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_registry() {
        let registry = create_default_registry();
        
        assert_eq!(registry.tool_count(), 3);
        assert!(registry.has_tool("search_data"));
        assert!(registry.has_tool("get_data"));
        assert!(registry.has_tool("create_data"));
    }

    #[tokio::test]
    async fn test_search_tool() {
        let registry = create_default_registry();
        
        let mut args = HashMap::new();
        args.insert("query".to_string(), serde_json::json!("github"));
        
        let result = registry.execute_tool("search_data", args).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_get_tool() {
        let registry = create_default_registry();
        
        let mut args = HashMap::new();
        args.insert("id".to_string(), serde_json::json!("test-id"));
        
        let result = registry.execute_tool("get_data", args).await.unwrap();
        assert!(result.success);
    }
}
