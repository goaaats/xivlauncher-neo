<template>
  <q-card id="xiv-news">
    <img id="xiv-news-img" :src="placeholder_src" :alt="placeholder_alt" @click="clickNews" draggable="false">
  </q-card>
</template>

<style lang="sass" scoped>
#xiv-news
  overflow: hidden

#xiv-news-img
  max-width: 100%
  max-height: 100%
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


