import {boot} from 'quasar/wrappers'
import {setupI18n} from '@/services/i18n'

export default boot(async ({app}) => {
  app.use(await setupI18n())
})
