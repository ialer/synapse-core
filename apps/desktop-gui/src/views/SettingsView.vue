<template>
  <div class="settings-view">
    <div class="page-header">
      <div>
        <h1 class="page-title">⚙️ Settings</h1>
        <p class="page-subtitle">System configuration and preferences</p>
      </div>
    </div>

    <StatusMessage
      :show="!!statusMsg"
      :type="statusType"
      :message="statusMsg"
      @dismiss="statusMsg = ''"
    />

    <div class="settings-grid">
      <!-- Storage Configuration -->
      <div class="card">
        <h3 class="card-title">💾 Storage Backend</h3>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Storage Backend</div>
            <div class="setting-description">Choose how your data is stored</div>
          </div>
          <select v-model="settings.storageBackend" class="form-select" style="width: auto">
            <option value="local">Local Storage</option>
            <option value="encrypted">Encrypted Local</option>
            <option value="cloud">Cloud Storage</option>
          </select>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Data Path</div>
            <div class="setting-description">Local storage directory path</div>
          </div>
          <input
            v-model="settings.storagePath"
            class="form-input"
            style="width: 240px"
            placeholder="/tmp/synapse-data"
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Encryption</div>
            <div class="setting-description">Enable AES-256 encryption at rest</div>
          </div>
          <label class="toggle">
            <input type="checkbox" v-model="settings.encryptionEnabled" />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div v-if="settings.storageBackend === 'cloud'" class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Cloud Endpoint</div>
            <div class="setting-description">S3-compatible storage endpoint URL</div>
          </div>
          <input
            v-model="settings.cloudEndpoint"
            class="form-input"
            style="width: 280px"
            placeholder="https://s3.amazonaws.com/my-bucket"
          />
        </div>
      </div>

      <!-- User Profile -->
      <div class="card">
        <h3 class="card-title">👤 User Profile</h3>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Display Name</div>
            <div class="setting-description">Your name shown in the UI</div>
          </div>
          <input
            v-model="settings.displayName"
            class="form-input"
            style="width: 200px"
            placeholder="Enter your name"
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Email</div>
            <div class="setting-description">Contact email for shares and notifications</div>
          </div>
          <input
            v-model="settings.email"
            class="form-input"
            style="width: 240px"
            type="email"
            placeholder="user@example.com"
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Default Share Permission</div>
            <div class="setting-description">Default permission level when sharing data</div>
          </div>
          <select v-model="settings.defaultPermission" class="form-select" style="width: auto">
            <option value="read">Read Only</option>
            <option value="write">Read & Write</option>
          </select>
        </div>
      </div>

      <!-- Appearance -->
      <div class="card">
        <h3 class="card-title">🎨 Appearance</h3>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Theme</div>
            <div class="setting-description">Choose your preferred color theme</div>
          </div>
          <select v-model="settings.theme" class="form-select" style="width: auto" @change="applyTheme">
            <option value="system">System Default</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Accent Color</div>
            <div class="setting-description">Primary color used throughout the app</div>
          </div>
          <div class="color-picker">
            <button
              v-for="color in accentColors"
              :key="color.value"
              :class="['color-swatch', { active: settings.accentColor === color.value }]"
              :style="{ background: color.hex }"
              :title="color.label"
              @click="settings.accentColor = color.value"
            ></button>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Compact Mode</div>
            <div class="setting-description">Reduce spacing for more content on screen</div>
          </div>
          <label class="toggle">
            <input type="checkbox" v-model="settings.compactMode" />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Show Animations</div>
            <div class="setting-description">Enable page transitions and micro-interactions</div>
          </div>
          <label class="toggle">
            <input type="checkbox" v-model="settings.showAnimations" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <!-- Security -->
      <div class="card">
        <h3 class="card-title">🔒 Security</h3>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Auto Lock</div>
            <div class="setting-description">Automatically lock the app after inactivity</div>
          </div>
          <label class="toggle">
            <input type="checkbox" v-model="settings.autoLock" />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div v-if="settings.autoLock" class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Lock Timeout</div>
            <div class="setting-description">Minutes before auto-lock activates</div>
          </div>
          <select v-model="settings.lockTimeout" class="form-select" style="width: auto">
            <option :value="5">5 minutes</option>
            <option :value="15">15 minutes</option>
            <option :value="30">30 minutes</option>
            <option :value="60">1 hour</option>
          </select>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Require Approval for Shares</div>
            <div class="setting-description">All incoming share requests require manual approval</div>
          </div>
          <label class="toggle">
            <input type="checkbox" v-model="settings.requireApproval" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <!-- About -->
      <div class="card">
        <h3 class="card-title">ℹ️ About</h3>
        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Version</div>
            <div class="setting-description">SynapseCore Desktop v0.1.0</div>
          </div>
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <div class="setting-label">Framework</div>
            <div class="setting-description">Vue 3 + Tauri + Rust</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="settings-actions">
      <button class="btn btn-primary" @click="saveSettings">
        💾 Save Settings
      </button>
      <button class="btn btn-secondary" @click="resetSettings">
        🔄 Reset Defaults
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import StatusMessage from '../components/StatusMessage.vue'

