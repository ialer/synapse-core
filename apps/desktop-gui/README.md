# SynapseCore Desktop

SynapseCore 桌面端应用，基于 Tauri + Vue.js 3 构建。

## 功能特性

- 🔐 凭证管理：安全存储和管理密码、API Key
- 📋 配置同步：跨设备同步应用配置
- 📁 文件管理：加密存储重要文件
- 👥 联系人管理：管理个人联系人信息
- 🔍 智能搜索：全文搜索和分类检索
- 🤖 Agent 集成：支持 AI Agent 访问个人数据

## 技术栈

- **后端**: Rust + Tauri
- **前端**: Vue.js 3 + TypeScript
- **构建**: Vite
- **数据**: SynapseCore 核心库

## 开发环境

### 前置要求

- Node.js 18+
- Rust 1.70+
- Cargo

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI
cargo install tauri-cli
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri dev
```

### 构建

```bash
# 构建生产版本
npm run tauri build
```

## 项目结构

```
desktop-gui/
├── src-tauri/              # Tauri 后端
│   ├── src/
│   │   └── main.rs         # Rust 入口
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── src/                    # Vue.js 前端
│   ├── main.ts             # Vue 入口
│   ├── App.vue             # 主组件
│   └── vite-env.d.ts       # 类型定义
├── index.html              # HTML 入口
├── package.json            # Node.js 依赖
├── vite.config.ts          # Vite 配置
└── tsconfig.json           # TypeScript 配置
```

## Tauri 命令

### login
用户登录，返回访问令牌。

```typescript
const token = await invoke('login', { 
  username: 'user', 
  password: 'pass' 
})
```

### store_data
存储加密数据。

```typescript
const id = await invoke('store_data', {
  token: 'access-token',
  dataType: 'credential',
  content: 'my-secret-data',
  tags: ['github', 'token']
})
```

### get_data
获取数据详情。

```typescript
const data = await invoke('get_data', {
  token: 'access-token',
  id: 'data-id'
})
```

### search_data
搜索数据。

```typescript
const results = await invoke('search_data', {
  query: 'github',
  limit: 10
})
```

### delete_data
删除数据。

```typescript
await invoke('delete_data', {
  token: 'access-token',
  id: 'data-id'
})
```

### get_stats
获取统计信息。

```typescript
const stats = await invoke('get_stats')
// { data_count: 10, index_count: 10, message_count: 5 }
```

## 截图

（待添加）

## 许可证

MIT License
