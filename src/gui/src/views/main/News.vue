<template>
  <q-card class="overflow-hidden">
    <img :src="placeholder_src" :alt="placeholder_alt"
         @click="clickNews" draggable="false"
         class="full-width full-height">
  </q-card>
</template>

<style lang="sass" scoped>
</style>

<script>
import {defineComponent} from "vue"
import {QCard} from "quasar";

function clickNews() {
}

export default defineComponent({
  name: "main-news",
  components: {
    QCard
  },
  setup() {
    const placeholder_src = "/static/placeholder-news.png"
    const placeholder_alt = "Placeholder"

    return {
      placeholder_src, placeholder_alt,
      clickNews,
    }
  }
})
</script>


