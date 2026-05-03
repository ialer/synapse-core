use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "synapse", about = "SynapseCore 命令行工具")]
struct Cli {
    /// 数据目录
    #[arg(short, long, default_value = "./data")]
    data_dir: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 注册用户
    Register { username: String, password: String },
    /// 登录
    Login { username: String, password: String },
    /// 存储数据
    Store { 
        #[arg(short = 'T', long)]
        token: String,
        #[arg(short, long)]
        data_type: String,
        #[arg(short, long)]
        content: String,
        #[arg(short, long, num_args = 0..)]
        tags: Vec<String>,
    },
    /// 获取数据
    Get {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        id: String,
    },
    /// 列出数据
    List {
        #[arg(short, long)]
        token: String,
    },
    /// 搜索数据
    Search {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        query: String,
    },
    /// 删除数据
    Delete {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        id: String,
    },
    /// 系统统计
    Stats {
        #[arg(short, long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut app = synapse_service::SynapseApp::new_local(&cli.data_dir).await?;
    app.init().await?;
    
    match cli.command {
        Commands::Register { username, password } => {
            let token = app.register(&username, &password).await?;
            println!("注册成功");
            println!("Token: {}", token);
        }
        Commands::Login { username, password } => {
            let token = app.login(&username, &password).await?;
            println!("登录成功");
            println!("Token: {}", token);
        }
        Commands::Store { token, data_type, content, tags } => {
            let dt = data_core::DataType::from_str(&data_type).unwrap_or(data_core::DataType::Generic);
            let entity = app.secure_store(&token, dt, content.into_bytes(), tags).await?;
            println!("存储成功");
            println!("ID: {}", entity.id);
        }
        Commands::Get { token, id } => {
            let (entity, decrypted) = app.secure_get_decrypted(&token, &id).await?;
            println!("ID: {}", entity.id);
            println!("类型: {}", entity.data_type);
            println!("内容: {}", String::from_utf8_lossy(&decrypted));
            println!("标签: {:?}", entity.tags);
        }
        Commands::List { token: _ } => {
            let items = app.list_all_data();
            for item in &items {
                println!("{} [{}] tags:{:?}", item.id, item.data_type, item.tags);
            }
        }
        Commands::Search { token: _, query } => {
            let results = app.search(&query, 20);
            for r in &results {
                println!("{}: {}", r.id, r.content);
            }
        }
        Commands::Delete { token, id } => {
            app.secure_delete(&token, &id).await?;
            println!("删除成功");
        }
        Commands::Stats { token: _ } => {
            let stats = app.stats();
            println!("数据: {} 条", stats.data_count);
            println!("索引: {} 条", stats.index_count);
            println!("消息: {} 条", stats.message_count);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use super::{Cli, Commands};

    #[test]
    fn test_cli_parse_register() {
        let cli = Cli::try_parse_from([
            "synapse",
            "--data-dir", "/tmp/test",
            "register", "alice", "secret123",
        ]).unwrap();
        
        assert_eq!(cli.data_dir, "/tmp/test");
        match cli.command {
            Commands::Register { username, password } => {
                assert_eq!(username, "alice");
                assert_eq!(password, "secret123");
            }
            _ => panic!("Expected Register command"),
        }
    }

    #[test]
    fn test_cli_parse_store() {
        let cli = Cli::try_parse_from([
            "synapse",
            "store",
            "-T", "my-token",
            "-d", "credential",
            "-c", "github_pat_xxx",
            "-t", "github", "token",
        ]).unwrap();
        
        match cli.command {
            Commands::Store { token, data_type, content, tags } => {
                assert_eq!(token, "my-token");
                assert_eq!(data_type, "credential");
                assert_eq!(content, "github_pat_xxx");
                assert_eq!(tags, vec!["github".to_string(), "token".to_string()]);
            }
            _ => panic!("Expected Store command"),
        }
    }

    #[test]
    fn test_cli_parse_default_data_dir() {
        let cli = Cli::try_parse_from([
            "synapse",
            "stats", "-t", "tok",
        ]).unwrap();
        
        assert_eq!(cli.data_dir, "./data");
    }
}
