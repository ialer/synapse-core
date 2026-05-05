<template>
  <div v-if="show" :class="['status-message', `status-message-${type}`]">
    <span class="status-message-icon">{{ icon }}</span>
    <span>{{ message }}</span>
    <button v-if="dismissible" class="btn btn-ghost btn-sm" @click="$emit('dismiss')">
      ✕
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    show: boolean
    type?: 'success' | 'error' | 'info' | 'warning'
    message: string
    dismissible?: boolean
  }>(),
  {
    type: 'info',
    dismissible: true,
  }
)

defineEmits<{
  dismiss: []
}>()

const icon = computed(() => {
  switch (props.type) {
    case 'success': return '✓'
    case 'error': return '✕'
    case 'warning': return '⚠'
    case 'info':
    default: return 'ℹ'
  }
})
</script>
