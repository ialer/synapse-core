// SynapseCore API - HTTP fetch adapter
// Works in both Tauri (desktop) and browser (web) modes
// All communication goes through the HTTP web server

import { request } from './adapter'
import { setToken, clearToken } from './auth'
import type { SearchResult, StatsInfo, MessageInfo, DataType, DataItemInfo, StorageInfo } from '../types'

// ============================================================
// Auth
// ============================================================

/** Login - stores token automatically */
export async function login(username: string, password: string): Promise<string> {
  const resp = await request<{ token: string }>('POST', '/api/login', { username, password })
  setToken(resp.token)
  return resp.token
}

/** Register user - stores token automatically */
export async function registerUser(username: string, password: string): Promise<string> {
  const resp = await request<{ token: string }>('POST', '/api/register', { username, password })
  setToken(resp.token)
  return resp.token
}

/** Logout - clears stored token */
export function logout(): void {
  clearToken()
}

// ============================================================
// Data
// ============================================================

/** Store data - returns the new item ID */
export async function storeData(
  token: string,
  dataType: DataType,
  content: string,
  tags: string[]
): Promise<string> {
  const resp = await request<{ id: string }>('POST', '/api/data', {
    token,
    data_type: dataType,
    content,
    tags,
  })
  return resp.id
}

/** Get data detail by ID - returns the decrypted content string */
export async function getData(token: string, id: string): Promise<string> {
  const resp = await request<{ content: string }>('GET', `/api/data/${id}?token=${encodeURIComponent(token)}`)
  return resp.content
}

/** List all data items for a user */
export async function listData(token: string): Promise<DataItemInfo[]> {
  const resp = await request<{ items: DataItemInfo[]; total: number }>('GET', `/api/data/list?token=${encodeURIComponent(token)}`)
  return resp.items
}

/** Update existing data */
export async function updateData(
  token: string,
  id: string,
  content: string,
  tags: string[]
): Promise<boolean> {
  const resp = await request<{ success: boolean }>('PUT', `/api/data/${id}`, {
    token,
    content,
    tags,
  })
  return resp.success
}

/** Delete data by ID */
export async function deleteData(token: string, id: string): Promise<boolean> {
  const resp = await request<{ success: boolean }>('DELETE', `/api/data/${id}?token=${encodeURIComponent(token)}`)
  return resp.success
}

// ============================================================
// Search
// ============================================================

/** Search data by query */
export async function searchData(query: string, limit: number = 10): Promise<SearchResult[]> {
  return request<SearchResult[]>('GET', `/api/search?q=${encodeURIComponent(query)}&limit=${limit}`)
}

// ============================================================
// Stats
// ============================================================

/** Get system statistics */
export async function getStats(): Promise<StatsInfo> {
  return request<StatsInfo>('GET', '/api/stats')
}

// ============================================================
// Messages
// ============================================================

/** Send a message */
export async function sendMessage(
  token: string,
  recipientId: string,
  title: string,
  content: string
): Promise<boolean> {
  const resp = await request<{ success: boolean }>('POST', '/api/messages', {
    token,
    recipient_id: recipientId,
    title,
    content,
  })
  return resp.success
}

/** Get messages for a user - maps API field names to frontend types */
export async function getMessages(userId: string, limit: number = 50): Promise<MessageInfo[]> {
  const resp = await request<Array<{
    id: string
    sender_id: string
    recipient_id: string
    title: string
    content: string
    sent_at: string
    is_read: boolean
  }>>(  'GET', `/api/messages/${encodeURIComponent(userId)}?limit=${limit}`)

  // Map API response fields to frontend MessageInfo type
  return resp.map((m) => ({
    id: m.id,
    from: m.sender_id,
    to: m.recipient_id,
    title: m.title,
    content: m.content,
    timestamp: m.sent_at,
  }))
}

// ============================================================
// Storage Info
// ============================================================

/** Get storage configuration info */
export async function getStorageInfo(): Promise<StorageInfo> {
  // No dedicated endpoint in the web server yet
  // Return a default value for browser mode
  return request<StorageInfo>('GET', '/api/health')
    .then(() => ({
      backend_type: 'local',
      is_configured: true,
    }))
    .catch(() => ({
      backend_type: 'unknown',
      is_configured: false,
    }))
}
