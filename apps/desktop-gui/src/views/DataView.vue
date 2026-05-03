<template>
  <div class="data-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">💾 Data Hub</h1>
        <p class="page-subtitle">Browse and manage all your data</p>
      </div>
      <router-link to="/data/new" class="btn btn-primary">
        ➕ New Data
      </router-link>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <div class="data-layout">
      <!-- Folder Tree Sidebar -->
      <aside class="folder-sidebar">
        <div class="folder-sidebar-header">
          <h4>Folders</h4>
        </div>
        <div class="folder-tree">
          <div
            v-for="folder in folderTree"
            :key="folder.id"
            class="folder-node"
          >
            <div
              :class="['folder-item', { active: selectedFolder === folder.id }]"
              @click="toggleFolder(folder.id)"
            >
              <span class="folder-toggle">{{ expandedFolders.has(folder.id) ? '▾' : '▸' }}</span>
              <span class="folder-icon">{{ folder.name.split(' ')[0] }}</span>
              <span class="folder-name">{{ folder.name.replace(/^[^\s]+\s/, '') }}</span>
              <span class="folder-count">{{ folder.children?.length || 0 }}</span>
            </div>
            <div v-if="expandedFolders.has(folder.id) && folder.children" class="folder-children">
              <div
                v-for="child in folder.children"
                :key="child.id"
                :class="['folder-child', { active: selectedFolder === child.id }]"
                @click="selectFolder(child.id)"
              >
                <span class="folder-child-icon">📄</span>
                <span class="folder-child-name">{{ child.name }}</span>
              </div>
            </div>
          </div>
        </div>
      </aside>

      <!-- Main Content -->
      <div class="data-main">
        <!-- Search and Filter Toolbar -->
        <div class="card" style="margin-bottom: 16px">
          <div class="toolbar">
            <SearchBar
              v-model="searchQuery"
              placeholder="Search data..."
              @search="handleSearch"
            />
            <div class="toolbar-actions">
              <select v-model="filterType" class="form-select toolbar-select">
                <option value="">All Types</option>
                <option value="credential">🔑 Credentials</option>
                <option value="config">⚙️ Config</option>
                <option value="file">📁 Files</option>
                <option value="contact">👤 Contacts</option>
                <option value="generic">📄 Generic</option>
              </select>

              <select v-model="sortBy" class="form-select toolbar-select">
                <option value="date-desc">Newest First</option>
                <option value="date-asc">Oldest First</option>
                <option value="name-asc">Name A-Z</option>
                <option value="name-desc">Name Z-A</option>
                <option value="type">By Type</option>
              </select>

              <div class="view-toggle">
                <button
                  :class="['btn btn-ghost btn-icon', { active: viewMode === 'grid' }]"
                  @click="viewMode = 'grid'"
                  title="Grid View"
                >▦</button>
                <button
                  :class="['btn btn-ghost btn-icon', { active: viewMode === 'table' }]"
                  @click="viewMode = 'table'"
                  title="List View"
                >☰</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Tag Filter Chips -->
        <div v-if="availableTags.length > 0" class="tag-filter-bar">
          <span class="tag-filter-label">Filter by tag:</span>
          <div class="tag-chips">
            <button
              v-for="tag in availableTags"
              :key="tag"
              :class="['tag-chip', { active: selectedTags.has(tag) }]"
              @click="toggleTag(tag)"
            >
              {{ tag }}
              <span v-if="selectedTags.has(tag)" class="tag-chip-x">✕</span>
            </button>
          </div>
          <button
            v-if="selectedTags.size > 0"
            class="btn btn-ghost btn-sm"
            @click="clearTagFilters"
          >
            Clear All
          </button>
        </div>

        <!-- Search Results Info -->
        <div v-if="isSearchMode" class="search-info">
          <span>Search "{{ lastQuery }}" results ({{ filteredItems.length }} items)</span>
          <button class="btn btn-ghost btn-sm" @click="clearSearch">Clear Search</button>
        </div>

        <!-- Results Count -->
        <div class="results-count" v-if="!isSearchMode && filteredItems.length > 0">
          {{ filteredItems.length }} items
          <span v-if="selectedTags.size > 0"> · {{ selectedTags.size }} tags selected</span>
        </div>

        <!-- Data List -->
        <DataList
          :items="filteredItems"
          :loading="loading"
          :view-mode="viewMode"
          empty-text="No data found. Click the button above to create some."
          @select="goToDetail"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { searchData, getFolderTree } from '../api'
import SearchBar from '../components/SearchBar.vue'
import DataList from '../components/DataList.vue'
import StatusMessage from '../components/StatusMessage.vue'
import type { DataItem, DataType, FolderNode } from '../types'

const router = useRouter()

const searchQuery = ref('')
const filterType = ref('')
const sortBy = ref('date-desc')
const viewMode = ref<'grid' | 'table'>('grid')
const loading = ref(false)
const items = ref<DataItem[]>([])
const statusMsg = ref('')
const statusType = ref<'success' | 'error' | 'info'>('info')
const isSearchMode = ref(false)
const lastQuery = ref('')
const selectedTags = ref(new Set<string>())
const folderTree = ref<FolderNode[]>([])
const expandedFolders = ref(new Set<string>())
const selectedFolder = ref('')

