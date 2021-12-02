<template>
  <q-card class="fit">
    <q-scroll-area class="fit">

      <q-list v-if="headlineData.length">
        <q-item v-for="(entry, index) in headlineData"
                :key="index"
                v-ripple clickable
                tag="a" :href="entry.url" target="_blank"
                style="padding: 7px 4px; min-height: 0">
          <q-item-section side class="q-pr-sm">
            <q-icon v-if="entry.tag === 'Important'"
                    name="mdi-alert-circle-outline"
                    size="xs"/>
            <q-icon v-else-if="entry.tag === 'Follow-up'"
                    name="mdi-information-outline"
                    size="xs"/>
            <q-icon v-else-if="entry.tag === 'Maintenance'"
                    name="mdi-wrench-clock"
                    size="xs"/>
            <q-icon v-else-if="entry.tag === 'Recovery'"
                    name="mdi-check-circle"
                    size="xs"/>
            <q-icon v-else-if="entry.type === 'topic'"
                    name="mdi-forum"
                    size="xs"/>
            <q-icon v-else-if="entry.type === 'error'"
                    name="mdi-lan-disconnect"
                    type="xs"/>
            <q-icon v-else
                    name="mdi-newspaper"
                    size="xs"/>
          </q-item-section>

          <q-item-label :lines="1"
                        style="font-size: 0.85rem"
                        class="flex flex-center">
            {{ entry.title }}
          </q-item-label>
        </q-item>

      </q-list>

      <q-list v-else>
        <q-item v-for="index in Array(3).keys()"
                :key="index"
                style="padding: 7px 4px; min-height: 0">
          <q-item-section side class="q-pr-sm">
            <q-skeleton type="rect" animation="none"
                        width="18px" height="18px"/>
          </q-item-section>

          <q-item-section>
            <q-skeleton type="text" animation="none"
                        height="18px"/>
          </q-item-section>
        </q-item>
      </q-list>

    </q-scroll-area>
  </q-card>
</template>

<script lang="ts" setup>
import {onMounted, Ref, ref} from 'vue'
import {Headline, HeadlineEntry} from '@/services/backend'

const props = defineProps<{
  headline: Promise<Headline>,
}>()

const headlineData = ref([]) as Ref<HeadlineData[]>

type HeadlineData = HeadlineEntry & {
  type: string
}

onMounted(loadHeadlineData)

async function loadHeadlineData() {
  const headline = await props.headline

  function mapHeadline(entry: HeadlineEntry, type: string): HeadlineData {
    return {...entry, type}
  }

  headlineData.value = ([] as HeadlineData[]).concat(
    headline.news.map(e => mapHeadline(e, 'news')),
    headline.pinned.map(e => mapHeadline(e, 'pinned')),
    headline.topics.map(e => mapHeadline(e, 'topic')),
  ).sort((e1, e2) => e2.date.localeCompare(e1.date))
}

</script>

<style lang="sass" scoped>
</style>
