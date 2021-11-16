import * as Vue from "vue"
import {Quasar} from "quasar"

import App from "./App.vue"
import {setupI18n} from "@/services/i18n"
import {setupRouter} from "@/services/router"

import "@quasar/extras/roboto-font/roboto-font.css"
import "@quasar/extras/material-icons/material-icons.css"
import "@/global.sass"

(async () => {
    Vue.createApp(App)
        .use(await setupI18n())
        .use(setupRouter())
        .use(Quasar, {config: {}})
        .mount("#app")
})()

