import { createMemoryHistory, createRouter } from 'vue-router';

import WorkStation from '../views/WorkStation.vue';
import HomeView from '../views/HomeView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/space', component: WorkStation },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router;