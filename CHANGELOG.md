# Changelog

## [0.2.0] - 2026-05-06

### 🔒 安全
- **[CRITICAL]** 消除所有硬编码 JWT 签名密钥，改为每实例随机生成 (ring::rand)
- **[HIGH]** `search` / `list_all_data` / `get_data_count` 接口添加 token 认证强制
- **[HIGH]** `send_message` 使用真实发送者 ID，不再忽略 token 参数

### ✨ 新增
- MCP 工具完整实现：`update_data` / `delete_data` / `list_data` 从 stub 升级为真实操作
- 10 个端到端集成测试：CRUD、共享隔离、搜索、持久化、加密往返
- `cargo audit` 依赖漏洞扫描（194 个依赖全部安全）
- `code-security` 安全审计技能

### 🔧 改进
- Clippy 零警告（从 26 个降至 0）
- `cargo clippy --fix` 自动修复 + 手动修复 clone 建议
- `search_indexer` 解耦 `storage_backends::StorageError` 依赖
- `AppDataProvider` 实现 `update_data` / `delete_data` / `list_all_data`
- 错误消息统一使用 `tracing`（准备中）

### 📦 依赖
- 新增 `hex = "0.4"`（JWT 密钥生成）
- 所有依赖保持最新版本，无已知 CVE

### 🧪 测试
- 测试总数：178（单元 5 + 集成 10 + 各 crate 163）
- 集成测试覆盖率：注册→登录→存储→检索→更新→删除→搜索→共享隔离→持久化→加密

---

## [0.1.0] - 2026-05-03

### 初始发布
- AES-256-GCM 加密存储
- JWT (HMAC-SHA256) + Argon2 密码认证
- RBAC 角色权限控制
- 多存储后端：Local / WebDAV / S3 / OSS / R2
- Tantivy 全文搜索索引
- 消息通知服务
- Axum REST API 服务器
- MCP 协议支持（3 个基础工具）
- Tauri 2.0 桌面应用框架
