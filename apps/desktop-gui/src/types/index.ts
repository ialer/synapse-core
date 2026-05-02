// SynapseCore TypeScript 类型定义
// 对应 Rust 后端类型

/** 搜索结果 */
export interface SearchResult {
  id: string
  content: string
  metadata: Record<string, string>
}

/** 统计信息 */
export interface StatsInfo {
  data_count: number
  index_count: number
  message_count: number
}

/** 消息信息 */
export interface MessageInfo {
  id: string
  from: string
  to: string
  title: string
  content: string
  timestamp: string
}

/** 数据类型枚举 */
export type DataType = 'credential' | 'config' | 'file' | 'contact' | 'generic'

/** 数据条目（前端扩展） */
export interface DataItem {
  id: string
  data_type: DataType
  content: string
  tags: string[]
  created_at?: string
  updated_at?: string
}

/** 存储表单 */
export interface StoreForm {
  dataType: DataType
  content: string
  tags: string
}

/** 搜索表单 */
export interface SearchForm {
  query: string
  limit: number
}

/** 登录表单 */
export interface LoginForm {
  username: string
  password: string
}

/** 消息发送表单 */
export interface MessageForm {
  recipientId: string
  title: string
  content: string
}

/** 设置项 */
export interface AppSettings {
  storageBackend: string
  storagePath: string
  autoLock: boolean
  lockTimeout: number
}

/** 组件通用状态 */
export interface LoadingState {
  loading: boolean
  error: string | null
}

/** 数据类型选项 */
export const DATA_TYPE_OPTIONS: { value: DataType; label: string; icon: string }[] = [
  { value: 'credential', label: '凭证', icon: '🔑' },
  { value: 'config', label: '配置', icon: '⚙️' },
  { value: 'file', label: '文件', icon: '📁' },
  { value: 'contact', label: '联系人', icon: '👤' },
  { value: 'generic', label: '通用', icon: '📄' },
]
