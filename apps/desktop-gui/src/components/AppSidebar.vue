<template>
  <!-- 桌面端侧边栏 -->
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
    </router-link>
    <div class="sidebar-nav-divider"></div>
    <router-link to="/settings" class="sidebar-nav-item" active-class="active">
      <span class="sidebar-nav-icon">⚙️</span>
      <span>设置</span>
    </router-link>
  </nav>

  <!-- 移动端底部导航 -->
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
  { path: '/', icon: '📊', label: '仪表盘' },
  { path: '/data', icon: '💾', label: '数据管理' },
  { path: '/data/new', icon: '➕', label: '新建数据' },
  { path: '/messages', icon: '💬', label: '消息' },
]

const bottomNavItems = [
  { path: '/', icon: '📊', label: '仪表盘' },
  { path: '/data', icon: '💾', label: '数据' },
  { path: '/data/new', icon: '➕', label: '新建' },
  { path: '/messages', icon: '💬', label: '消息' },
  { path: '/settings', icon: '⚙️', label: '设置' },
]
</script>
