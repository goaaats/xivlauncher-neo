<template>
  <q-card>
    <q-carousel v-if="bannerData.length"
                v-model="bannerIndex"
                transition-prev="slide-right"
                transition-next="slide-left"
                animated infinite arrows
                :autoplay="5000"
                class="fit">
      <q-carousel-slide v-for="(banner, index) in bannerData"
                        :key="index" :name="index"
                        class="fit q-pa-none">

        <a v-if="banner.loaded"
           :href="banner.link"
           target="_blank">
          <q-img :src="banner.srcData"/>
        </a>

        <q-skeleton v-else type="rect" class="fit"/>

      </q-carousel-slide>
    </q-carousel>

    <q-carousel v-else :model-value="0" class="fit">
      <q-carousel-slide :name="0"
                        img-src="/static/banner_placeholder.png"
                        class="fit q-pa-none"/>
    </q-carousel>
  </q-card>
</template>

<script lang="ts" setup>
import {onMounted, Ref, ref} from 'vue'
import {backend} from '@/services/'
import {BannerEntry, Headline} from '@/services/backend'

const props = defineProps<{
  headline: Promise<Headline>,
}>()

const bannerIndex = ref(0)
const bannerData = ref([]) as Ref<BannerData[]>

type BannerData = BannerEntry & {
  loaded: boolean,
  srcData: string,
}

onMounted(loadBannerData)

async function loadBannerData() {
  const headline = await props.headline

  bannerData.value = headline.banner.map((banner) => {
    return {...banner, loaded: false, srcData: ''}
  })

  for (const banner of bannerData.value) {
    const data = await backend.getBannerImageData(banner.lsb_banner)
    banner.srcData = `data:image;base64, ${data}`
    banner.loaded = true
  }
}

</script>

<style lang="sass" scoped>
</style>
