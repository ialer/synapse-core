// Token management for authentication
// Stores token in localStorage for persistence across sessions

const TOKEN_KEY = 'synapse_auth_token'

/** Get stored auth token */
export function getToken(): string | null {
  try {
    return localStorage.getItem(TOKEN_KEY)
  } catch {
    return null
  }
}

/** Store auth token */
export function setToken(token: string): void {
  try {
    localStorage.setItem(TOKEN_KEY, token)
  } catch {
    // localStorage unavailable (SSR, private browsing, etc.)
  }
}

/** Clear auth token */
export function clearToken(): void {
  try {
    localStorage.removeItem(TOKEN_KEY)
  } catch {
    // ignore
  }
}

/** Check if user is authenticated */
export function isAuthenticated(): boolean {
  return getToken() !== null
}
