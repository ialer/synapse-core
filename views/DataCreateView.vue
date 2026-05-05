<template>
  <div class="data-create-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">➕ New Data</h1>
        <p class="page-subtitle">Store a new data item</p>
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

    <div class="card">
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label class="form-label">Data Type</label>
          <select v-model="form.dataType" class="form-select">
            <option v-for="opt in DATA_TYPE_OPTIONS" :key="opt.value" :value="opt.value">
              {{ opt.icon }} {{ opt.label }}
            </option>
          </select>
        </div>

        <div class="form-group">
          <label class="form-label">Content</label>
          <textarea
            v-model="form.content"
            class="form-textarea"
            placeholder="Enter data content..."
            rows="6"
            required
          ></textarea>
          <span class="form-hint">Supports text, JSON, config files, etc.</span>
        </div>

        <div class="form-group">
          <label class="form-label">Tags</label>
          <input
            v-model="form.tags"
            class="form-input"
            placeholder="Enter tags, comma separated (e.g.: work, important)"
          />
          <span class="form-hint">Separate multiple tags with commas</span>
        </div>

        <div class="form-actions">
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? 'Storing...' : '💾 Store Data' }}
          </button>
          <router-link to="/data" class="btn btn-secondary">Cancel</router-link>
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
    statusMsg.value = 'Please enter data content'
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
    statusMsg.value = 'Data stored successfully!'

    // Redirect after 1 second
    setTimeout(() => {
      router.push('/data')
    }, 1000)
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Storage failed: ${e}`
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
