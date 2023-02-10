import { createRouter, createWebHistory } from 'vue-router';

import Main from '@/views/Main.vue';
import Setting from '@/views/Setting.vue';

const routes = [
  { path: '/', component: Main },
  { path: '/setting', component: Setting },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
