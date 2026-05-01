//! # Agent Interface - Agent 接口模块
//! 
//! 提供 MCP 协议和 CLI 接口，支持 AI Agent 访问个人资料库。

pub mod mcp;
pub mod cli;
pub mod tools;
pub mod agent_service;

pub use mcp::McpServer;
pub use cli::CliInterface;
pub use tools::{ToolRegistry, Tool};
pub use agent_service::{AgentService, MemoryAgentService, AgentSession, AgentAccess, AgentError};
