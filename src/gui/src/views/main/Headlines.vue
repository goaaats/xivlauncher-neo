<template>
  <q-card class="overflow-hidden">
    <q-scroll-area class="full-height full-width">
      <q-list>

        <q-item v-ripple clickable>
          <q-item-section avatar>
            <q-skeleton type="rect" animation="blink" width="24px"/>
          </q-item-section>
          <q-item-section>
            <q-skeleton type="text" animation="blink"/>
          </q-item-section>
        </q-item>

        <q-item v-ripple clickable>
          <q-item-section avatar>
            <q-icon name="priority_high"/>
          </q-item-section>
          <q-item-section>
            Actions Taken Against In-Game RMT & Other Illicit Activities (Nov. 11)
          </q-item-section>
        </q-item>

        <q-item v-ripple clickable>
          <q-item-section avatar>
            <q-icon name="error_outline"/>
          </q-item-section>
          <q-item-section>
            [Chaos] Recovery from Instanced Areas Technical Difficulties (Nov. 08)
          </q-item-section>
        </q-item>

        <q-item v-ripple clickable>
          <q-item-section avatar>
            <q-icon name="feed"/>
          </q-item-section>
          <q-item-section>
            [Primal] Leviathan World Emergency Maintenance (Nov. 6): Follow-up
          </q-item-section>
        </q-item>

      </q-list>
    </q-scroll-area>
  </q-card>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {QCard} from 'quasar'

export default defineComponent({
  name: 'MainHeadlines',
  components: {
    QCard,
  },
  setup() {
    return {}
  },
})
</script>

<style lang="sass" scoped>
</style>
