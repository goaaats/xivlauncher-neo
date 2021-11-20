import * as Vue from "vue"
import {Quasar, Notify} from "quasar"

import App from "./App.vue"
import {setupI18n} from "@/services/i18n"
import {setupRouter} from "@/services/router"

import "@quasar/extras/roboto-font/roboto-font.css"
import "@quasar/extras/material-icons/material-icons.css"
import "@/global.sass"

(async () => {
    const router = setupRouter()
    const i18n = await setupI18n()

    const app = Vue.createApp(App)
        .use(i18n)
        .use(router)
        .use(Quasar, {
            plugins: {Notify},
            config: {notify: {}},
        })

    app.provide("app", app)
    app.mount("#app")
})()

