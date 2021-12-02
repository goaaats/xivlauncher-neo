import {createLogger} from 'vue-logger-plugin'
import {LogEvent} from 'vue-logger-plugin/dist/types/logger'
import {backend} from '@/services'

async function logBackend(event: LogEvent) {
  console.log(event.argumentArray)
  return

  switch (event.level) {
    case 'debug':
      await backend.debug('')
      return
    case 'info':
      await backend.info('')
      return
    case 'warn':
      await backend.warn('')
      return
    case 'error':
      await backend.error('')
      return
  }
}

export const log = createLogger({
  enabled: true,
  level: 'debug',
  afterHooks: [{run: logBackend}],
})
