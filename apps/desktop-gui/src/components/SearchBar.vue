<template>
  <div class="search-bar">
    <span class="search-bar-icon">🔍</span>
    <input
      ref="inputRef"
      class="search-bar-input"
      type="text"
      :value="modelValue"
      :placeholder="placeholder"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      @keyup.enter="$emit('search')"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

withDefaults(
  defineProps<{
    modelValue: string
    placeholder?: string
  }>(),
  {
    placeholder: '搜索...',
  }
)

defineEmits<{
  'update:modelValue': [value: string]
  search: []
}>()

const inputRef = ref<HTMLInputElement>()

const focus = () => {
  inputRef.value?.focus()
}

defineExpose({ focus })
</script>
