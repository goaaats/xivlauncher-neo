<template>
  <div class="fullscreen overflow-hidden">
    <router-view></router-view>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, provide, Ref, ref} from 'vue'
import {useQuasar} from 'quasar'
import {backend, constants, i18n, log} from '@/services'
import {MAIN_ROUTE, SETUP_ROUTE} from '@/services/router'

const $q = useQuasar()

$q.dark.set(true)

const config = ref({}) as Ref<backend.LauncherConfig>

log.debug('Setting up providers')
provide(constants.CONFIG_KEY, config)

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
