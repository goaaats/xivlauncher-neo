import * as VueRouter from "vue-router"
import RootView from "@/views/Root.vue"
import MainView from "@/views/main/Main.vue"
import SetupView from "@/views/Setup.vue"

const routes = [
    {path: "/", component: RootView},
    {path: "/main", component: MainView},
    {path: "/setup", component: SetupView},
]

export function setupRouter() {
    return VueRouter.createRouter({
        history: VueRouter.createWebHashHistory(),
        routes: routes,
    })
}
