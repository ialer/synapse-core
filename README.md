# SynapseCore - 神经突触核心

> 高性能、模块化的跨平台数据管理系统

[![CI](https://github.com/ialer/synapse-core/actions/workflows/ci.yml/badge.svg)](https://github.com/ialer/synapse-core/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20Android%20%7C%20iOS-lightgrey.svg)]()

## 项目简介

SynapseCore 是一个高性能、模块化的跨平台数据管理系统，为个人/团队/家庭提供统一的数据备份、交互和管理能力。

### 核心特性

- **AES-256-GCM 加密** - 认证加密，保障数据安全
- **JWT 认证 + RBAC** - 基于角色的访问控制与令牌认证
- **多存储后端** - 本地存储、WebDAV、S3、OSS、R2
- **全文搜索** - 快速检索与分类过滤
- **消息服务** - 账户间消息通知与共享
- **Tauri 2.0 桌面/移动端应用** - 跨平台原生应用
- **REST API 服务器** - Axum 高性能 Web 服务
- **实时同步引擎** - 多设备数据同步与冲突解决
- **AI Agent 集成** - MCP 协议支持，智能体外挂数据库

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                    SynapseCore 架构                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Tauri 2.0    │  │ Web Server   │  │ Agent        │      │
│  │ Desktop GUI  │  │ (Axum REST)  │  │ Interface    │      │
│  │ Vue 3 + Vite │  │ :8080        │  │ MCP Protocol │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │                │
│         └────────┬────────┘                 │                │
│                  ▼                          │                │
│  ┌─────────────────────────────────┐       │                │
│  │      synapse_service            │       │                │
│  │   (统一网关 - 整合所有模块)      │◄──────┘                │
│  └─────────────┬───────────────────┘                        │
│                │                                            │
│    ┌───────────┼───────────────────────┐                    │
│    │           │           │           │                    │
│    ▼           ▼           ▼           ▼                    │
│ ┌──────┐  ┌────────┐  ┌───────┐  ┌─────────┐              │
│ │ data │  │  iam   │  │storage│  │ search  │              │
│ │_core │  │ _core  │  │backends│ │ indexer │              │
│ │      │  │        │  │       │  │         │              │
│ │加密  │  │认证    │  │本地   │  │全文     │              │
│ │实体  │  │RBAC    │  │WebDAV │  │搜索     │              │
│ │序列化│  │JWT     │  │S3     │  │分类     │              │
│ └──────┘  └────────┘  │OSS    │  └─────────┘              │
│                       │R2     │                            │
│                       └───────┘                            │
│    ┌───────────────────────┐  ┌───────────────────┐        │
│    │     sync_engine       │  │ messaging_service  │        │
│    │  多设备同步/冲突解决   │  │  消息通知/共享     │        │
│    └───────────────────────┘  └───────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### Crate 依赖关系

```
synapse_service ──┬──► data_core
                  ├──► iam_core ──► data_core
                  ├──► storage_backends
                  ├──► sync_engine ──► storage_backends
                  ├──► search_indexer ──► storage_backends
                  └──► messaging_service

web_server ──────┬──► synapse_service
                 └──► data_core

agent_interface ─────► data_core
```

## 快速开始

### 环境要求

- **Rust** 1.70+ (推荐最新 stable)
- **Node.js** 18+ (用于 Tauri 前端)
- **npm** 9+

### 安装与构建

```bash
# 克隆仓库
git clone https://github.com/ialer/synapse-core.git
cd synapse-core

# 构建整个项目
cargo build --release
```

### 启动 Web 服务器

```bash
# 启动 REST API 服务器 (端口 8080)
cargo run -p synapse-web

# 服务器启动后访问:
# API: http://localhost:8080/api/
# 健康检查: http://localhost:8080/api/health
```

### 启动桌面应用

```bash
# 安装依赖并启动 Tauri 开发模式
cd apps/desktop-gui
npm install
npm run tauri dev
```

### 基本使用

```rust
use synapse_service::SynapseApp;
use data_core::DataType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建应用实例（本地存储）
    let mut app = SynapseApp::new_local("./data")?;
    app.init().await?;

    // 注册用户
    let token = app.register("alice", "secure_password").await?;

    // 存储加密数据
    let entity = app.secure_store(
        &token,
        DataType::Credential,
        b"my-api-key-12345".to_vec(),
        vec!["api".to_string(), "github".to_string()],
    ).await?;

    // 搜索数据
    let results = app.search("github", 10);

    // 发送消息
    app.send_message(&token, "bob", "Hello", "Welcome to SynapseCore!")?;

    // 获取统计信息
    let stats = app.stats();
    println!("数据数量: {}, 索引数量: {}", stats.data_count, stats.index_count);

    Ok(())
}
```

## API 参考

### REST 端点

所有 API 基础路径: `http://localhost:8080/api`

| 方法 | 端点 | 说明 | 认证 |
|------|------|------|------|
| `GET` | `/health` | 健康检查 | 无 |
| `POST` | `/login` | 用户登录 | 无 |
| `POST` | `/register` | 用户注册 | 无 |
| `GET` | `/stats` | 系统统计 | 无 |
| `POST` | `/data` | 存储数据 | Token |
| `GET` | `/data/list` | 列出所有数据 | Token |
| `GET` | `/data/:id` | 获取数据详情 | Token |
| `PUT` | `/data/:id` | 更新数据 | Token |
| `DELETE` | `/data/:id` | 删除数据 | Token |
| `GET` | `/search?q=xxx&limit=10` | 全文搜索 | 无 |
| `POST` | `/messages` | 发送消息 | Token |
| `GET` | `/messages/:user_id?limit=50` | 获取消息列表 | 无 |

### 请求/响应示例

**登录**
```bash
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "secure_password"}'

# 响应: {"token": "eyJ..."}
```

**存储数据**
```bash
curl -X POST http://localhost:8080/api/data \
  -H "Content-Type: application/json" \
  -d '{
    "token": "eyJ...",
    "data_type": "credential",
    "content": "my-api-key-12345",
    "tags": ["api", "github"]
  }'

# 响应: {"id": "uuid-string"}
```

**搜索**
```bash
curl "http://localhost:8080/api/search?q=github&limit=5"

# 响应: [{"id": "...", "content": "...", "metadata": {...}}]
```

## 平台支持

| 平台 | 桌面应用 | Web 服务器 | 状态 |
|------|---------|-----------|------|
| Windows x64 | Tauri 2.0 | Axum | ✅ 完全支持 |
| macOS (Intel) | Tauri 2.0 | Axum | ✅ 完全支持 |
| macOS (Apple Silicon) | Tauri 2.0 | Axum | ✅ 完全支持 |
| Linux x64 | Tauri 2.0 | Axum | ✅ 完全支持 |
| Android | Tauri 2.0 | - | 🔄 开发中 |
| iOS | Tauri 2.0 | - | 🔄 开发中 |
| Web Browser | - | REST API | ✅ 完全支持 |

## 模块说明

### data_core - 数据核心
定义数据实体结构与加密核心功能。
- `DataEntity`: 数据实体（UUID、所有者、类型、加密内容）
- `Cipher`: AES-256-GCM 加解密器
- `DataType`: 数据类型枚举（Credential、Document、Note 等）

### iam_core - 身份认证
基于角色的访问控制（RBAC）与 JWT 认证。
- `AuthService`: 认证服务 Trait
- `Role`, `Permission`: 角色与权限枚举
- `JwtService`: JWT 签发与校验

### storage_backends - 存储后端
抽象存储层，支持多种后端实现（基于 OpenDAL）。
- `StorageBackend`: 存储后端 Trait
- `LocalBackend`: 本地文件系统存储
- `WebDavBackend`: WebDAV 远程存储
- `S3Backend`: AWS S3 / Cloudflare R2
- `OssBackend`: 阿里云 OSS

### sync_engine - 同步引擎
多设备数据同步与冲突解决。
- `SyncEngine`: 同步引擎
- `ConflictResolver`: 冲突解决策略

### search_indexer - 搜索索引
全文搜索与分类检索。
- `Indexer`: 索引构建器
- `QueryParser`: 查询解析器

### messaging_service - 消息服务
账户间消息通知与共享。
- `MessageService`: 消息服务
- `NotificationManager`: 通知管理

### agent_interface - Agent 接口
MCP 协议集成，支持 AI Agent 调用。
- `McpServer`: MCP 服务器
- `CliInterface`: CLI 接口

### synapse_service - 统一网关
整合所有模块的统一服务层。
- `SynapseApp`: 应用主入口
- 提供统一的 API 接口

### web_server - Web 服务器
基于 Axum 的 REST API 服务器。
- 完整的 CRUD API
- CORS 支持
- 静态文件服务

## 开发指南

### 项目结构

```
synapse-core/
├── Cargo.toml                          # Workspace 配置
├── crates/
│   ├── data_core/                      # 数据核心模块
│   ├── iam_core/                       # 身份认证模块
│   ├── storage_backends/               # 存储后端模块
│   ├── sync_engine/                    # 同步引擎模块
│   ├── search_indexer/                 # 搜索索引模块
│   ├── agent_interface/                # Agent 接口模块
│   ├── messaging_service/              # 消息服务模块
│   ├── synapse_service/                # 统一网关服务
│   └── web_server/                     # Web API 服务器
├── apps/
│   └── desktop-gui/                    # Tauri 2.0 桌面应用
│       ├── src-tauri/                  # Tauri Rust 后端
│       └── src/                        # Vue 3 前端
└── .github/workflows/                  # CI/CD 配置
```

### 开发环境搭建

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 Node.js (推荐使用 nvm)
nvm install 18
nvm use 18

# 3. 克隆并构建
git clone https://github.com/ialer/synapse-core.git
cd synapse-core
cargo build

# 4. 运行测试
cargo test

# 5. 代码检查
cargo clippy -- -D warnings
cargo fmt -- --check
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定 crate 测试
cargo test -p data_core
cargo test -p iam_core
cargo test -p storage_backends
cargo test -p synapse_service
```

### 添加新功能

1. Fork 项目
2. 创建特性分支: `git checkout -b feature/amazing-feature`
3. 编写代码并添加测试
4. 确保所有测试通过: `cargo test`
5. 检查代码质量: `cargo clippy && cargo fmt`
6. 提交更改: `git commit -m 'Add amazing feature'`
7. 推送分支: `git push origin feature/amazing-feature`
8. 创建 Pull Request

## 配置

项目配置通过 `config.toml` 或环境变量进行:

```toml
[storage]
default_backend = "local"
local_path = "./data"

[encryption]
algorithm = "AES-256-GCM"

[search]
index_path = "./index"
max_results = 100

[agent]
mcp_enabled = true
mcp_port = 8080
```

## 安全特性

- **AES-256-GCM** 认证加密
- **密钥派生** 使用 HKDF
- **Nonce** 每次加密唯一生成
- **关联数据** 支持 AEAD
- **RBAC** 基于角色的访问控制
- **JWT** 安全的令牌认证

## 性能指标

| 操作 | 耗时 | 说明 |
|------|------|------|
| 加密 1KB | < 1ms | AES-256-GCM |
| 解密 1KB | < 1ms | AES-256-GCM |
| 序列化 JSON | < 1ms | DataEntity |
| 序列化 MessagePack | < 0.5ms | DataEntity |
| 搜索 1000 条 | < 10ms | 全文搜索 |

## 开源协议

本项目采用 MIT 协议开源 - 查看 [LICENSE](LICENSE) 文件了解详情

## 致谢

- [Rust](https://www.rust-lang.org/) - 伟大的编程语言
- [ring](https://github.com/briansmith/ring) - 加密库
- [tokio](https://tokio.rs/) - 异步运行时
- [serde](https://serde.rs/) - 序列化框架
- [Axum](https://github.com/tokio-rs/axum) - Web 框架
- [Tauri](https://tauri.app/) - 跨平台应用框架
- [OpenDAL](https://opendal.apache.org/) - 统一存储访问层

---

**SynapseCore** - 让数据管理更智能、更安全、更高效