const availableTags = computed(() => {
  const tags = new Set<string>()
  items.value.forEach(item => {
    item.tags?.forEach(tag => tags.add(tag))
  })
  return Array.from(tags).sort()
})

const filteredItems = computed(() => {
  let result = items.value

  // Filter by type
  if (filterType.value) {
    result = result.filter(item => item.data_type === filterType.value)
  }

  // Filter by tags
  if (selectedTags.value.size > 0) {
    result = result.filter(item =>
      item.tags?.some(tag => selectedTags.value.has(tag))
    )
  }

  // Sort
  result = [...result].sort((a, b) => {
    switch (sortBy.value) {
      case 'date-desc':
        return (b.created_at || '').localeCompare(a.created_at || '')
      case 'date-asc':
        return (a.created_at || '').localeCompare(b.created_at || '')
      case 'name-asc':
        return a.id.localeCompare(b.id)
      case 'name-desc':
        return b.id.localeCompare(a.id)
      case 'type':
        return a.data_type.localeCompare(b.data_type)
      default:
        return 0
    }
  })

  return result
})

const toggleTag = (tag: string) => {
  if (selectedTags.value.has(tag)) {
    selectedTags.value.delete(tag)
  } else {
    selectedTags.value.add(tag)
  }
  // Force reactivity
  selectedTags.value = new Set(selectedTags.value)
}

const clearTagFilters = () => {
  selectedTags.value = new Set()
}

const toggleFolder = (folderId: string) => {
  if (expandedFolders.value.has(folderId)) {
    expandedFolders.value.delete(folderId)
  } else {
    expandedFolders.value.add(folderId)
  }
  expandedFolders.value = new Set(expandedFolders.value)
}

const selectFolder = (folderId: string) => {
  selectedFolder.value = selectedFolder.value === folderId ? '' : folderId
}

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
    statusMsg.value = `Found ${items.value.length} results`
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Search failed: ${e}`
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

onMounted(async () => {
  // Load folder tree
  try {
    folderTree.value = await getFolderTree()
  } catch {
    // Mock data will be used
  }
})
</script>

<style scoped>
.data-layout {
  display: flex;
  gap: var(--space-md);
  align-items: flex-start;
}

/* Folder Sidebar */
.folder-sidebar {
  display: none;
  width: 240px;
  flex-shrink: 0;
  background: var(--color-bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border-light);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
}

.folder-sidebar-header {
  padding: var(--space-md) var(--space-lg);
  border-bottom: 1px solid var(--color-border-light);
}

.folder-sidebar-header h4 {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.folder-tree {
  padding: var(--space-sm);
}

.folder-node {
  margin-bottom: 2px;
}

.folder-item {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: 8px var(--space-sm);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition-fast);
  font-size: 0.875rem;
}

.folder-item:hover {
  background: var(--color-bg-hover);
}

.folder-item.active {
  background: rgba(102, 126, 234, 0.08);
  color: var(--color-primary);
}

.folder-toggle {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  width: 12px;
}

.folder-icon {
  font-size: 1rem;
}

.folder-name {
  flex: 1;
  font-weight: 500;
}

.folder-count {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  background: var(--color-bg);
  padding: 1px 6px;
  border-radius: var(--radius-full);
}

.folder-children {
  padding-left: 28px;
}

.folder-child {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: 6px var(--space-sm);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition-fast);
  font-size: 0.8125rem;
  color: var(--color-text-secondary);
}

.folder-child:hover {
  background: var(--color-bg-hover);
  color: var(--color-text);
}

.folder-child.active {
  background: rgba(102, 126, 234, 0.08);
  color: var(--color-primary);
}

.folder-child-icon {
  font-size: 0.875rem;
}

.folder-child-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Main Content */
.data-main {
  flex: 1;
  min-width: 0;
}

/* Toolbar */
.toolbar {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
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

/* Tag Filter Bar */
.tag-filter-bar {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) 0;
  margin-bottom: var(--space-md);
  flex-wrap: wrap;
}

.tag-filter-label {
  font-size: 0.8125rem;
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.tag-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tag-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  font-size: 0.75rem;
  font-weight: 500;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-full);
  background: var(--color-bg-card);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tag-chip:hover {
  border-color: var(--color-primary-light);
  color: var(--color-primary);
}

.tag-chip.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.tag-chip-x {
  font-size: 0.625rem;
  margin-left: 2px;
}

/* Search Info */
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

/* Results Count */
.results-count {
  font-size: 0.8125rem;
  color: var(--color-text-muted);
  margin-bottom: var(--space-md);
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

@media (min-width: 1024px) {
  .folder-sidebar {
    display: block;
  }

  .data-layout {
    gap: var(--space-lg);
  }
}
</style>
