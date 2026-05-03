<template>
  <!-- Desktop sidebar -->
  <nav class="sidebar-nav">
    <router-link
      v-for="item in navItems"
      :key="item.path"
      :to="item.path"
      class="sidebar-nav-item"
      active-class="active"
    >
      <span class="sidebar-nav-icon">{{ item.icon }}</span>
      <span>{{ item.label }}</span>
      <span v-if="item.badge" class="sidebar-nav-badge">{{ item.badge }}</span>
    </router-link>
    <div class="sidebar-nav-divider"></div>
    <router-link to="/settings" class="sidebar-nav-item" active-class="active">
      <span class="sidebar-nav-icon">⚙️</span>
      <span>Settings</span>
    </router-link>
  </nav>

  <!-- Mobile bottom nav -->
  <nav class="bottom-nav">
    <router-link
      v-for="item in bottomNavItems"
      :key="item.path"
      :to="item.path"
      class="bottom-nav-item"
      active-class="active"
    >
      <span class="bottom-nav-icon">{{ item.icon }}</span>
      <span class="bottom-nav-label">{{ item.label }}</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
const navItems = [
  { path: '/', icon: '📊', label: 'Dashboard' },
  { path: '/data', icon: '💾', label: 'Data Hub' },
  { path: '/data/new', icon: '➕', label: 'New Data' },
  { path: '/shares', icon: '🔗', label: 'Shares', badge: 2 },
  { path: '/messages', icon: '💬', label: 'Messages' },
]

const bottomNavItems = [
  { path: '/', icon: '📊', label: 'Home' },
  { path: '/data', icon: '💾', label: 'Data' },
  { path: '/data/new', icon: '➕', label: 'New' },
  { path: '/shares', icon: '🔗', label: 'Shares' },
  { path: '/settings', icon: '⚙️', label: 'Settings' },
]
</script>

<style scoped>
.sidebar-nav-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 18px;
  height: 18px;
  padding: 0 5px;
  font-size: 0.6875rem;
  font-weight: 600;
  background: var(--color-primary);
  color: white;
  border-radius: var(--radius-full);
  margin-left: auto;
}
</style>
