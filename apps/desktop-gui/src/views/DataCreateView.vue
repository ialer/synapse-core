<template>
  <div class="data-create-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">➕ 新建数据</h1>
        <p class="page-subtitle">存储新的数据条目</p>
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

    <div class="card">
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label class="form-label">数据类型</label>
          <select v-model="form.dataType" class="form-select">
            <option v-for="opt in DATA_TYPE_OPTIONS" :key="opt.value" :value="opt.value">
              {{ opt.icon }} {{ opt.label }}
            </option>
          </select>
        </div>

        <div class="form-group">
          <label class="form-label">内容</label>
          <textarea
            v-model="form.content"
            class="form-textarea"
            placeholder="输入数据内容..."
            rows="6"
            required
          ></textarea>
          <span class="form-hint">支持文本、JSON、配置文件等内容</span>
        </div>

        <div class="form-group">
          <label class="form-label">标签</label>
          <input
            v-model="form.tags"
            class="form-input"
            placeholder="输入标签，用逗号分隔 (如: 工作, 重要)"
          />
          <span class="form-hint">多个标签用逗号分隔</span>
        </div>

        <div class="form-actions">
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? '存储中...' : '💾 存储数据' }}
          </button>
          <router-link to="/data" class="btn btn-secondary">取消</router-link>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { storeData } from '../api'
import StatusMessage from '../components/StatusMessage.vue'
import { DATA_TYPE_OPTIONS } from '../types'
import type { DataType } from '../types'

const router = useRouter()

const form = ref({
  dataType: 'generic' as DataType,
  content: '',
  tags: '',
})

const submitting = ref(false)
const statusMsg = ref('')
const statusType = ref<'success' | 'error'>('success')

const handleSubmit = async () => {
  if (!form.value.content.trim()) {
    statusType.value = 'error'
    statusMsg.value = '请输入数据内容'
    return
  }

  submitting.value = true

  try {
    const tags = form.value.tags
      .split(',')
      .map((t) => t.trim())
      .filter(Boolean)

    await storeData('demo-token', form.value.dataType, form.value.content, tags)

    statusType.value = 'success'
    statusMsg.value = '数据存储成功！'

    // 1秒后跳转
    setTimeout(() => {
      router.push('/data')
    }, 1000)
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `存储失败: ${e}`
  } finally {
    submitting.value = false
  }
}
</script>

<style scoped>
.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

@media (max-width: 480px) {
  .form-actions {
    flex-direction: column;
  }

  .form-actions .btn {
    width: 100%;
  }
}
</style>
