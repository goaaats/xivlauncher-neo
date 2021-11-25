<template>
  <q-item :key="entry.id" v-ripple clickable
          class="q-px-sm q-py-none">
    <q-item-section side class="q-pr-sm">
      <q-icon v-if="entry.tag === 'Important'"
              name="mdi-alert-circle-outline"
              color="negative"
              size="xs"/>
      <q-icon v-else-if="entry.tag === 'Follow-up'"
              name="mdi-information-outline"
              color="warning"
              size="xs"/>
      <q-icon v-else-if="entry.tag === 'Maintenance'"
              name="mdi-tools"
              color="secondary"
              size="xs"/>
      <!-- <q-icon v-else-if="entry.tag === 'Recovery'" name="mdi-hospital" size="xs"/> -->
      <q-icon v-else
              name="mdi-newspaper"
              size="xs"/>
    </q-item-section>

    <q-item-section class="flex q-py-xs">
      <div class="" style="font-size: 0.75rem">
        {{ entry.title }}
      </div>
    </q-item-section>
  </q-item>
</template>

<script lang="ts" setup>
import {HeadlineEntry} from '@/services/backend'

defineProps<{
  entry: HeadlineEntry
}>()
</script>

<style lang="sass">
.q-item
  min-height: 0
</style>
