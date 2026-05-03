<template>
  <div class="messages-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">💬 Messages</h1>
        <p class="page-subtitle">View and send messages</p>
      </div>
      <button class="btn btn-primary" @click="showCompose = true">
        ✉️ Compose
      </button>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <!-- Search -->
    <div class="card" style="margin-bottom: 16px">
      <SearchBar
        v-model="searchQuery"
        placeholder="Search messages..."
        @search="handleSearch"
      />
    </div>

    <!-- Message List -->
    <LoadingSpinner v-if="loading" text="Loading messages..." />

    <div v-else-if="messages.length === 0" class="empty-state">
      <div class="empty-state-icon">📭</div>
      <div class="empty-state-text">No messages yet</div>
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

    <!-- Compose Dialog -->
    <div v-if="showCompose" class="modal-overlay" @click.self="showCompose = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="modal-title">✉️ Compose Message</h3>
          <button class="btn btn-ghost btn-sm" @click="showCompose = false">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label class="form-label">Recipient ID</label>
            <input
              v-model="composeForm.recipientId"
              class="form-input"
              placeholder="Enter recipient ID"
            />
          </div>
          <div class="form-group">
            <label class="form-label">Title</label>
            <input
              v-model="composeForm.title"
              class="form-input"
              placeholder="Enter message title"
            />
          </div>
          <div class="form-group">
            <label class="form-label">Content</label>
            <textarea
              v-model="composeForm.content"
              class="form-textarea"
              placeholder="Enter message content..."
              rows="4"
            ></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showCompose = false">Cancel</button>
          <button
            class="btn btn-primary"
            @click="handleSend"
            :disabled="sending"
          >
            {{ sending ? 'Sending...' : '📤 Send' }}
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
    statusMsg.value = `Failed to load messages: ${e}`
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  // Local filter
}

const handleSend = async () => {
  if (!composeForm.value.recipientId || !composeForm.value.content) {
    statusType.value = 'error'
    statusMsg.value = 'Please fill in recipient and message content'
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
    statusMsg.value = 'Message sent successfully'
    showCompose.value = false
    composeForm.value = { recipientId: '', title: '', content: '' }
    await loadMessages()
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to send: ${e}`
  } finally {
    sending.value = false
  }
}

const formatTime = (timestamp: string) => {
  try {
    const d = new Date(timestamp)
    return d.toLocaleString()
  } catch {
    return timestamp
  }
}

onMounted(loadMessages)
</script>
