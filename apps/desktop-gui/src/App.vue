<template>
  <div class="app">
    <header class="header">
      <h1>🧠 SynapseCore</h1>
      <p>个人数据管理系统</p>
    </header>

    <main class="main">
      <!-- 统计信息 -->
      <section class="stats">
        <h2>📊 统计信息</h2>
        <div class="stats-grid">
          <div class="stat-card">
            <span class="stat-value">{{ stats.data_count }}</span>
            <span class="stat-label">数据条目</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">{{ stats.index_count }}</span>
            <span class="stat-label">索引条目</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">{{ stats.message_count }}</span>
            <span class="stat-label">消息数量</span>
          </div>
        </div>
      </section>

      <!-- 存储数据 -->
      <section class="store-section">
        <h2>💾 存储数据</h2>
        <div class="form-group">
          <label>数据类型</label>
          <select v-model="storeForm.dataType">
            <option value="credential">凭证</option>
            <option value="config">配置</option>
            <option value="file">文件</option>
            <option value="contact">联系人</option>
            <option value="generic">通用</option>
          </select>
        </div>
        <div class="form-group">
          <label>内容</label>
          <textarea v-model="storeForm.content" placeholder="输入数据内容..."></textarea>
        </div>
        <div class="form-group">
          <label>标签 (逗号分隔)</label>
          <input v-model="storeForm.tags" placeholder="tag1, tag2, tag3" />
        </div>
        <button @click="handleStore" :disabled="storing">
          {{ storing ? '存储中...' : '存储数据' }}
        </button>
      </section>

      <!-- 搜索数据 -->
      <section class="search-section">
        <h2>🔍 搜索数据</h2>
        <div class="form-group">
          <input v-model="searchQuery" placeholder="输入搜索关键词..." @keyup.enter="handleSearch" />
        </div>
        <button @click="handleSearch" :disabled="searching">
          {{ searching ? '搜索中...' : '搜索' }}
        </button>
        
        <div v-if="searchResults.length > 0" class="results">
          <h3>搜索结果</h3>
          <ul>
            <li v-for="result in searchResults" :key="result">
              {{ result }}
            </li>
          </ul>
        </div>
      </section>

      <!-- 状态消息 -->
      <div v-if="message" :class="['message', messageType]">
        {{ message }}
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// 状态
const stats = ref({ data_count: 0, index_count: 0, message_count: 0 })
const storing = ref(false)
const searching = ref(false)
const message = ref('')
const messageType = ref('success')
const searchResults = ref<string[]>([])

// 表单
const storeForm = ref({
  dataType: 'credential',
  content: '',
  tags: ''
})
const searchQuery = ref('')

// 获取统计信息
const fetchStats = async () => {
  try {
    const result = await invoke('get_stats')
    stats.value = result as any
  } catch (error) {
    console.error('Failed to fetch stats:', error)
  }
}

// 存储数据
const handleStore = async () => {
  if (!storeForm.value.content) {
    message.value = '请输入内容'
    messageType.value = 'error'
    return
  }

  storing.value = true
  message.value = ''

  try {
    const tags = storeForm.value.tags
      .split(',')
      .map(t => t.trim())
      .filter(t => t)

    await invoke('store_data', {
      token: 'demo-token',
      dataType: storeForm.value.dataType,
      content: storeForm.value.content,
      tags
    })

    message.value = '数据存储成功'
    messageType.value = 'success'
    storeForm.value.content = ''
    storeForm.value.tags = ''
    await fetchStats()
  } catch (error) {
    message.value = `存储失败: ${error}`
    messageType.value = 'error'
  } finally {
    storing.value = false
  }
}

// 搜索数据
const handleSearch = async () => {
  if (!searchQuery.value) {
    message.value = '请输入搜索关键词'
    messageType.value = 'error'
    return
  }

  searching.value = true
  message.value = ''

  try {
    const results = await invoke('search_data', {
      query: searchQuery.value,
      limit: 10
    })
    searchResults.value = results as string[]
    message.value = `找到 ${searchResults.value.length} 条结果`
    messageType.value = 'success'
  } catch (error) {
    message.value = `搜索失败: ${error}`
    messageType.value = 'error'
  } finally {
    searching.value = false
  }
}

// 初始化
onMounted(() => {
  fetchStats()
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
  color: #333;
}

.app {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.header {
  text-align: center;
  padding: 40px 0;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 12px;
  margin-bottom: 30px;
}

.header h1 {
  font-size: 2.5em;
  margin-bottom: 10px;
}

.main {
  display: grid;
  gap: 30px;
}

section {
  background: white;
  padding: 25px;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

h2 {
  margin-bottom: 20px;
  color: #333;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

.stat-card {
  text-align: center;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
}

.stat-value {
  display: block;
  font-size: 2em;
  font-weight: bold;
  color: #667eea;
}

.stat-label {
  color: #666;
  font-size: 0.9em;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
}

.form-group textarea {
  min-height: 100px;
  resize: vertical;
}

button {
  background: #667eea;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.2s;
}

button:hover:not(:disabled) {
  background: #5a6fd6;
}

button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.results {
  margin-top: 20px;
  padding: 15px;
  background: #f8f9fa;
  border-radius: 8px;
}

.results h3 {
  margin-bottom: 10px;
  font-size: 1em;
}

.results ul {
  list-style: none;
}

.results li {
  padding: 8px;
  border-bottom: 1px solid #eee;
}

.results li:last-child {
  border-bottom: none;
}

.message {
  padding: 15px;
  border-radius: 8px;
  margin-top: 20px;
}

.message.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.message.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}
</style>
