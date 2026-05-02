import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('../views/DashboardView.vue'),
    meta: { title: '仪表盘', icon: '📊' },
  },
  {
    path: '/data',
    name: 'data',
    component: () => import('../views/DataView.vue'),
    meta: { title: '数据管理', icon: '💾' },
  },
  {
    path: '/data/new',
    name: 'data-create',
    component: () => import('../views/DataCreateView.vue'),
    meta: { title: '新建数据', icon: '➕' },
  },
  {
    path: '/data/:id',
    name: 'data-detail',
    component: () => import('../views/DataDetailView.vue'),
    meta: { title: '数据详情', icon: '📄' },
  },
  {
    path: '/messages',
    name: 'messages',
    component: () => import('../views/MessagesView.vue'),
    meta: { title: '消息', icon: '💬' },
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('../views/SettingsView.vue'),
    meta: { title: '设置', icon: '⚙️' },
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

// 路由守卫 - 更新页面标题
router.beforeEach((to) => {
  const title = (to.meta.title as string) || 'SynapseCore'
  document.title = `${title} - SynapseCore`
})

export default router
