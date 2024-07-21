import { createMemoryHistory, createRouter } from 'vue-router';

import HomeView from '../views/HomeView.vue'
import EnamelDesigner from '../views/workspaces/EnamelDesigner.vue';
import DatasetProducer from '../views/workspaces/exps/DatasetProducer.vue';

const routes = [
  { path: '/', component: HomeView },
  { path: '/space/enamel_designer', component: EnamelDesigner },
  { path: '/space/exps/dataset_producer', component: DatasetProducer },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router;