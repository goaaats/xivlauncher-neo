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

const settingsRef = ref({}) as Ref<backend.LauncherSettings>
const addonsRef = ref([]) as Ref<backend.AddonEntry[]>
const accountsRef = ref([]) as Ref<backend.AccountEntry[]>
const uidCacheRef = ref([]) as Ref<backend.UidCacheEntry[]>

log.debug('Setting up providers')
provide(constants.SETTINGS_KEY, settingsRef)
provide(constants.ADDONS_KEY, addonsRef)
provide(constants.ACCOUNTS_KEY, accountsRef)
provide(constants.UID_CACHE_KEY, uidCacheRef)

async function setupProviders() {
  log.debug('Loading data')
  settingsRef.value = await backend.getSettings()
  addonsRef.value = await backend.getAddons()
  accountsRef.value = await backend.getAccounts()
  uidCacheRef.value = await backend.getUidCache()

  if (settingsRef.value.game_path) {
    log.debug('Game path set, loading main')
    await i18n.setLanguage(settingsRef.value.launcher_language)
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
