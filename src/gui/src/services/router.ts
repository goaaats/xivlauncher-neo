import * as VueRouter from "vue-router"
import LoadingView from "@/views/Loading.vue"
import MainView from "@/views/Main.vue"
import SetupView from "@/views/Setup.vue"
import ConfigView from "@/views/Config.vue"

const routes = [
    {path: "/", component: LoadingView},
    {path: "/main", component: MainView},
    {path: "/setup", component: SetupView},
    {path: "/config", component: ConfigView},
]

export function setupRouter() {
    return VueRouter.createRouter({
        history: VueRouter.createWebHashHistory(),
        routes: routes,
    })
}
