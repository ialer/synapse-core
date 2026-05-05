// SynapseCore API - HTTP fetch adapter
// Works in both Tauri (desktop) and browser (web) modes
// All communication goes through the HTTP web server

import { request } from './adapter'
import { setToken, clearToken } from './auth'
import type { SearchResult, StatsInfo, MessageInfo, DataType, DataItemInfo, StorageInfo, ShareRequest, HealthStatus, FolderNode } from '../types'

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

// ============================================================
// Health
// ============================================================

/** Get system health status */
export async function getHealthStatus(): Promise<HealthStatus> {
  // Mock data - no dedicated endpoint yet
  return {
    status: 'healthy',
    uptime: Date.now() - 86400000,
    last_check: new Date().toISOString(),
    storage_backend: 'local',
    version: 'v0.1.0',
  }
}

// ============================================================
// Shares (Mock - endpoints not yet implemented)
// ============================================================

/** Get all share requests (incoming + outgoing) */
export async function getShareRequests(): Promise<ShareRequest[]> {
  // Mock data for development
  return [
    {
      id: 'share-001',
      data_id: 'data-abc',
      data_title: 'AWS Credentials',
      sender_id: 'user-1',
      sender_name: 'Alice',
      recipient_id: 'user-2',
      recipient_name: 'Bob',
      status: 'pending',
      permission: 'read',
      created_at: '2026-05-01T10:30:00Z',
      updated_at: '2026-05-01T10:30:00Z',
      message: 'Need access to deploy pipeline credentials',
    },
    {
      id: 'share-002',
      data_id: 'data-def',
      data_title: 'Server Config',
      sender_id: 'user-2',
      sender_name: 'Bob',
      recipient_id: 'user-3',
      recipient_name: 'Charlie',
      status: 'approved',
      permission: 'read',
      created_at: '2026-04-28T14:00:00Z',
      updated_at: '2026-04-29T09:15:00Z',
      message: 'Production server config for monitoring',
    },
    {
      id: 'share-003',
      data_id: 'data-ghi',
      data_title: 'Contact List',
      sender_id: 'user-3',
      sender_name: 'Charlie',
      recipient_id: 'user-1',
      recipient_name: 'Alice',
      status: 'pending',
      permission: 'write',
      created_at: '2026-05-02T16:45:00Z',
      updated_at: '2026-05-02T16:45:00Z',
      message: 'Collaborative contact database',
    },
    {
      id: 'share-004',
      data_id: 'data-jkl',
      data_title: 'API Keys',
      sender_id: 'user-1',
      sender_name: 'Alice',
      recipient_id: 'user-4',
      recipient_name: 'Diana',
      status: 'denied',
      permission: 'read',
      created_at: '2026-04-25T08:00:00Z',
      updated_at: '2026-04-26T11:30:00Z',
    },
    {
      id: 'share-005',
      data_id: 'data-mno',
      data_title: 'Shared Notes',
      sender_id: 'user-2',
      sender_name: 'Bob',
      recipient_id: 'user-1',
      recipient_name: 'Alice',
      status: 'revoked',
      permission: 'write',
      created_at: '2026-04-20T09:00:00Z',
      updated_at: '2026-05-01T12:00:00Z',
    },
  ]
}

/** Approve a share request */
export async function approveShareRequest(shareId: string): Promise<boolean> {
  console.log(`[Mock] Approving share request: ${shareId}`)
  // TODO: Replace with real API call
  return true
}

/** Deny a share request */
export async function denyShareRequest(shareId: string): Promise<boolean> {
  console.log(`[Mock] Denying share request: ${shareId}`)
  // TODO: Replace with real API call
  return true
}

/** Revoke an active share */
export async function revokeShare(shareId: string): Promise<boolean> {
  console.log(`[Mock] Revoking share: ${shareId}`)
  // TODO: Replace with real API call
  return true
}

// ============================================================
// Folder Tree (Mock)
// ============================================================

/** Get folder tree structure */
export async function getFolderTree(): Promise<FolderNode[]> {
  // Mock data for development
  return [
    {
      id: 'root-credentials',
      name: '🔑 Credentials',
      type: 'folder',
      children: [
        { id: 'cred-aws', name: 'AWS Keys', type: 'file', data_type: 'credential' },
        { id: 'cred-gcp', name: 'GCP Service Account', type: 'file', data_type: 'credential' },
        { id: 'cred-db', name: 'Database Passwords', type: 'file', data_type: 'credential' },
      ],
    },
    {
      id: 'root-configs',
      name: '⚙️ Configurations',
      type: 'folder',
      children: [
        { id: 'cfg-server', name: 'Server Config', type: 'file', data_type: 'config' },
        { id: 'cfg-docker', name: 'Docker Compose', type: 'file', data_type: 'config' },
        { id: 'cfg-nginx', name: 'Nginx Rules', type: 'file', data_type: 'config' },
      ],
    },
    {
      id: 'root-contacts',
      name: '👤 Contacts',
      type: 'folder',
      children: [
        { id: 'cnt-team', name: 'Team Members', type: 'file', data_type: 'contact' },
        { id: 'cnt-vendors', name: 'Vendors', type: 'file', data_type: 'contact' },
      ],
    },
    {
      id: 'root-files',
      name: '📁 Files',
      type: 'folder',
      children: [
        { id: 'file-notes', name: 'Project Notes', type: 'file', data_type: 'file' },
        { id: 'file-docs', name: 'Documentation', type: 'file', data_type: 'file' },
      ],
    },
  ]
}
