import {createRouter, createWebHashHistory} from 'vue-router'
import * as route from '@/router/route'

const routeOptions = [
  {path: route.LOADING_ROUTE, name: 'Loading'},
  {path: route.MAIN_ROUTE, name: 'Main'},
  {path: route.SETUP_ROUTE, name: 'Setup'},
  {path: route.CONFIG_ROUTE, name: 'Config'},
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
