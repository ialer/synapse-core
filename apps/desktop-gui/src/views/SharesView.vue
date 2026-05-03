<template>
  <div class="shares-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">🔗 Shares</h1>
        <p class="page-subtitle">Manage data sharing and access permissions</p>
      </div>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <!-- Tab Navigation -->
    <div class="share-tabs">
      <button
        :class="['share-tab', { active: activeTab === 'incoming' }]"
        @click="activeTab = 'incoming'"
      >
        📥 Incoming
        <span v-if="incomingPending.length" class="share-tab-badge">{{ incomingPending.length }}</span>
      </button>
      <button
        :class="['share-tab', { active: activeTab === 'outgoing' }]"
        @click="activeTab = 'outgoing'"
      >
        📤 Outgoing
        <span v-if="outgoingPending.length" class="share-tab-badge">{{ outgoingPending.length }}</span>
      </button>
      <button
        :class="['share-tab', { active: activeTab === 'active' }]"
        @click="activeTab = 'active'"
      >
        ✅ Active
        <span v-if="activeShares.length" class="share-tab-badge">{{ activeShares.length }}</span>
      </button>
    </div>

    <!-- Loading -->
    <LoadingSpinner v-if="loading" text="Loading shares..." />

    <!-- Empty State -->
    <div v-else-if="currentShares.length === 0" class="empty-state">
      <div class="empty-state-icon">📭</div>
      <div class="empty-state-text">{{ emptyMessage }}</div>
    </div>

    <!-- Share Request Cards -->
    <div v-else class="share-list">
      <div
        v-for="share in currentShares"
        :key="share.id"
        class="share-card"
      >
        <div class="share-card-header">
          <div class="share-card-info">
            <span class="share-card-icon">📄</span>
            <div>
              <div class="share-card-title">{{ share.data_title }}</div>
              <div class="share-card-meta">
                <span v-if="activeTab === 'incoming'">
                  From: <strong>{{ share.sender_name }}</strong>
                </span>
                <span v-else>
                  To: <strong>{{ share.recipient_name }}</strong>
                </span>
                · {{ formatDate(share.created_at) }}
              </div>
            </div>
          </div>
          <span :class="['share-status-badge', `status-${share.status}`]">
            {{ statusLabel(share.status) }}
          </span>
        </div>

        <div v-if="share.message" class="share-card-message">
          "{{ share.message }}"
        </div>

        <div class="share-card-footer">
          <span class="share-permission">
            <span class="share-permission-icon">{{ share.permission === 'write' ? '✏️' : '👁️' }}</span>
            {{ share.permission === 'write' ? 'Read & Write' : 'Read Only' }}
          </span>

          <div class="share-card-actions">
            <!-- Incoming pending: approve/deny -->
            <template v-if="activeTab === 'incoming' && share.status === 'pending'">
              <button class="btn btn-ghost btn-sm" @click="handleDeny(share.id)">
                ✕ Deny
              </button>
              <button class="btn btn-primary btn-sm" @click="handleApprove(share.id)">
                ✓ Approve
              </button>
            </template>

            <!-- Active: revoke -->
            <template v-if="activeTab === 'active'">
              <button class="btn btn-danger btn-sm" @click="handleRevoke(share.id)">
                Revoke Access
              </button>
            </template>

            <!-- Read-only for others -->
            <template v-if="activeTab === 'outgoing'">
              <span v-if="share.status === 'pending'" class="share-waiting">⏳ Awaiting response</span>
              <span v-if="share.status === 'denied'" class="share-denied-label">Request denied</span>
              <span v-if="share.status === 'revoked'" class="share-revoked-label">Access revoked</span>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getShareRequests, approveShareRequest, denyShareRequest, revokeShare } from '../api'
import StatusMessage from '../components/StatusMessage.vue'
import LoadingSpinner from '../components/LoadingSpinner.vue'
import type { ShareRequest } from '../types'

const loading = ref(false)
const shares = ref<ShareRequest[]>([])
const activeTab = ref<'incoming' | 'outgoing' | 'active'>('incoming')
const statusMsg = ref('')
const statusType = ref<'success' | 'error' | 'info'>('info')

const incomingPending = computed(() =>
  shares.value.filter(s => s.status === 'pending' && s.recipient_name === 'Alice')
)

const outgoingPending = computed(() =>
  shares.value.filter(s => s.sender_name === 'Alice' && ['pending', 'denied', 'revoked'].includes(s.status))
)

const activeShares = computed(() =>
  shares.value.filter(s => s.status === 'approved')
)

