<template>
  <div class="fullscreen overflow-hidden">
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import {defineComponent, onMounted, provide, Ref, ref} from 'vue'
import {useRouter} from 'vue-router'
import {useQuasar} from 'quasar'
import * as backend from 'src/services/backend'
import * as constants from '@/services/constants'
import * as i18n from '@/services/i18n'
import * as route from '@/router/route'

export default defineComponent({
  name: 'App',
  setup() {
    const router = useRouter()
    const $q = useQuasar()

    $q.dark.set(true)

    console.log('Loading data')
    const settingsRef = ref({}) as Ref<backend.LauncherSettings>
    const addonsRef = ref([]) as Ref<backend.AddonEntry[]>
    const accountsRef = ref([]) as Ref<backend.AccountEntry[]>
    const uidCacheRef = ref([]) as Ref<backend.UidCacheEntry[]>

    provide(constants.SETTINGS_KEY, settingsRef)
    provide(constants.ADDONS_KEY, addonsRef)
    provide(constants.ACCOUNTS_KEY, accountsRef)
    provide(constants.UID_CACHE_KEY, uidCacheRef)

    onMounted(async () => {
      settingsRef.value = await backend.getSettings()
      addonsRef.value = await backend.getAddons()
      accountsRef.value = await backend.getAccounts()
      uidCacheRef.value = await backend.getUidCache()

      if (settingsRef.value.game_path) {
        // Valid config, set the language and proceed to main
        await i18n.setLanguage(settingsRef.value.launcher_language)
        await router.push(route.MAIN_ROUTE)
      } else {
        // Either invalid or default, proceed to setup
        await router.push(route.SETUP_ROUTE)
      }
    })

    return {}
  },
})
</script>

<style lang="sass">
</style>
