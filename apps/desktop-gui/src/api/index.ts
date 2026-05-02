// SynapseCore API 封装
// Tauri invoke 命令包装，类型安全

import { invoke } from '@tauri-apps/api/core'
import type { SearchResult, StatsInfo, MessageInfo, DataType } from '../types'

/** 登录 */
export async function login(username: string, password: string): Promise<string> {
  return await invoke('login', { username, password })
}

/** 存储数据 */
export async function storeData(
  token: string,
  dataType: DataType,
  content: string,
  tags: string[]
): Promise<string> {
  return await invoke('store_data', { token, dataType, content, tags })
}

/** 获取数据 */
export async function getData(token: string, id: string): Promise<string> {
  return await invoke('get_data', { token, id })
}

/** 搜索数据 */
export async function searchData(query: string, limit: number = 10): Promise<SearchResult[]> {
  return await invoke('search_data', { query, limit })
}

/** 删除数据 */
export async function deleteData(token: string, id: string): Promise<boolean> {
  return await invoke('delete_data', { token, id })
}

/** 获取统计信息 */
export async function getStats(): Promise<StatsInfo> {
  return await invoke('get_stats')
}

/** 发送消息 */
export async function sendMessage(
  token: string,
  recipientId: string,
  title: string,
  content: string
): Promise<boolean> {
  return await invoke('send_message', { token, recipientId, title, content })
}

/** 获取用户消息 */
export async function getMessages(userId: string, limit: number = 50): Promise<MessageInfo[]> {
  return await invoke('get_messages', { userId, limit })
}
