import {createApp} from 'vue'
import {Notify, Quasar} from 'quasar'
import quasarIconSet from 'quasar/icon-set/mdi-v6'

import '@quasar/extras/roboto-font/roboto-font.css'
import '@quasar/extras/mdi-v6/mdi-v6.css'
import 'quasar/dist/quasar.prod.css'
import '@/css/app.sass'

import App from '@/App.vue'
import {log, router} from '@/services'
import {setupI18n} from '@/services/i18n'
import {setupErrorHandler} from '@/services/error'

void (async () => {
  const app = createApp(App)

  // Quasar
  app.use(Quasar, {
    config: {
      notify: {},
    },
    iconSet: quasarIconSet,
    plugins: {Notify},
  })


  // i18n
  app.use(await setupI18n())

  // Logging
  app.use(log)

  // Errors
  setupErrorHandler()

  app.use(router)

  app.mount('#app')
})()
