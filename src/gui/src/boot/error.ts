import {boot} from 'quasar/wrappers'
import {Notify, useQuasar} from 'quasar'
import {log, t} from '@/services'

export default boot(({app}) => {

  Notify.registerType('message', {
    color: 'primary',
    timeout: 3000,
    multiLine: false,
  })

  Notify.registerType('error', {
    icon: 'mdi-error',
    color: 'negative',
    timeout: 0,
    multiLine: true,
    classes: 'ws-wrap',
    actions: [{label: t('ErrorDismiss'), color: 'white'}],
  })

  /*
  app.config.errorHandler = (err: unknown, vm: ComponentPublicInstance | null, info: string) => {
    console.error(err)
    errorHandler(info)
  }
  */


  function promiseHandler(event: PromiseRejectionEvent) {
    event.preventDefault()

    if (event.reason instanceof Object && 'message' in event.reason) {
      const context = event.reason as { message: string }
      log.error('Unhandled error: ', context.message)
    }
    else {
      console.error('Unhandled error:', event.reason)
    }

    // errorHandler(event.reason as string)
  }

  window.addEventListener('unhandledrejection', promiseHandler)

  // noinspection JSUnusedLocalSymbols
  function errorHandler(msg: string) {
    log.error(`Unhandled error: ${msg}`)

    const $q = useQuasar()
    if (!$q) {
      console.error('Error during the loading process')
      return
    }

    $q.notify({
      type: 'error',
      message: `${t('ErrorWarning')}\n${msg}`,
    })
  }
})
