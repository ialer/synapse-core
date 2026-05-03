<template>
  <div class="data-detail-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">📄 Data Detail</h1>
        <p class="page-subtitle">View and edit data item</p>
      </div>
      <router-link to="/data" class="btn btn-secondary">
        ← Back to Data
      </router-link>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <LoadingSpinner v-if="loading" text="Loading data..." />

    <template v-else-if="item">
      <div class="card">
        <div class="detail-header">
          <div>
            <span class="tag tag-primary">{{ getTypeLabel(item.data_type) }}</span>
            <span class="detail-id">{{ item.id }}</span>
          </div>
          <div class="detail-actions">
            <button v-if="!editing" class="btn btn-secondary btn-sm" @click="startEdit">
              ✏️ Edit
            </button>
            <button v-if="editing" class="btn btn-primary btn-sm" @click="saveEdit" :disabled="saving">
              {{ saving ? 'Saving...' : '💾 Save' }}
            </button>
            <button v-if="editing" class="btn btn-secondary btn-sm" @click="cancelEdit">
              Cancel
            </button>
            <button class="btn btn-danger btn-sm" @click="confirmDelete">
              🗑️ Delete
            </button>
          </div>
        </div>

        <!-- View Mode -->
        <div v-if="!editing" class="detail-content">
          <div class="detail-section">
            <h4 class="detail-section-title">Content</h4>
            <pre class="detail-pre">{{ item.content }}</pre>
          </div>
          <div v-if="item.tags?.length" class="detail-section">
            <h4 class="detail-section-title">Tags</h4>
            <div class="detail-tags">
              <span v-for="tag in item.tags" :key="tag" class="tag">{{ tag }}</span>
            </div>
          </div>
          <div v-if="item.created_at" class="detail-section">
            <h4 class="detail-section-title">Created</h4>
            <span class="detail-meta">{{ item.created_at }}</span>
          </div>
        </div>

        <!-- Edit Mode -->
        <div v-else class="detail-content">
          <div class="form-group">
            <label class="form-label">Type</label>
            <select v-model="editForm.dataType" class="form-select">
              <option v-for="opt in DATA_TYPE_OPTIONS" :key="opt.value" :value="opt.value">
                {{ opt.icon }} {{ opt.label }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Content</label>
            <textarea
              v-model="editForm.content"
              class="form-textarea"
              rows="8"
            ></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">Tags (comma separated)</label>
            <input
              v-model="editForm.tags"
              class="form-input"
              placeholder="tag1, tag2"
            />
          </div>
        </div>
      </div>
    </template>

    <div v-else class="empty-state">
      <div class="empty-state-icon">❓</div>
      <div class="empty-state-text">Data item not found</div>
      <router-link to="/data" class="btn btn-primary">Back to Data</router-link>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteConfirm" class="modal-overlay" @click.self="showDeleteConfirm = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="modal-title">Confirm Delete</h3>
          <button class="btn btn-ghost btn-sm" @click="showDeleteConfirm = false">✕</button>
        </div>
        <div class="modal-body">
          <p>Are you sure you want to delete this data item? This action cannot be undone.</p>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showDeleteConfirm = false">Cancel</button>
          <button class="btn btn-danger" @click="handleDelete" :disabled="deleting">
            {{ deleting ? 'Deleting...' : 'Confirm Delete' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getData, storeData, deleteData } from '../api'
import LoadingSpinner from '../components/LoadingSpinner.vue'
import StatusMessage from '../components/StatusMessage.vue'
import { DATA_TYPE_OPTIONS } from '../types'
import type { DataType } from '../types'

const route = useRoute()
const router = useRouter()

const loading = ref(true)
const item = ref<{ id: string; data_type: DataType; content: string; tags: string[]; created_at?: string } | null>(null)
const editing = ref(false)
const saving = ref(false)
const deleting = ref(false)
const showDeleteConfirm = ref(false)
const statusMsg = ref('')
const statusType = ref<'success' | 'error'>('success')

const editForm = ref({
  dataType: 'generic' as DataType,
  content: '',
  tags: '',
})

const dataId = computed(() => route.params.id as string)

const getTypeLabel = (type: DataType) => {
  const labels: Record<DataType, string> = {
    credential: 'Credential', config: 'Config', file: 'File', contact: 'Contact', generic: 'Generic',
  }
  return labels[type] || 'Unknown'
}

const loadItem = async () => {
  loading.value = true
  try {
    const content = await getData('demo-token', dataId.value)
    item.value = {
      id: dataId.value,
      data_type: 'generic',
      content,
      tags: [],
    }
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to load: ${e}`
  } finally {
    loading.value = false
  }
}

const startEdit = () => {
  if (!item.value) return
  editForm.value = {
    dataType: item.value.data_type,
    content: item.value.content,
    tags: item.value.tags.join(', '),
  }
  editing.value = true
}

const cancelEdit = () => {
  editing.value = false
}

const saveEdit = async () => {
  if (!item.value) return
  saving.value = true

  try {
    const tags = editForm.value.tags
      .split(',')
      .map((t) => t.trim())
      .filter(Boolean)

    await storeData('demo-token', editForm.value.dataType, editForm.value.content, tags)

    item.value.data_type = editForm.value.dataType
    item.value.content = editForm.value.content
    item.value.tags = tags

    editing.value = false
    statusType.value = 'success'
    statusMsg.value = 'Data updated successfully'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Update failed: ${e}`
  } finally {
    saving.value = false
  }
}

const confirmDelete = () => {
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  deleting.value = true
  try {
    await deleteData('demo-token', dataId.value)
    statusType.value = 'success'
    statusMsg.value = 'Data deleted successfully'
    showDeleteConfirm.value = false
    setTimeout(() => router.push('/data'), 1000)
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Delete failed: ${e}`
  } finally {
    deleting.value = false
  }
}

onMounted(loadItem)
</script>

<style scoped>
.detail-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 20px;
}

.detail-id {
  margin-left: 8px;
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  color: var(--color-text-muted);
}

.detail-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.detail-content {
  margin-top: 8px;
}

.detail-section {
  margin-bottom: 20px;
}

.detail-section-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.detail-pre {
  background: var(--color-bg);
  padding: 16px;
  border-radius: var(--radius-md);
  font-family: var(--font-mono);
  font-size: 0.875rem;
  line-height: 1.6;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
}

.detail-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.detail-meta {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}
</style>
