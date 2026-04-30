//! CLI 接口模块
//! 
//! 提供命令行工具接口，支持 Agent 直接调用。

use std::collections::HashMap;

/// CLI 命令
#[derive(Debug, Clone)]
pub enum CliCommand {
    /// 搜索数据
    Search {
        query: String,
        data_type: Option<String>,
        limit: Option<usize>,
    },
    
    /// 获取数据详情
    Get {
        id: String,
    },
    
    /// 创建数据
    Create {
        data_type: String,
        content: String,
        tags: Vec<String>,
    },
    
    /// 更新数据
    Update {
        id: String,
        content: Option<String>,
        tags: Option<Vec<String>>,
    },
    
    /// 删除数据
    Delete {
        id: String,
        hard: bool,
    },
    
    /// 列出所有数据
    List {
        data_type: Option<String>,
        limit: Option<usize>,
    },
    
    /// 导出数据
    Export {
        id: String,
        format: String,
    },
    
    /// 导入数据
    Import {
        file: String,
    },
}

/// CLI 结果
#[derive(Debug, Clone)]
pub struct CliResult {
    /// 是否成功
    pub success: bool,
    
    /// 输出消息
    pub message: String,
    
    /// 输出数据
    pub data: Option<String>,
}

/// CLI 接口
pub struct CliInterface {
    /// 命令历史
    history: Vec<CliCommand>,
}

impl CliInterface {
    /// 创建新的 CLI 接口
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    /// 解析命令行参数
    pub fn parse_args(args: Vec<String>) -> Result<CliCommand, String> {
        if args.len() < 2 {
            return Err("Usage: synapse-core <command> [args]".to_string());
        }
        
        match args[1].as_str() {
            "search" => {
                if args.len() < 3 {
                    return Err("Usage: synapse-core search <query> [type] [limit]".to_string());
                }
                Ok(CliCommand::Search {
                    query: args[2].clone(),
                    data_type: args.get(3).cloned(),
                    limit: args.get(4).and_then(|s| s.parse().ok()),
                })
            }
            "get" => {
                if args.len() < 3 {
                    return Err("Usage: synapse-core get <id>".to_string());
                }
                Ok(CliCommand::Get { id: args[2].clone() })
            }
            "create" => {
                if args.len() < 4 {
                    return Err("Usage: synapse-core create <type> <content> [tags...]".to_string());
                }
                Ok(CliCommand::Create {
                    data_type: args[2].clone(),
                    content: args[3].clone(),
                    tags: args[4..].to_vec(),
                })
            }
            "update" => {
                if args.len() < 3 {
                    return Err("Usage: synapse-core update <id> [content] [tags...]".to_string());
                }
                Ok(CliCommand::Update {
                    id: args[2].clone(),
                    content: args.get(3).cloned(),
                    tags: if args.len() > 4 { Some(args[4..].to_vec()) } else { None },
                })
            }
            "delete" => {
                if args.len() < 3 {
                    return Err("Usage: synapse-core delete <id> [--hard]".to_string());
                }
                let hard = args.contains(&"--hard".to_string());
                Ok(CliCommand::Delete { id: args[2].clone(), hard })
            }
            "list" => {
                Ok(CliCommand::List {
                    data_type: args.get(2).cloned(),
                    limit: args.get(3).and_then(|s| s.parse().ok()),
                })
            }
            "export" => {
                if args.len() < 4 {
                    return Err("Usage: synapse-core export <id> <format>".to_string());
                }
                Ok(CliCommand::Export {
                    id: args[2].clone(),
                    format: args[3].clone(),
                })
            }
            "import" => {
                if args.len() < 3 {
                    return Err("Usage: synapse-core import <file>".to_string());
                }
                Ok(CliCommand::Import { file: args[2].clone() })
            }
            _ => Err(format!("Unknown command: {}", args[1])),
        }
    }
    
    /// 执行命令
    pub fn execute(&mut self, command: CliCommand) -> CliResult {
        self.history.push(command.clone());
        
        match command {
            CliCommand::Search { query, data_type, limit } => {
                CliResult {
                    success: true,
                    message: format!("Searching for '{}' with type {:?} and limit {:?}", query, data_type, limit),
                    data: None,
                }
            }
            CliCommand::Get { id } => {
                CliResult {
                    success: true,
                    message: format!("Getting data with id: {}", id),
                    data: None,
                }
            }
            CliCommand::Create { data_type, content, tags } => {
                CliResult {
                    success: true,
                    message: format!("Creating {} data with tags {:?}", data_type, tags),
                    data: None,
                }
            }
            CliCommand::Update { id, content, tags } => {
                CliResult {
                    success: true,
                    message: format!("Updating data {}", id),
                    data: None,
                }
            }
            CliCommand::Delete { id, hard } => {
                CliResult {
                    success: true,
                    message: format!("Deleting data {} (hard: {})", id, hard),
                    data: None,
                }
            }
            CliCommand::List { data_type, limit } => {
                CliResult {
                    success: true,
                    message: format!("Listing data with type {:?} and limit {:?}", data_type, limit),
                    data: None,
                }
            }
            CliCommand::Export { id, format } => {
                CliResult {
                    success: true,
                    message: format!("Exporting data {} as {}", id, format),
                    data: None,
                }
            }
            CliCommand::Import { file } => {
                CliResult {
                    success: true,
                    message: format!("Importing from {}", file),
                    data: None,
                }
            }
        }
    }
    
    /// 获取命令历史
    pub fn history(&self) -> &[CliCommand] {
        &self.history
    }
}

impl Default for CliInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_search_command() {
        let args = vec![
            "synapse-core".to_string(),
            "search".to_string(),
            "github".to_string(),
            "credential".to_string(),
            "10".to_string(),
        ];
        
        let command = CliInterface::parse_args(args).unwrap();
        match command {
            CliCommand::Search { query, data_type, limit } => {
                assert_eq!(query, "github");
                assert_eq!(data_type, Some("credential".to_string()));
                assert_eq!(limit, Some(10));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_parse_get_command() {
        let args = vec![
            "synapse-core".to_string(),
            "get".to_string(),
            "test-id".to_string(),
        ];
        
        let command = CliInterface::parse_args(args).unwrap();
        match command {
            CliCommand::Get { id } => {
                assert_eq!(id, "test-id");
            }
            _ => panic!("Expected Get command"),
        }
    }

    #[test]
    fn test_execute_search() {
        let mut cli = CliInterface::new();
        let result = cli.execute(CliCommand::Search {
            query: "test".to_string(),
            data_type: None,
            limit: None,
        });
        
        assert!(result.success);
        assert!(cli.history().len() == 1);
    }
}
