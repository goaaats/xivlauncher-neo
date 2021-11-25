import {Router} from 'vue-router'

class Route {
  public path: string

  private static router: Router | null = null

  constructor(path: string) {
    this.path = path
  }

  public async push() {
    if (Route.router === null)
      Route.router = (await import('@/router/')).default

    await Route.router.push(this.path)
  }
}

export const LOADING_ROUTE = new Route('/')
export const MAIN_ROUTE = new Route('/main')
export const SETUP_ROUTE = new Route('/setup')
export const CONFIG_ROUTE = new Route('/config')
