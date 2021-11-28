import {createRouter, createWebHashHistory} from 'vue-router'

class Route {
  public path: string

  constructor(path: string) {
    this.path = path
  }

  public async push() {
    await router.push(this.path)
  }
}

export const LOADING_ROUTE = new Route('/')
export const MAIN_ROUTE = new Route('/main')
export const SETUP_ROUTE = new Route('/setup')
export const CONFIG_ROUTE = new Route('/config')

const routes = [
  {path: LOADING_ROUTE.path, name: 'Loading'},
  {path: MAIN_ROUTE.path, name: 'Main'},
  {path: SETUP_ROUTE.path, name: 'Setup'},
  {path: CONFIG_ROUTE.path, name: 'Config'},
]

export const router = createRouter({
  scrollBehavior: () => ({left: 0, top: 0}),
  history: createWebHashHistory(),
  routes: routes.map((route) => {
    return {
      ...route,
      component: () => import(`../views/${route.name}.vue`),
    }
  })
})
