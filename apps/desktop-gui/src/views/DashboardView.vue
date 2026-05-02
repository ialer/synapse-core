<template>
  <div class="dashboard-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">📊 仪表盘</h1>
        <p class="page-subtitle">SynapseCore 个人数据管理系统</p>
      </div>
    </div>

    <StatusMessage
      :show="!!errorMsg"
      type="error"
      :message="errorMsg"
      @dismiss="errorMsg = ''"
    />

    <!-- 统计卡片 -->
    <div class="stats-grid">
      <StatsCard icon="💾" :value="stats.data_count" label="数据条目" />
      <StatsCard icon="📇" :value="stats.index_count" label="索引条目" />
      <StatsCard icon="💬" :value="stats.message_count" label="消息数量" />
    </div>

    <!-- 快捷操作 -->
    <div class="card" style="margin-top: 24px">
      <h3 class="card-title">🚀 快捷操作</h3>
      <div class="quick-actions">
        <router-link to="/data/new" class="quick-action">
          <span class="quick-action-icon">➕</span>
          <span class="quick-action-text">新建数据</span>
        </router-link>
        <router-link to="/data" class="quick-action">
          <span class="quick-action-icon">🔍</span>
          <span class="quick-action-text">搜索数据</span>
        </router-link>
        <router-link to="/messages" class="quick-action">
          <span class="quick-action-icon">💬</span>
          <span class="quick-action-text">查看消息</span>
        </router-link>
        <router-link to="/settings" class="quick-action">
          <span class="quick-action-icon">⚙️</span>
          <span class="quick-action-text">系统设置</span>
        </router-link>
      </div>
    </div>

    <!-- 最近活动 -->
    <div class="card" style="margin-top: 24px">
      <h3 class="card-title">📋 系统信息</h3>
      <div class="system-info">
        <div class="info-row">
          <span class="info-label">存储后端</span>
          <span class="info-value">本地存储</span>
        </div>
        <div class="info-row">
          <span class="info-label">数据路径</span>
          <span class="info-value">/tmp/synapse-data</span>
        </div>
        <div class="info-row">
          <span class="info-label">版本</span>
          <span class="info-value">v0.1.0</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getStats } from '../api'
import StatsCard from '../components/StatsCard.vue'
import StatusMessage from '../components/StatusMessage.vue'
import type { StatsInfo } from '../types'

const stats = ref<StatsInfo>({ data_count: 0, index_count: 0, message_count: 0 })
const errorMsg = ref('')

onMounted(async () => {
  try {
    stats.value = await getStats()
  } catch (e) {
    errorMsg.value = `获取统计信息失败: ${e}`
  }
})
</script>

<style scoped>
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
}

.quick-action-icon {
  font-size: 1.75rem;
}

.quick-action-text {
  font-size: 0.875rem;
  font-weight: 500;
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
}
</style>
