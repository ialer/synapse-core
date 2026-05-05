# SynapseCore Phase 5 开发计划

> 制定人: 宁薇 (调度官) | 日期: 2026-05-06
> 项目: /home/ningwei/synapse-core | 11,251 行 Rust | 168 测试

## 一、当前状态

### 已完成
- Phase 1: 数据模型重构 + Provider 插件 + 流式管道 ✅
- Phase 2: 共享授权模型 + 消息集成 ✅
- Phase 3: MCP Agent + CLI 工具 ✅
- Phase 4: Web UI 重设计 ✅
- 安全加固: JWT/Argon2/AES-256 ✅

### 架构审查遗留问题
| ID | 问题 | 严重度 | 状态 |
|----|------|--------|------|
| I1 | SynapseCore 和 SynapseApp 重复 API | 中 | 待修 |
| I2 | 存储后端大量重复代码 | 中 | 待修 |
| I3 | MCP 工具实现 | 低 | Phase 3 已部分完成 |
| I4 | 无集成测试 | 高 | 待建 |
| I5 | search_indexer 不必要依赖 storage_backends | 低 | 待修 |

### 测试覆盖
| Crate | 测试数 | 覆盖评估 |
|-------|--------|---------|
| agent_interface | 39 | ⭐⭐⭐ 良好 |
| synapse_service | 36 | ⭐⭐⭐ 良好 |
| data_core | 21 | ⭐⭐ 良好 |
| iam_core | 16 | ⭐⭐ 良好 |
| storage_backends | 12 | ⭐ 需补充 |
| sync_engine | 8 | ⭐ 需补充 |
| search_indexer | 7 | ⭐ 需补充 |
| messaging_service | 5 | ⭐ 需补充 |
| synapse-connect | 0 | ❌ 缺失 |

## 二、任务分配

### 分配原则

| 能力 | 宁薇 (Hermes) | 宁织 (OpenClaw) |
|------|---------------|-----------------|
| 代码编写 | ✅ 主力 | ❌ exec 不可用 |
| 代码审查 | ✅ 主力 | ⚠️ 只读分析 |
| Web 搜索/研究 | ✅ | ✅ 主力 |
| 文档编写 | ✅ | ✅ 主力 |
| 测试执行 | ✅ (cargo test) | ❌ |
| 架构分析 | ✅ | ⚠️ 只读 |
| 文件操作 | ✅ | ✅ 只读 |

### 核心分工

```
宁薇 (代码执行者):                    宁织 (研究/分析者):
├─ 代码重构与修复                      ├─ Rust 最佳实践研究
├─ 编写集成测试                        ├─ 依赖分析与兼容性检查
├─ cargo test/build 验证               ├─ 文档撰写与整理
├─ Git 提交与版本管理                  ├─ 代码只读审查 (file_fetch)
└─ 问题诊断与修复                      └─ 测试用例设计
```

## 三、开发任务清单

### Sprint 1: 架构清理 (2-3天)

| # | 任务 | 负责 | 预估 | 依赖 |
|---|------|------|------|------|
| 1.1 | **I5: 移除 search_indexer 对 storage_backends 的不必要依赖** | 宁薇 | 1h | - |
| 1.2 | **I2: 泛型化存储后端** — 用 OpendalBackend<T> 统一 LocalBackend/WebdavBackend/S3Backend | 宁薇 | 4h | 1.1 |
| 1.3 | **I1: 统一 SynapseCore/SynapseApp API** — 消除重复接口 | 宁薇 | 3h | 1.2 |
| 1.4 | **研究: Rust workspace 最佳依赖管理** | 宁织 | 1h | - |
| 1.5 | **审查: 重构后的模块边界** | 宁织 | 1h | 1.3 |

### Sprint 2: 集成测试 (2-3天)

| # | 任务 | 负责 | 预估 | 依赖 |
|---|------|------|------|------|
| 2.1 | **设计集成测试架构** — 测试场景、fixture、mock 策略 | 宁织 | 2h | - |
| 2.2 | **编写核心流程测试** — 注册→登录→存储→检索→共享→删除 | 宁薇 | 4h | 2.1 |
| 2.3 | **编写存储后端集成测试** — Local + WebDAV 端到端 | 宁薇 | 3h | 2.1 |
| 2.4 | **编写 MCP 协议测试** — Initialize→ListTools→CallTool 全流程 | 宁薇 | 3h | 2.1 |
| 2.5 | **补充 synapse-connect 测试** — Provider trait 实现测试 | 宁薇 | 2h | 2.1 |
| 2.6 | **代码审查: 测试覆盖度报告** | 宁织 | 1h | 2.5 |

