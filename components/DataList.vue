<template>
  <div class="data-list">
    <!-- 空状态 -->
    <div v-if="!loading && items.length === 0" class="empty-state">
      <div class="empty-state-icon">📭</div>
      <div class="empty-state-text">{{ emptyText }}</div>
    </div>

    <!-- 加载状态 -->
    <LoadingSpinner v-if="loading" text="加载中..." />

    <!-- 网格视图 -->
    <div v-else-if="viewMode === 'grid'" class="data-grid">
      <div
        v-for="item in items"
        :key="item.id"
        class="data-grid-item"
        @click="$emit('select', item)"
      >
        <div class="data-grid-item-header">
          <span class="data-grid-item-title">{{ getTypeIcon(item.data_type) }} {{ item.id }}</span>
          <span class="tag tag-primary">{{ getTypeLabel(item.data_type) }}</span>
        </div>
        <div class="data-grid-item-content">{{ item.content }}</div>
        <div class="data-grid-item-tags" v-if="item.tags?.length">
          <span v-for="tag in item.tags" :key="tag" class="tag">{{ tag }}</span>
        </div>
        <div v-if="item.created_at" class="data-grid-item-time">{{ item.created_at }}</div>
      </div>
    </div>

    <!-- 表格视图 -->
    <div v-else class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>类型</th>
            <th>内容</th>
            <th>标签</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in items"
            :key="item.id"
            class="data-table-row"
            @click="$emit('select', item)"
          >
            <td class="data-table-id">{{ item.id }}</td>
            <td>
              <span class="tag tag-primary">{{ getTypeLabel(item.data_type) }}</span>
            </td>
            <td class="data-table-content">{{ item.content }}</td>
            <td>
              <div class="data-table-tags">
                <span v-for="tag in item.tags?.slice(0, 3)" :key="tag" class="tag">{{ tag }}</span>
                <span v-if="(item.tags?.length || 0) > 3" class="tag">+{{ (item.tags?.length || 0) - 3 }}</span>
              </div>
            </td>
            <td>
              <button class="btn btn-ghost btn-sm" @click.stop="$emit('select', item)">查看</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DataItem, DataType } from '../types'
import LoadingSpinner from './LoadingSpinner.vue'

withDefaults(
  defineProps<{
    items: DataItem[]
    loading?: boolean
    viewMode?: 'grid' | 'table'
    emptyText?: string
  }>(),
  {
    loading: false,
    viewMode: 'grid',
    emptyText: '暂无数据',
  }
)

defineEmits<{
  select: [item: DataItem]
}>()

const typeIcons: Record<DataType, string> = {
  credential: '🔑',
  config: '⚙️',
  file: '📁',
  contact: '👤',
  generic: '📄',
}

const typeLabels: Record<DataType, string> = {
  credential: '凭证',
  config: '配置',
  file: '文件',
  contact: '联系人',
  generic: '通用',
}

const getTypeIcon = (type: DataType) => typeIcons[type] || '📄'
const getTypeLabel = (type: DataType) => typeLabels[type] || '未知'
</script>

<style scoped>
.table-wrapper {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

.data-table-row {
  cursor: pointer;
  transition: background 0.15s ease;
}

.data-table-row:hover {
  background: var(--color-bg-hover);
}

.data-table-id {
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.data-table-content {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.data-table-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
</style>
