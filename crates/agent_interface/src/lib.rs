//! # Agent Interface - Agent 接口模块
//! 
//! 提供 MCP 协议和 CLI 接口，支持 AI Agent 访问个人资料库。
//! 
//! ## 核心功能
//! 
//! - MCP Server: 标准 MCP 协议实现
//! - CLI Interface: 命令行工具接口
//! - Tool Registry: 工具注册与发现
//! - Permission Control: 访问权限控制

pub mod mcp;
pub mod cli;
pub mod tools;
pub mod error;

pub use mcp::McpServer;
pub use cli::CliInterface;
pub use tools::{ToolRegistry, Tool};
pub use error::AgentError;
