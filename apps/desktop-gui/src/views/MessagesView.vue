<template>
  <div class="messages-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">💬 消息</h1>
        <p class="page-subtitle">查看和发送消息</p>
      </div>
      <button class="btn btn-primary" @click="showCompose = true">
        ✉️ 写消息
      </button>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <!-- 搜索 -->
    <div class="card" style="margin-bottom: 16px">
      <SearchBar
        v-model="searchQuery"
        placeholder="搜索消息..."
        @search="handleSearch"
      />
    </div>

    <!-- 消息列表 -->
    <LoadingSpinner v-if="loading" text="加载消息..." />

    <div v-else-if="messages.length === 0" class="empty-state">
      <div class="empty-state-icon">📭</div>
      <div class="empty-state-text">暂无消息</div>
    </div>

    <div v-else class="message-list">
      <div v-for="msg in messages" :key="msg.id" class="message-item">
        <div class="message-item-header">
          <span class="message-item-from">{{ msg.from }}</span>
          <span class="message-item-time">{{ formatTime(msg.timestamp) }}</span>
        </div>
        <div class="message-item-title">{{ msg.title }}</div>
        <div class="message-item-content">{{ msg.content }}</div>
      </div>
    </div>

    <!-- 写消息对话框 -->
    <div v-if="showCompose" class="modal-overlay" @click.self="showCompose = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="modal-title">✉️ 写消息</h3>
          <button class="btn btn-ghost btn-sm" @click="showCompose = false">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label class="form-label">收件人 ID</label>
            <input
              v-model="composeForm.recipientId"
              class="form-input"
              placeholder="输入收件人 ID"
            />
          </div>
          <div class="form-group">
            <label class="form-label">标题</label>
            <input
              v-model="composeForm.title"
              class="form-input"
              placeholder="输入消息标题"
            />
          </div>
          <div class="form-group">
            <label class="form-label">内容</label>
            <textarea
              v-model="composeForm.content"
              class="form-textarea"
              placeholder="输入消息内容..."
              rows="4"
            ></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showCompose = false">取消</button>
          <button
            class="btn btn-primary"
            @click="handleSend"
            :disabled="sending"
          >
            {{ sending ? '发送中...' : '📤 发送' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getMessages, sendMessage } from '../api'
import SearchBar from '../components/SearchBar.vue'
import LoadingSpinner from '../components/LoadingSpinner.vue'
import StatusMessage from '../components/StatusMessage.vue'
import type { MessageInfo } from '../types'

const messages = ref<MessageInfo[]>([])
const loading = ref(true)
const searchQuery = ref('')
const showCompose = ref(false)
const sending = ref(false)
const statusMsg = ref('')
const statusType = ref<'success' | 'error'>('success')

const composeForm = ref({
  recipientId: '',
  title: '',
  content: '',
})

const loadMessages = async () => {
  loading.value = true
  try {
    messages.value = await getMessages('user-1', 50)
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `加载消息失败: ${e}`
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  // 本地过滤
}

const handleSend = async () => {
  if (!composeForm.value.recipientId || !composeForm.value.content) {
    statusType.value = 'error'
    statusMsg.value = '请填写收件人和消息内容'
    return
  }

  sending.value = true
  try {
    await sendMessage(
      'demo-token',
      composeForm.value.recipientId,
      composeForm.value.title,
      composeForm.value.content
    )
    statusType.value = 'success'
    statusMsg.value = '消息发送成功'
    showCompose.value = false
    composeForm.value = { recipientId: '', title: '', content: '' }
    await loadMessages()
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `发送失败: ${e}`
  } finally {
    sending.value = false
  }
}

const formatTime = (timestamp: string) => {
  try {
    const d = new Date(timestamp)
    return d.toLocaleString('zh-CN')
  } catch {
    return timestamp
  }
}

onMounted(loadMessages)
</script>
