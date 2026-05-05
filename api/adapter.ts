// Dual-mode API adapter
// Works in both Tauri (desktop/mobile) and browser (web) modes
// Uses HTTP fetch for both - the web server is the single backend

import { getToken } from './auth'

// Detect if running inside Tauri
export function isTauri(): boolean {
  return '__TAURI__' in window || navigator.userAgent.includes('Tauri')
}

// Base URL for API requests
// - Tauri dev mode (Vite dev server on 5173): http://localhost:8080
// - Browser dev mode (Vite dev server on 5173): http://localhost:8080
// - Production (served by web server on 8080): '' (same origin)
const API_BASE: string = (import.meta as any).env?.VITE_API_BASE || ''

/** HTTP error with status code */
export class ApiError extends Error {
  status: number
  constructor(message: string, status: number) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

/** Generic HTTP request function */
export async function request<T = any>(
  method: string,
  path: string,
  body?: Record<string, any>
): Promise<T> {
  const url = `${API_BASE}${path}`
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  }

  // Auto-include auth token if available
  const token = getToken()
  if (token) {
    headers['Authorization'] = `Bearer ${token}`
  }

  const options: RequestInit = { method, headers }
  if (body !== undefined) {
    options.body = JSON.stringify(body)
  }

  const resp = await fetch(url, options)

  if (!resp.ok) {
    const err = await resp.json().catch(() => ({ error: resp.statusText }))
    throw new ApiError(err.error || `Request failed (${resp.status})`, resp.status)
  }

  // Handle 204 No Content
  if (resp.status === 204) {
    return undefined as T
  }

  return resp.json()
}
