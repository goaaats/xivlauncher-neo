<template>
  <div class="window-width window-height overflow-hidden">
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import {defineComponent, inject, App} from "vue"
import {ComponentPublicInstance} from "@vue/runtime-core"
import {useRouter} from "vue-router"
import {useQuasar, Notify} from "quasar"
import {useI18n} from "vue-i18n"
import {
  getAccounts, getAddons, getSettings, getUidCache,
} from "@/services/backend"

export default defineComponent({
  name: "app",
  components: {},
  setup() {
    const app = inject("app") as App
    const router = useRouter()
    const {t} = useI18n()
    const q = useQuasar()

    app.provide("i18n", t)
    app.provide("quasar", q)

    q.dark.set(true)

    // region Error handling

    console.log("Setting up error handling")

    Notify.registerType("error", {
      icon: "error",
      color: "negative",
      timeout: 0,
      multiLine: true,
      classes: "ws-wrap",
      actions: [{label: t("ErrorDismiss"), color: "white"}],
    })

    app.config.errorHandler = (err: unknown, vm: ComponentPublicInstance | null, info: string) => {
      console.error(err)
      errorHandler(info)
    }

    window.addEventListener("unhandledrejection", event => {
      event.preventDefault()
      console.error(event)
      errorHandler(event.reason)
    })

    function errorHandler(msg: string) {
      console.error(`Unhandled error: ${msg}`)
      q.notify({
        type: "error",
        message: `${t("ErrorWarning")}\n${msg}`,
      })
    }

    // endregion

    console.log("Loading data");

    (async () => {
      const settings = await getSettings()
      app.provide("settings", settings)
      app.provide("addons", await getAddons())
      app.provide("accounts", await getAccounts())
      app.provide("uidCache", await getUidCache())

      if (settings.game_path) {
        await router.push("/setup")
        //await router.push("main")
      } else {
        //await router.push("setup")
      }
    })()

    return {}
  },

})
</script>

<style lang="sass">
.ws-wrap
  white-space: pre-wrap !important
</style>