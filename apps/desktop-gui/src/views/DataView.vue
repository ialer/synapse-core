<template>
  <div class="data-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">💾 数据管理</h1>
        <p class="page-subtitle">浏览和搜索所有数据</p>
      </div>
      <router-link to="/data/new" class="btn btn-primary">
        ➕ 新建数据
      </router-link>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <!-- 搜索和筛选 -->
    <div class="card" style="margin-bottom: 16px">
      <div class="toolbar">
        <SearchBar
          v-model="searchQuery"
          placeholder="搜索数据..."
          @search="handleSearch"
        />
        <div class="toolbar-actions">
          <select v-model="filterType" class="form-select toolbar-select">
            <option value="">全部类型</option>
            <option value="credential">🔑 凭证</option>
            <option value="config">⚙️ 配置</option>
            <option value="file">📁 文件</option>
            <option value="contact">👤 联系人</option>
            <option value="generic">📄 通用</option>
          </select>
          <div class="view-toggle">
            <button
              :class="['btn btn-ghost btn-icon', { active: viewMode === 'grid' }]"
              @click="viewMode = 'grid'"
              title="网格视图"
            >▦</button>
            <button
              :class="['btn btn-ghost btn-icon', { active: viewMode === 'table' }]"
              @click="viewMode = 'table'"
              title="列表视图"
            >☰</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索结果提示 -->
    <div v-if="isSearchMode" class="search-info">
      <span>搜索 "{{ lastQuery }}" 的结果 ({{ filteredItems.length }} 条)</span>
      <button class="btn btn-ghost btn-sm" @click="clearSearch">清除搜索</button>
    </div>

    <!-- 数据列表 -->
    <DataList
      :items="filteredItems"
      :loading="loading"
      :view-mode="viewMode"
      empty-text="暂无数据，点击上方按钮创建"
      @select="goToDetail"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { searchData } from '../api'
import SearchBar from '../components/SearchBar.vue'
import DataList from '../components/DataList.vue'
import StatusMessage from '../components/StatusMessage.vue'
import type { DataItem, DataType } from '../types'

const router = useRouter()

const searchQuery = ref('')
const filterType = ref('')
const viewMode = ref<'grid' | 'table'>('grid')
const loading = ref(false)
const items = ref<DataItem[]>([])
const statusMsg = ref('')
const statusType = ref<'success' | 'error' | 'info'>('info')
const isSearchMode = ref(false)
const lastQuery = ref('')

const filteredItems = computed(() => {
  if (!filterType.value) return items.value
  return items.value.filter((item) => item.data_type === filterType.value)
})

const handleSearch = async () => {
  const query = searchQuery.value.trim()
  if (!query) {
    clearSearch()
    return
  }

  loading.value = true
  lastQuery.value = query

  try {
    const results = await searchData(query, 50)
    items.value = results.map((r) => ({
      id: r.id,
      data_type: (r.metadata?.type as DataType) || 'generic',
      content: r.content,
      tags: r.metadata?.tags?.split(',') || [],
      created_at: r.metadata?.timestamp,
    }))
    isSearchMode.value = true
    statusType.value = 'info'
    statusMsg.value = `找到 ${items.value.length} 条结果`
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `搜索失败: ${e}`
  } finally {
    loading.value = false
  }
}

const clearSearch = () => {
  searchQuery.value = ''
  filterType.value = ''
  items.value = []
  isSearchMode.value = false
  lastQuery.value = ''
}

const goToDetail = (item: DataItem) => {
  router.push(`/data/${item.id}`)
}

onMounted(() => {
  // 初始不加载数据，等待用户搜索
})
</script>

<style scoped>
.toolbar {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-select {
  min-width: 120px;
}

.view-toggle {
  display: flex;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.view-toggle .btn {
  border-radius: 0;
  min-height: 36px;
  padding: 6px 10px;
}

.view-toggle .btn.active {
  background: var(--color-primary);
  color: white;
}

.search-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
  margin-bottom: 16px;
  background: var(--color-info-bg);
  border-radius: var(--radius-md);
  font-size: 0.875rem;
  color: var(--color-info);
}

@media (min-width: 768px) {
  .toolbar {
    flex-direction: row;
    align-items: center;
  }

  .toolbar-select {
    min-width: 140px;
  }
}
</style>
