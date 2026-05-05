# SynapseCore 代码质量改进计划

> 基于: Rust 1.95.0 + 代码审查 + Clippy 分析
> 日期: 2026-05-06

## 依赖版本分析

| 依赖 | 当前版本 | 最新版本 | 行动 | 风险 |
|------|---------|---------|------|------|
| thiserror | 1.0.69 | **2.x** | ⚠️ 升级 | 中 (API 变化) |
| async-trait | 0.1.89 | **可移除** | ✅ 用 RPITIT 替代 | 低 |
| axum | 0.7.9 | 0.7.x (最新) | ✅ 已最新 | - |
| opendal | 0.55.0 | 需确认 | ⏸️ 暂不动 | - |
| ring | 0.17.14 | 0.17.x (最新) | ✅ 已最新 | - |
| tokio | 1.52.1 | 1.x (最新) | ✅ 已最新 | - |
| serde | 1.0.228 | 1.x (最新) | ✅ 已最新 | - |
| argon2 | 0.5.x | 0.5.x (最新) | ✅ 已最新 | - |

## Clippy 警告修复 (13个)

1. `this impl can be derived` — 用 #[derive] 替代手动 impl
2. `method from_str can be confused` — 重命名方法
3. `match expression looks like matches! macro` — 用 matches! 宏
4. `consider adding Default implementation` — 添加 #[derive(Default)]
5. `unused import` — 删除未使用导入 (4处)
6. `&PathBuf instead of &Path` — 改用 &Path
7. `redundant closure` — 简化闭包 (2处)

## 代码现代化

### 1. 移除 async-trait，使用 RPITIT
Rust 1.75+ 原生支持 `async fn in trait`，不再需要 async-trait crate。

```rust
// 旧: use async_trait::async_trait;
// #[async_trait]
// pub trait MyTrait: Send + Sync {
//     async fn do_something(&self) -> Result<(), Error>;
// }

// 新: Rust 1.75+ 原生支持
pub trait MyTrait: Send + Sync {
    fn do_something(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
```

**影响范围**: agent_interface, synapse_service 中的 DataProvider trait
**风险**: 中 — 需要修改所有 trait 实现

### 2. thiserror 1.x → 2.x
thiserror 2.0 改进了 derive 宏，支持更多特性。

**影响范围**: 所有 error.rs 文件
**风险**: 中 — API 可能有变化

### 3. Clippy 警告修复
**影响范围**: 多个文件
**风险**: 低 — 纯重构

## 实施顺序

### Phase A: Clippy 修复 (低风险)
1. 修复 13 个 Clippy 警告
2. 运行 cargo clippy 确认无警告
3. 运行 cargo test 确认无回归

### Phase B: async-trait 移除 (中风险)
1. 替换 agent_interface/src/lib.rs 中的 DataProvider trait
2. 替换 synapse_service 中的 async trait
3. 移除 async-trait 依赖
4. 运行全量测试

### Phase C: thiserror 升级 (中风险)
1. 更新 Cargo.toml 版本
2. 修复 API 变化
3. 运行全量测试

### Phase D: 验证与提交
1. cargo test --workspace
2. cargo clippy --workspace
3. git commit