interface ExtendedSettings {
  storageBackend: string
  storagePath: string
  autoLock: boolean
  lockTimeout: number
  encryptionEnabled: boolean
  cloudEndpoint: string
  displayName: string
  email: string
  defaultPermission: string
  theme: string
  accentColor: string
  compactMode: boolean
  showAnimations: boolean
  requireApproval: boolean
}

const defaultSettings: ExtendedSettings = {
  storageBackend: 'local',
  storagePath: '/tmp/synapse-data',
  autoLock: false,
  lockTimeout: 15,
  encryptionEnabled: true,
  cloudEndpoint: '',
  displayName: '',
  email: '',
  defaultPermission: 'read',
  theme: 'system',
  accentColor: 'purple',
  compactMode: false,
  showAnimations: true,
  requireApproval: true,
}

const settings = ref<ExtendedSettings>({ ...defaultSettings })
const statusMsg = ref('')
const statusType = ref<'success' | 'error' | 'info'>('success')

const accentColors = [
  { value: 'purple', hex: '#667eea', label: 'Purple' },
  { value: 'blue', hex: '#3b82f6', label: 'Blue' },
  { value: 'green', hex: '#10b981', label: 'Green' },
  { value: 'orange', hex: '#f59e0b', label: 'Orange' },
  { value: 'red', hex: '#ef4444', label: 'Red' },
  { value: 'pink', hex: '#ec4899', label: 'Pink' },
]

const applyTheme = () => {
  const root = document.documentElement
  const theme = settings.value.theme

  if (theme === 'dark') {
    root.classList.add('dark')
    root.classList.remove('light')
  } else if (theme === 'light') {
    root.classList.add('light')
    root.classList.remove('dark')
  } else {
    root.classList.remove('dark', 'light')
  }
}

const saveSettings = () => {
  try {
    localStorage.setItem('synapse-settings', JSON.stringify(settings.value))
    applyTheme()
    statusType.value = 'success'
    statusMsg.value = 'Settings saved successfully'
  } catch (e) {
    statusType.value = 'error'
    statusMsg.value = `Failed to save: ${e}`
  }
}

const resetSettings = () => {
  settings.value = { ...defaultSettings }
  statusType.value = 'info'
  statusMsg.value = 'Settings reset to defaults (click Save to apply)'
}

onMounted(() => {
  const stored = localStorage.getItem('synapse-settings')
  if (stored) {
    try {
      settings.value = { ...defaultSettings, ...JSON.parse(stored) }
    } catch {
      // Use defaults
    }
  }
})
</script>

<style scoped>
.settings-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--space-md);
}

.color-picker {
  display: flex;
  gap: 8px;
}

.color-swatch {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 3px solid transparent;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.color-swatch:hover {
  transform: scale(1.1);
}

.color-swatch.active {
  border-color: var(--color-text);
  box-shadow: 0 0 0 2px var(--color-bg-card);
}

.settings-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

@media (min-width: 768px) {
  .settings-grid {
    grid-template-columns: repeat(2, 1fr);
  }
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
