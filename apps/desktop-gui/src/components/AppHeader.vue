<template>
  <header class="app-header">
    <div class="header-brand">
      <span class="header-logo">🧠</span>
      <span class="header-title">SynapseCore</span>
    </div>
    <div class="header-actions">
      <slot name="actions" />
    </div>
  </header>
</template>

<script setup lang="ts">
</script>

<style scoped>
.header-brand {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-logo {
  font-size: 1.5rem;
}

.header-title {
  font-size: 1.125rem;
  font-weight: 700;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-actions {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