const currentShares = computed(() => {
  switch (activeTab.value) {
    case 'incoming': return incomingPending.value
    case 'outgoing': return outgoingPending.value
    case 'active': return activeShares.value
    default: return []
  }
})

const emptyMessage = computed(() => {
  switch (activeTab.value) {
    case 'incoming': return 'No pending share requests'
    case 'outgoing': return 'No outgoing share requests'
    case 'active': return 'No active shares'
    default: return 'No shares'
  }
})

const statusLabel = (status: string) => {
  const labels: Record<string, string> = {
    pending: 'Pending',
    approved: 'Approved',
    denied: 'Denied',
    revoked: 'Revoked',
  }
  return labels[status] || status
}

const formatDate = (dateStr: string) => {
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

const handleApprove = async (shareId: string) => {
  try {
    await approveShareRequest(shareId)
    const share = shares.value.find(s => s.id === shareId)
    if (share) share.status = 'approved'
    statusType.value = 'success'
    statusMsg.value = 'Share request approved'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to approve: ${e}`
  }
}

const handleDeny = async (shareId: string) => {
  try {
    await denyShareRequest(shareId)
    const share = shares.value.find(s => s.id === shareId)
    if (share) share.status = 'denied'
    statusType.value = 'info'
    statusMsg.value = 'Share request denied'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to deny: ${e}`
  }
}

const handleRevoke = async (shareId: string) => {
  try {
    await revokeShare(shareId)
    const share = shares.value.find(s => s.id === shareId)
    if (share) share.status = 'revoked'
    statusType.value = 'info'
    statusMsg.value = 'Share access revoked'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to revoke: ${e}`
  }
}

onMounted(async () => {
  loading.value = true
  try {
    shares.value = await getShareRequests()
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to load shares: ${e}`
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.share-tabs {
  display: flex;
  gap: 4px;
  margin-bottom: var(--space-lg);
  background: var(--color-bg);
  padding: 4px;
  border-radius: var(--radius-lg);
}

.share-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 16px;
  font-size: 0.875rem;
  font-weight: 500;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.share-tab:hover {
  color: var(--color-text);
  background: var(--color-bg-hover);
}

.share-tab.active {
  background: var(--color-bg-card);
  color: var(--color-primary);
  box-shadow: var(--shadow-sm);
}

.share-tab-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 20px;
  padding: 0 6px;
  font-size: 0.75rem;
  font-weight: 600;
  background: var(--color-primary);
  color: white;
  border-radius: var(--radius-full);
}

.share-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.share-card {
  background: var(--color-bg-card);
  border-radius: var(--radius-lg);
  padding: var(--space-lg);
  border: 1px solid var(--color-border-light);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-fast);
}

.share-card:hover {
  box-shadow: var(--shadow-md);
}

.share-card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-md);
  margin-bottom: var(--space-md);
}

.share-card-info {
  display: flex;
  align-items: flex-start;
  gap: var(--space-md);
}

.share-card-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.share-card-title {
  font-weight: 600;
  font-size: 1rem;
  color: var(--color-text);
  margin-bottom: 2px;
}

.share-card-meta {
  font-size: 0.8125rem;
  color: var(--color-text-secondary);
}

.share-status-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  font-size: 0.75rem;
  font-weight: 600;
  border-radius: var(--radius-full);
  white-space: nowrap;
  flex-shrink: 0;
}

.status-pending {
  background: var(--color-warning-bg);
  color: #92400e;
}

.status-approved {
  background: var(--color-success-bg);
  color: #065f46;
}

.status-denied {
  background: var(--color-error-bg);
  color: #991b1b;
}

.status-revoked {
  background: var(--color-bg);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.share-card-message {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  font-style: italic;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-bg);
  border-radius: var(--radius-md);
  margin-bottom: var(--space-md);
  border-left: 3px solid var(--color-primary-light);
}

.share-card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-md);
  padding-top: var(--space-md);
  border-top: 1px solid var(--color-border-light);
}

.share-permission {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.8125rem;
  color: var(--color-text-secondary);
}

.share-permission-icon {
  font-size: 1rem;
}

.share-card-actions {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.share-waiting {
  font-size: 0.8125rem;
  color: var(--color-warning);
  font-weight: 500;
}

.share-denied-label {
  font-size: 0.8125rem;
  color: var(--color-error);
  font-weight: 500;
}

.share-revoked-label {
  font-size: 0.8125rem;
  color: var(--color-text-muted);
  font-weight: 500;
}

@media (min-width: 768px) {
  .share-tabs {
    width: fit-content;
  }

  .share-tab {
    flex: none;
    min-width: 140px;
  }
}
</style>
