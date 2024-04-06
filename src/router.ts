
import { createRouter, createWebHashHistory, RouterOptions, Router, RouteRecordRaw } from "vue-router";
import main_panel from './components/main_panel.vue';
import create_panel from './components/create_panel.vue';
// export default new VueRouter()
const routes: RouteRecordRaw[] =[
    {
      path: '/main_panel',
      name: 'main_panel',
      component: main_panel
    },
    {
      path: '/',
      name: 'index',
      component: main_panel
    },
    {
      path: '/create_panel',
      name: 'create_panel',
      component: create_panel
    },

  ]
const options: RouterOptions = {
    history: createWebHashHistory(),
    routes,
}
const router: Router = createRouter(options)
export default router