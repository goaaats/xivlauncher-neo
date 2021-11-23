import {boot} from 'quasar/wrappers'
import {ComponentPublicInstance} from '@vue/runtime-core'
import {Notify, useQuasar} from 'quasar'
import {t} from '@/services/i18n'

export default boot(({app}) => {

  Notify.registerType('message', {
    color: 'primary',
    timeout: 3000,
    multiLine: false,
  })

  Notify.registerType('error', {
    icon: 'error',
    color: 'negative',
    timeout: 0,
    multiLine: true,
    classes: 'ws-wrap',
    actions: [{label: t('ErrorDismiss'), color: 'white'}],
  })

  app.config.errorHandler = (err: unknown, vm: ComponentPublicInstance | null, info: string) => {
    console.error(err)
    errorHandler(info)
  }

  window.addEventListener('unhandledrejection', event => {
    event.preventDefault()
    console.error(event)
    errorHandler(event.reason as string)
  })

  function errorHandler(msg: string) {
    console.error(`Unhandled error: ${msg}`)

    const $q = useQuasar()
    $q.notify({
      type: 'error',
      message: `${t('ErrorWarning')}\n${msg}`,
    })
  }
})
