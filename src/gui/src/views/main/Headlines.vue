<template>
  <q-card id="xiv-headlines">
    <q-scroll-area id="xiv-headlines-scroll">
      <q-list>

        <q-item clickable v-ripple>
          <q-item-section avatar>
            <q-icon name="feed"/>
          </q-item-section>
          <q-item-section>
            blah blah
            <!--<q-skeleton type="text" animation="blink"/>-->
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple>
          <q-item-section avatar>
            <q-icon name="feed"/>
          </q-item-section>
          <q-item-section>
            blah blah
            <!--<q-skeleton type="text" animation="blink"/>-->
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple>
          <q-item-section avatar>
            <q-icon name="feed"/>
          </q-item-section>
          <q-item-section>
            blah blah
            <!--<q-skeleton type="text" animation="blink"/>-->
          </q-item-section>
        </q-item>

      </q-list>
    </q-scroll-area>
  </q-card>
</template>

<style lang="sass" scoped>
#xiv-headlines
  overflow: hidden
  
#xiv-headlines-scroll
  width: 100%
  height: 100%
</style>

<script>
import {defineComponent} from "vue"
import {QCard} from "quasar";

export default defineComponent({
  name: "main-headlines",
  components: {
    QCard
  },
  setup() {
    return {}
  }
})
</script>


