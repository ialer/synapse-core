<template>
  <div class="dashboard-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">📊 Dashboard</h1>
        <p class="page-subtitle">SynapseCore Data Hub</p>
      </div>
      <div class="header-actions">
        <span :class="['health-indicator', `health-${healthStatus.status}`]">
          <span class="health-dot"></span>
          {{ healthStatus.status === 'healthy' ? 'System Healthy' : healthStatus.status === 'degraded' ? 'Degraded' : 'Down' }}
        </span>
      </div>
    </div>

    <StatusMessage
      :show="!!errorMsg"
      type="error"
      :message="errorMsg"
      @dismiss="errorMsg = ''"
    />

    <!-- Stats Cards -->
    <div class="stats-grid">
      <StatsCard icon="💾" :value="stats.data_count" label="Data Items" />
      <StatsCard icon="📇" :value="stats.index_count" label="Indexed" />
      <StatsCard icon="💬" :value="stats.message_count" label="Messages" />
      <StatsCard icon="🔗" :value="pendingShareCount" label="Pending Shares" />
    </div>

    <!-- Quick Actions -->
    <div class="card" style="margin-top: 24px">
      <h3 class="card-title">🚀 Quick Actions</h3>
      <div class="quick-actions">
        <router-link to="/data/new" class="quick-action">
          <span class="quick-action-icon">➕</span>
          <span class="quick-action-text">Store Data</span>
        </router-link>
        <router-link to="/data" class="quick-action">
          <span class="quick-action-icon">🔍</span>
          <span class="quick-action-text">Search</span>
        </router-link>
        <router-link to="/shares" class="quick-action">
          <span class="quick-action-icon">🔗</span>
          <span class="quick-action-text">Share</span>
        </router-link>
        <router-link to="/messages" class="quick-action">
          <span class="quick-action-icon">💬</span>
          <span class="quick-action-text">Messages</span>
        </router-link>
      </div>
    </div>

    <!-- Two Column Layout: Recent Data + Pending Shares -->
    <div class="dashboard-columns">
      <!-- Recent Data Items -->
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">📋 Recent Data</h3>
          <router-link to="/data" class="btn btn-ghost btn-sm">View All →</router-link>
        </div>
        <div v-if="loading" class="loading-placeholder">
          <LoadingSpinner text="Loading..." />
        </div>
        <div v-else-if="recentItems.length === 0" class="empty-state-mini">
          <span>No data items yet</span>
        </div>
        <div v-else class="recent-list">
          <div
            v-for="item in recentItems"
            :key="item.id"
            class="recent-item"
            @click="$router.push(`/data/${item.id}`)"
          >
            <span class="recent-item-icon">{{ getTypeIcon(item.data_type) }}</span>
            <div class="recent-item-info">
              <span class="recent-item-id">{{ item.id }}</span>
              <span class="recent-item-tags">
                <span v-for="tag in item.tags?.slice(0, 2)" :key="tag" class="tag tag-sm">{{ tag }}</span>
                <span v-if="(item.tags?.length || 0) > 2" class="tag tag-sm">+{{ (item.tags?.length || 0) - 2 }}</span>
              </span>
            </div>
            <span class="recent-item-time">{{ formatDate(item.created_at) }}</span>
          </div>
        </div>
      </div>

      <!-- Pending Share Requests -->
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">🔗 Pending Shares</h3>
          <router-link to="/shares" class="btn btn-ghost btn-sm">Manage →</router-link>
        </div>
        <div v-if="loading" class="loading-placeholder">
          <LoadingSpinner text="Loading..." />
        </div>
        <div v-else-if="pendingShares.length === 0" class="empty-state-mini">
          <span>No pending share requests</span>
        </div>
        <div v-else class="recent-list">
          <div
            v-for="share in pendingShares"
            :key="share.id"
            class="recent-item"
          >
            <span class="recent-item-icon">📨</span>
            <div class="recent-item-info">
              <span class="recent-item-id">{{ share.data_title }}</span>
              <span class="recent-item-meta">From {{ share.sender_name }} · {{ share.permission }}</span>
            </div>
            <div class="recent-item-actions">
              <button class="btn btn-ghost btn-sm" @click="quickDeny(share.id)">✕</button>
              <button class="btn btn-primary btn-sm" @click="quickApprove(share.id)">✓</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- System Info -->
    <div class="card" style="margin-top: 24px">
      <h3 class="card-title">ℹ️ System Info</h3>
      <div class="system-info">
        <div class="info-row">
          <span class="info-label">Storage Backend</span>
          <span class="info-value">{{ healthStatus.storage_backend }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">Version</span>
          <span class="info-value">{{ healthStatus.version }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">Uptime</span>
          <span class="info-value">{{ formatUptime(healthStatus.uptime) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getStats, listData, getShareRequests, approveShareRequest, denyShareRequest, getHealthStatus } from '../api'
import StatsCard from '../components/StatsCard.vue'
import StatusMessage from '../components/StatusMessage.vue'
import LoadingSpinner from '../components/LoadingSpinner.vue'
import type { StatsInfo, DataItemInfo, ShareRequest, HealthStatus } from '../types'

const stats = ref<StatsInfo>({ data_count: 0, index_count: 0, message_count: 0 })
const errorMsg = ref('')
const loading = ref(false)
const recentItems = ref<DataItemInfo[]>([])
const pendingShares = ref<ShareRequest[]>([])
const healthStatus = ref<HealthStatus>({
  status: 'healthy',
  uptime: 0,
  last_check: '',
  storage_backend: 'local',
  version: 'v0.1.0',
})

const pendingShareCount = computed(() => pendingShares.value.length)

const getTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    credential: '🔑',
    config: '⚙️',
    file: '📁',
    contact: '👤',
    generic: '📄',
  }
  return icons[type] || '📄'
}

const formatDate = (dateStr?: string) => {
  if (!dateStr) return ''
  const d = new Date(dateStr)
  const now = new Date()
  const diffMs = now.getTime() - d.getTime()
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffHours / 24)

  if (diffHours < 1) return 'Just now'
  if (diffHours < 24) return `${diffHours}h ago`
  if (diffDays < 7) return `${diffDays}d ago`
  return d.toLocaleDateString()
}

