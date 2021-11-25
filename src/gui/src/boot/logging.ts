import {boot} from 'quasar/wrappers'
import {log} from '@/services/logging'

export default boot(({app}) => {
  app.use(log)
})
