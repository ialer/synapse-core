# SynapseCore - 神经突触核心

> 高性能、模块化的跨平台数据管理系统

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20Android%20%7C%20iOS-lightgrey.svg)]()

## 🎯 项目简介

SynapseCore 是一个高性能、模块化的跨平台数据管理系统，为个人/团队/家庭提供统一的数据备份、交互和管理能力。

### 核心特性

- **跨平台支持**：Windows、macOS、Linux、Android、iOS、Web
- **智能备份**：完整备份或配置备份，减少存储占用
- **多源存储**：NAS、云盘、网络硬盘、本地路径、U盘
- **数据检索**：全文搜索 + 分类检索
- **AI 集成**：MCP 协议支持，作为智能体的外挂数据库
- **安全加密**：AES-256-GCM 认证加密
- **团队协作**：账户共享、权限管理、消息通知

### 🤖 Agent 集成

SynapseCore 支持通过 MCP 协议接入 AI Agent，使其成为智能体的个人资料库：

```rust
use agent_interface::McpServer;

// 创建 MCP 服务器
let server = McpServer::new("synapse-core", "0.1.0");

// Agent 可以通过 MCP 协议：
// 1. 搜索个人数据
// 2. 获取数据详情
// 3. 创建/更新/删除数据
// 4. 管理数据标签和分类
```

**Agent 使用场景**：
- 🔐 **凭证管理**：Agent 自动填充密码、API Key
- 📋 **配置同步**：Agent 跨设备同步应用配置
- 📁 **文件检索**：Agent 智能搜索和整理文件
- 👥 **联系人管理**：Agent 辅助管理联系人信息
- 📊 **数据分析**：Agent 分析个人数据趋势

## 📁 项目结构

```
synapse-core/
├── Cargo.toml                          # Workspace 配置
├── README.md                           # 项目说明
├── LICENSE                             # 开源协议
├── src/
│   └── lib.rs                          # 主库入口
├── crates/
│   ├── data_core/                      # 数据核心模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── entity.rs               # 数据实体定义
│   │   │   ├── crypto.rs               # AES-256-GCM 加解密
│   │   │   ├── metadata.rs             # 元数据管理
│   │   │   └── error.rs                # 错误类型
│   │   └── Cargo.toml
│   ├── iam_core/                       # 身份认证模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── auth.rs                 # 认证服务
│   │   │   ├── rbac.rs                 # RBAC 访问控制
│   │   │   ├── jwt.rs                  # JWT 签发与校验
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   ├── storage_backends/               # 存储后端模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait.rs                # StorageBackend Trait
│   │   │   ├── local.rs                # 本地存储
│   │   │   ├── cloud.rs                # 云存储
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   ├── sync_engine/                    # 同步引擎模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── sync.rs                 # 同步逻辑
│   │   │   ├── conflict.rs             # 冲突解决
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   ├── search_indexer/                 # 搜索索引模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── indexer.rs              # 索引构建
│   │   │   ├── query.rs                # 查询解析
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   ├── agent_interface/                # Agent 接口模块
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── mcp.rs                  # MCP 协议实现
│   │   │   ├── cli.rs                  # CLI 接口
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   └── messaging_service/              # 消息服务模块
│       ├── src/
│       │   ├── lib.rs
│       │   ├── message.rs              # 消息处理
│       │   ├── notification.rs         # 通知管理
│       │   └── error.rs
│       └── Cargo.toml
├── docs/                               # 项目文档
│   ├── architecture.md                 # 架构设计
│   ├── api.md                          # API 文档
│   └── deployment.md                   # 部署指南
├── examples/                           # 使用示例
│   ├── basic_usage.rs
│   └── advanced_usage.rs
├── tests/                              # 集成测试
│   └── integration_tests.rs
└── benches/                            # 性能基准测试
    └── crypto_bench.rs
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- Cargo (随 Rust 一起安装)

### 安装

```bash
git clone https://github.com/ialer/synapse-core.git
cd synapse-core
cargo build --release
```

### 基本使用

```rust
use data_core::{DataEntity, DataType, Cipher};

// 创建加密器
let cipher = Cipher::new().expect("密钥生成失败");

// 加密数据
let plaintext = b"my secret data";
let ciphertext = cipher.encrypt(plaintext, None).expect("加密失败");

// 创建数据实体
let entity = DataEntity::new(
    Uuid::new_v4(),
    DataType::Credential,
    ciphertext,
);

// 序列化
let json = entity.to_json().expect("序列化失败");
```

## 📚 模块说明

### data_core - 数据核心

定义数据实体结构与加密核心功能。

- `DataEntity`: 数据实体（UUID、所有者、类型、加密内容）
- `Cipher`: AES-256-GCM 加解密器
- `Metadata`: 元数据管理

### iam_core - 身份认证

基于角色的访问控制（RBAC）与 JWT 认证。

- `AuthService`: 认证服务 Trait
- `Role`, `Permission`: 角色与权限枚举
- `JwtService`: JWT 签发与校验

### storage_backends - 存储后端

抽象存储层，支持多种后端实现。

- `StorageBackend`: 存储后端 Trait
- `LocalBackend`: 本地文件系统存储
- `CloudBackend`: 云存储（S3、OSS 等）

### sync_engine - 同步引擎

多设备数据同步与冲突解决。

- `SyncEngine`: 同步引擎
- `ConflictResolver`: 冲突解决策略

### search_indexer - 搜索索引

全文搜索与分类检索。

- `Indexer`: 索引构建器
- `QueryParser`: 查询解析器

### agent_interface - Agent 接口

MCP 协议集成，支持 AI Agent 调用。

- `McpServer`: MCP 服务器
- `CliInterface`: CLI 接口

### messaging_service - 消息服务

账户间消息通知与共享。

- `MessageService`: 消息服务
- `NotificationManager`: 通知管理

## 🔧 配置

项目配置通过 `config.toml` 或环境变量进行：

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

## 🛡️ 安全特性

- **AES-256-GCM** 认证加密
- **密钥派生** 使用 HKDF
- **Nonce** 每次加密唯一生成
- **关联数据** 支持 AEAD
- **RBAC** 基于角色的访问控制
- **JWT** 安全的令牌认证

## 📊 性能指标

| 操作 | 耗时 | 说明 |
|------|------|------|
| 加密 1KB | < 1ms | AES-256-GCM |
| 解密 1KB | < 1ms | AES-256-GCM |
| 序列化 JSON | < 1ms | DataEntity |
| 序列化 MessagePack | < 0.5ms | DataEntity |
| 搜索 1000 条 | < 10ms | 全文搜索 |

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 开源协议

本项目采用 MIT 协议开源 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Rust](https://www.rust-lang.org/) - 伟大的编程语言
- [ring](https://github.com/briansmith/ring) - 加密库
- [tokio](https://tokio.rs/) - 异步运行时
- [serde](https://serde.rs/) - 序列化框架

---

**SynapseCore** - 让数据管理更智能、更安全、更高效
