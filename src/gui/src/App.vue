<template>
  <div class="fullscreen overflow-hidden">
    <router-view></router-view>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, Ref, ref} from 'vue'
import {backend, i18n, log, store} from '@/services'
import {AccountEntry} from '@/services/backend'
import {MAIN_ROUTE, SETUP_ROUTE} from '@/services/router'

const config = ref({}) as Ref<backend.LauncherConfig>

log.debug('Setting up providers')
store.CONFIG.provide(config)
store.SHOW_MAINTENANCE_DIALOG.provide(ref(false))
store.SHOW_ACCOUNTS_DIALOG.provide(ref(false))
store.SHOW_EDIT_ACCOUNT_DIALOG.provide(ref(false))
store.CURRENT_ACCOUNT.provide(ref({}) as Ref<AccountEntry>)

async function setupProviders() {
  log.debug('Loading data')
  config.value = await backend.getConfig()

  if (config.value.game_path) {
    log.debug('Game path set, loading main')
    await i18n.setLanguage(config.value.launcher_language)
    await MAIN_ROUTE.push()
  } else {
    log.debug('Game path missing, loading setup')
    await SETUP_ROUTE.push()
  }
}

onMounted(setupProviders)

</script>

<style lang="sass">
</style>
