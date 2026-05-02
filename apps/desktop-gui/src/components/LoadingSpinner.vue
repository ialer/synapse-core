<template>
  <div class="loading-spinner">
    <div class="spinner"></div>
    <span v-if="text" class="loading-text">{{ text }}</span>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  text?: string
}>()
</script>
