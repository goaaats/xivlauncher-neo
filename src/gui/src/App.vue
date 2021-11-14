<template>
  <div class="content">
    <MainWindow/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {useQuasar} from "quasar"
import MainWindow from "@/views/main/Main.vue"

export default defineComponent({
  name: 'App',
  components: {
    MainWindow,
  },
  setup() {
    const $q = useQuasar()
    $q.dark.set(true)
  }
});
</script>

<style lang="sass">

body
  margin: 0

</style>