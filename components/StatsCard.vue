<template>
  <div class="stat-card">
    <div class="stat-card-icon">{{ icon }}</div>
    <div class="stat-card-value">{{ formattedValue }}</div>
    <div class="stat-card-label">{{ label }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  icon: string
  value: number | string
  label: string
}>()

const formattedValue = computed(() => {
  if (typeof props.value === 'number') {
    return props.value.toLocaleString()
  }
  return props.value
})
</script>
