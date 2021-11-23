<template>
  <q-card class="overflow-hidden">
    <img :src="placeholder_src" :alt="placeholder_alt"
         draggable="false" class="full-width full-height"
         @click="clickNews">
  </q-card>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {QCard} from 'quasar'

function clickNews() {
  console.log('clickNews')
}

export default defineComponent({
  name: 'MainNews',
  components: {
    QCard,
  },
  setup() {
    const placeholder_src = '/static/placeholder-news.png'
    const placeholder_alt = 'Placeholder'

    return {
      placeholder_src, placeholder_alt,
      clickNews,
    }
  },
})
</script>

<style lang="sass" scoped>
</style>
