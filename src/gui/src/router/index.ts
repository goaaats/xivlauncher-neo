import {createRouter, createWebHashHistory} from 'vue-router'
import * as route from '@/router/route'

const routeOptions = [
  {path: route.LOADING_ROUTE.path, name: 'Loading'},
  {path: route.MAIN_ROUTE.path, name: 'Main'},
  {path: route.SETUP_ROUTE.path, name: 'Setup'},
  {path: route.CONFIG_ROUTE.path, name: 'Config'},
]

export default createRouter({
  scrollBehavior: () => ({left: 0, top: 0}),
  history: createWebHashHistory(),
  routes: routeOptions.map(route => {
    return {
      ...route,
      component: () => import(`@/views/${route.name}.vue`),
    }
  }),
})
