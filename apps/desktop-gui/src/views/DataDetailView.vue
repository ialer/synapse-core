<template>
  <div class="data-detail-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">📄 数据详情</h1>
        <p class="page-subtitle">查看和编辑数据条目</p>
      </div>
      <router-link to="/data" class="btn btn-secondary">
        ← 返回列表
      </router-link>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <LoadingSpinner v-if="loading" text="加载数据..." />

    <template v-else-if="item">
      <div class="card">
        <div class="detail-header">
          <div>
            <span class="tag tag-primary">{{ getTypeLabel(item.data_type) }}</span>
            <span class="detail-id">{{ item.id }}</span>
          </div>
          <div class="detail-actions">
            <button v-if="!editing" class="btn btn-secondary btn-sm" @click="startEdit">
              ✏️ 编辑
            </button>
            <button v-if="editing" class="btn btn-primary btn-sm" @click="saveEdit" :disabled="saving">
              {{ saving ? '保存中...' : '💾 保存' }}
            </button>
            <button v-if="editing" class="btn btn-secondary btn-sm" @click="cancelEdit">
              取消
            </button>
            <button class="btn btn-danger btn-sm" @click="confirmDelete">
              🗑️ 删除
            </button>
          </div>
        </div>

        <!-- 查看模式 -->
        <div v-if="!editing" class="detail-content">
          <div class="detail-section">
            <h4 class="detail-section-title">内容</h4>
            <pre class="detail-pre">{{ item.content }}</pre>
          </div>
          <div v-if="item.tags?.length" class="detail-section">
            <h4 class="detail-section-title">标签</h4>
            <div class="detail-tags">
              <span v-for="tag in item.tags" :key="tag" class="tag">{{ tag }}</span>
            </div>
          </div>
          <div v-if="item.created_at" class="detail-section">
            <h4 class="detail-section-title">创建时间</h4>
            <span class="detail-meta">{{ item.created_at }}</span>
          </div>
        </div>

        <!-- 编辑模式 -->
        <div v-else class="detail-content">
          <div class="form-group">
            <label class="form-label">类型</label>
            <select v-model="editForm.dataType" class="form-select">
              <option v-for="opt in DATA_TYPE_OPTIONS" :key="opt.value" :value="opt.value">
                {{ opt.icon }} {{ opt.label }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">内容</label>
            <textarea
              v-model="editForm.content"
              class="form-textarea"
              rows="8"
            ></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">标签 (逗号分隔)</label>
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
      <div class="empty-state-text">数据不存在</div>
      <router-link to="/data" class="btn btn-primary">返回列表</router-link>
    </div>

    <!-- 删除确认对话框 -->
    <div v-if="showDeleteConfirm" class="modal-overlay" @click.self="showDeleteConfirm = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="modal-title">确认删除</h3>
          <button class="btn btn-ghost btn-sm" @click="showDeleteConfirm = false">✕</button>
        </div>
        <div class="modal-body">
          <p>确定要删除这条数据吗？此操作不可撤销。</p>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showDeleteConfirm = false">取消</button>
          <button class="btn btn-danger" @click="handleDelete" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
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
    credential: '凭证', config: '配置', file: '文件', contact: '联系人', generic: '通用',
  }
  return labels[type] || '未知'
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
    statusMsg.value = `加载失败: ${e}`
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
    statusMsg.value = '数据已更新'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `更新失败: ${e}`
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
    statusMsg.value = '数据已删除'
    showDeleteConfirm.value = false
    setTimeout(() => router.push('/data'), 1000)
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `删除失败: ${e}`
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
