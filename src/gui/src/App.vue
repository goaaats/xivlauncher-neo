<template>
  <div class="window-width window-height overflow-hidden">
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {useQuasar} from "quasar"

export default defineComponent({
  name: "app",
  components: {},
  setup() {
    const q = useQuasar()
    q.dark.set(true)

    //launcherConfig.initialize().then()
    //dalamudConfig.initialize().then()
  },
})
</script>

<style lang="sass">

</style>