### Sprint 3: MCP 工具实现 (3-4天)

| # | 任务 | 负责 | 预估 | 依赖 |
|---|------|------|------|------|
| 3.1 | **研究: MCP 工具最佳实践** (Anthropic 官方规范) | 宁织 | 2h | - |
| 3.2 | **实现 search_data 工具** — 语义搜索 + 标签过滤 | 宁薇 | 3h | 3.1 |
| 3.3 | **实现 create/update/delete_data 工具** — 完整 CRUD | 宁薇 | 3h | 3.1 |
| 3.4 | **实现 share_data 工具** — 共享授权操作 | 宁薇 | 3h | 3.1 |
| 3.5 | **实现 list_tools 动态注册** — 工具自动发现 | 宁薇 | 2h | 3.2-3.4 |
| 3.6 | **集成测试: MCP 端到端** | 宁薇 | 2h | 3.5 |
| 3.7 | **文档: MCP API 参考手册** | 宁织 | 2h | 3.5 |

### Sprint 4: 质量与发布 (2天)

| # | 任务 | 负责 | 预估 | 依赖 |
|---|------|------|------|------|
| 4.1 | **代码审查: 全量安全审计** | 宁织 | 2h | - |
| 4.2 | **修复审查发现的问题** | 宁薇 | 3h | 4.1 |
| 4.3 | **性能基准测试** | 宁薇 | 2h | 4.2 |
| 4.4 | **更新 README + CHANGELOG** | 宁织 | 1h | 4.3 |
| 4.5 | **Git tag + GitHub Release** | 宁薇 | 0.5h | 4.4 |

## 四、协作流程

### 每个 Sprint 的标准流程

```
Sprint 开始
    ↓
宁织: 研究/设计 (1-2h)
    ├─ Web 搜索最佳实践
    ├─ 分析现有代码 (file_fetch)
    └─ 输出: 设计文档/测试用例
    ↓
宁薇: 实现 (3-5h)
    ├─ 代码编写
    ├─ cargo test 验证
    └─ git commit
    ↓
宁织: 审查 (1h)
    ├─ 代码只读审查
    ├─ 测试覆盖度检查
    └─ 输出: 审查报告
    ↓
宁薇: 修复 + 提交
    ↓
Sprint 完成
```

### 通信协议

宁薇通过 ACP CLI 向宁织派发研究/分析任务：

```bash
# 研究任务
openclaw agent --agent ningweiwei -m "研究 xxx 最佳实践，输出 markdown 报告" --json --timeout 120

# 分析任务
openclaw agent --agent ningweiwei -m "读取文件 xxx，分析代码结构，列出改进建议" --json --timeout 120

# 文档任务
openclaw agent --agent ningweiwei -m "根据以下内容撰写 API 文档: xxx" --json --timeout 120
```

## 五、风险与应对

| 风险 | 概率 | 影响 | 应对 |
|------|------|------|------|
| 重构引入回归 | 中 | 高 | 每个 PR 必须全量测试通过 |
| MCP 规范变更 | 低 | 中 | 宁织持续监控 Anthropic 文档 |
| 依赖版本冲突 | 中 | 中 | 锁定 Cargo.lock，渐进升级 |
| 测试环境不稳定 | 低 | 低 | 使用 tempfile 隔离 |

## 六、验收标准

### Sprint 1 验收
- [ ] `cargo test --workspace` 全部通过
- [ ] 无 `#[allow(deprecated)]` 或 TODO 注释
- [ ] 代码行数减少 (去重效果)

### Sprint 2 验收
- [ ] 集成测试 ≥ 20 个
- [ ] 覆盖核心 CRUD + 共享 + MCP 流程
- [ ] 测试执行时间 < 30s

### Sprint 3 验收
- [ ] MCP 工具 10/10 全部真实实现 (非 stub)
- [ ] 端到端测试通过
- [ ] API 文档完整

### Sprint 4 验收
- [ ] 全量测试通过 (目标: 200+)
- [ ] 无安全漏洞
- [ ] README 更新
- [ ] Git tag v0.2.0
