<template>
  <div class="settings-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">⚙️ 设置</h1>
        <p class="page-subtitle">系统配置与偏好设置</p>
      </div>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <!-- 存储设置 -->
    <div class="card">
      <h3 class="card-title">💾 存储设置</h3>

      <div class="setting-item">
        <div class="setting-info">
          <div class="setting-label">存储后端</div>
          <div class="setting-description">选择数据存储方式</div>
        </div>
        <select v-model="settings.storageBackend" class="form-select" style="width: auto">
          <option value="local">本地存储</option>
          <option value="encrypted">加密存储</option>
          <option value="cloud">云存储</option>
        </select>
      </div>

      <div class="setting-item">
        <div class="setting-info">
          <div class="setting-label">数据路径</div>
          <div class="setting-description">本地数据存储目录</div>
        </div>
        <input
          v-model="settings.storagePath"
          class="form-input"
          style="width: 240px"
          placeholder="/tmp/synapse-data"
        />
      </div>
    </div>

    <!-- 安全设置 -->
    <div class="card" style="margin-top: 16px">
      <h3 class="card-title">🔒 安全设置</h3>

      <div class="setting-item">
        <div class="setting-info">
          <div class="setting-label">自动锁定</div>
          <div class="setting-description">空闲一段时间后自动锁定应用</div>
        </div>
        <label class="toggle">
          <input type="checkbox" v-model="settings.autoLock" />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <div v-if="settings.autoLock" class="setting-item">
        <div class="setting-info">
          <div class="setting-label">锁定超时</div>
          <div class="setting-description">空闲多久后锁定（分钟）</div>
        </div>
        <select v-model="settings.lockTimeout" class="form-select" style="width: auto">
          <option :value="5">5 分钟</option>
          <option :value="15">15 分钟</option>
          <option :value="30">30 分钟</option>
          <option :value="60">1 小时</option>
        </select>
      </div>
    </div>

    <!-- 关于 -->
    <div class="card" style="margin-top: 16px">
      <h3 class="card-title">ℹ️ 关于</h3>
      <div class="setting-item">
        <div class="setting-info">
          <div class="setting-label">版本</div>
          <div class="setting-description">SynapseCore Desktop v0.1.0</div>
        </div>
      </div>
      <div class="setting-item">
        <div class="setting-info">
          <div class="setting-label">框架</div>
          <div class="setting-description">Vue 3 + Tauri + Rust</div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="settings-actions">
      <button class="btn btn-primary" @click="saveSettings">
        💾 保存设置
      </button>
      <button class="btn btn-secondary" @click="resetSettings">
        🔄 重置默认
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import StatusMessage from '../components/StatusMessage.vue'
import type { AppSettings } from '../types'

const settings = ref<AppSettings>({
  storageBackend: 'local',
  storagePath: '/tmp/synapse-data',
  autoLock: false,
  lockTimeout: 15,
})

const statusMsg = ref('')
const statusType = ref<'success' | 'error' | 'info'>('success')

const saveSettings = () => {
  try {
    localStorage.setItem('synapse-settings', JSON.stringify(settings.value))
    statusType.value = 'success'
    statusMsg.value = '设置已保存'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `保存失败: ${e}`
  }
}

const resetSettings = () => {
  settings.value = {
    storageBackend: 'local',
    storagePath: '/tmp/synapse-data',
    autoLock: false,
    lockTimeout: 15,
  }
  statusType.value = 'info'
  statusMsg.value = '已重置为默认设置（点击保存生效）'
}
</script>

<style scoped>
.settings-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

@media (max-width: 480px) {
  .settings-actions {
    flex-direction: column;
  }

  .settings-actions .btn {
    width: 100%;
  }
}
</style>
