import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('../views/DashboardView.vue'),
    meta: { title: 'Dashboard', icon: '📊' },
  },
  {
    path: '/data',
    name: 'data',
    component: () => import('../views/DataView.vue'),
    meta: { title: 'Data Hub', icon: '💾' },
  },
  {
    path: '/data/new',
    name: 'data-create',
    component: () => import('../views/DataCreateView.vue'),
    meta: { title: 'New Data', icon: '➕' },
  },
  {
    path: '/data/:id',
    name: 'data-detail',
    component: () => import('../views/DataDetailView.vue'),
    meta: { title: 'Data Detail', icon: '📄' },
  },
  {
    path: '/shares',
    name: 'shares',
    component: () => import('../views/SharesView.vue'),
    meta: { title: 'Shares', icon: '🔗' },
  },
  {
    path: '/messages',
    name: 'messages',
    component: () => import('../views/MessagesView.vue'),
    meta: { title: 'Messages', icon: '💬' },
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('../views/SettingsView.vue'),
    meta: { title: 'Settings', icon: '⚙️' },
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

// Route guard - update page title
router.beforeEach((to) => {
  const title = (to.meta.title as string) || 'SynapseCore'
  document.title = `${title} - SynapseCore`
})

export default router