const formatUptime = (ms: number) => {
  if (!ms) return 'N/A'
  const hours = Math.floor(ms / (1000 * 60 * 60))
  const days = Math.floor(hours / 24)
  if (days > 0) return `${days}d ${hours % 24}h`
  return `${hours}h`
}

const quickApprove = async (shareId: string) => {
  try {
    await approveShareRequest(shareId)
    pendingShares.value = pendingShares.value.filter(s => s.id !== shareId)
  } catch (e) {
    errorMsg.value = `Failed to approve: ${e}`
  }
}

const quickDeny = async (shareId: string) => {
  try {
    await denyShareRequest(shareId)
    pendingShares.value = pendingShares.value.filter(s => s.id !== shareId)
  } catch (e) {
    errorMsg.value = `Failed to deny: ${e}`
  }
}

onMounted(async () => {
  loading.value = true
  try {
    const [statsData, healthData, sharesData] = await Promise.allSettled([
      getStats(),
      getHealthStatus(),
      getShareRequests(),
    ])

    if (statsData.status === 'fulfilled') {
      stats.value = statsData.value
      // Get recent items (last 10)
      try {
        const items = await listData('current-user')
        recentItems.value = items.slice(0, 10)
      } catch {
        // listData might fail without auth - that's OK
      }
    }

    if (healthData.status === 'fulfilled') {
      healthStatus.value = healthData.value
    }

    if (sharesData.status === 'fulfilled') {
      pendingShares.value = sharesData.value.filter(s => s.status === 'pending')
    }
  } catch (e) {
    errorMsg.value = `Failed to load dashboard: ${e}`
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.health-indicator {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: 0.8125rem;
  font-weight: 500;
  border-radius: var(--radius-full);
}

.health-healthy {
  background: var(--color-success-bg);
  color: #065f46;
}

.health-degraded {
  background: var(--color-warning-bg);
  color: #92400e;
}

.health-down {
  background: var(--color-error-bg);
  color: #991b1b;
}

.health-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.quick-actions {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.quick-action {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px 12px;
  background: var(--color-bg);
  border-radius: var(--radius-lg);
  text-decoration: none;
  color: var(--color-text);
  transition: all var(--transition-fast);
  min-height: 44px;
}

.quick-action:hover {
  background: rgba(102, 126, 234, 0.08);
  color: var(--color-primary);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.quick-action-icon {
  font-size: 1.75rem;
}

.quick-action-text {
  font-size: 0.875rem;
  font-weight: 500;
}

.dashboard-columns {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--space-md);
  margin-top: 24px;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-md);
}

.card-header .card-title {
  margin-bottom: 0;
}

.recent-list {
  display: flex;
  flex-direction: column;
}

.recent-item {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: 10px 0;
  border-bottom: 1px solid var(--color-border-light);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.recent-item:last-child {
  border-bottom: none;
}

.recent-item:hover {
  background: var(--color-bg-hover);
  margin: 0 calc(-1 * var(--space-lg));
  padding: 10px var(--space-lg);
  border-radius: var(--radius-md);
}

.recent-item-icon {
  font-size: 1.25rem;
  flex-shrink: 0;
}

.recent-item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.recent-item-id {
  font-weight: 500;
  font-size: 0.875rem;
  color: var(--color-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.recent-item-meta {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.recent-item-tags {
  display: flex;
  gap: 4px;
}

.recent-item-time {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  white-space: nowrap;
  flex-shrink: 0;
}

.recent-item-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.tag-sm {
  padding: 1px 6px;
  font-size: 0.6875rem;
}

.loading-placeholder {
  display: flex;
  justify-content: center;
  padding: var(--space-xl);
}

.empty-state-mini {
  text-align: center;
  padding: var(--space-lg);
  color: var(--color-text-muted);
  font-size: 0.875rem;
}

.system-info {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.info-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid var(--color-border-light);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.info-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
  font-family: var(--font-mono);
}

@media (min-width: 768px) {
  .quick-actions {
    grid-template-columns: repeat(4, 1fr);
  }

  .dashboard-columns {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (min-width: 1024px) {
  .dashboard-columns {
    grid-template-columns: 1.2fr 0.8fr;
  }
}
</style>
