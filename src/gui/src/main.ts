import {createApp} from "vue"
import {Quasar} from 'quasar'

import App from "./App.vue"
import {setupI18n} from "@/services/i18n"

import "@quasar/extras/roboto-font/roboto-font.css"
import "@quasar/extras/material-icons/material-icons.css"
import "@/global.sass"

const i18n = setupI18n()

createApp(App)
    .use(i18n)
    .use(Quasar, {config: {}})
    .mount('#app')
