<template>
  <div id="content">

    <!-- Banner -->
    <q-card>
      <q-carousel v-model="bannerIndex"
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

        <!-- Placeholder -->
        <q-carousel-slide v-if="!bannerData"
                          :name="0"
                          :img-src="bannerPlaceholder"
                          class="fit q-pa-none"/>
      </q-carousel>
    </q-card>

    <!-- Login -->
    <q-card class="q-px-md q-py-sm">
      <div>
        <q-input v-model="username" dense :label="$t('LoginBoxUsername')" maxlength="100"></q-input>
        <q-input v-model="password" dense :label="$t('LoginBoxPassword')" maxlength="100" type="password"/>
      </div>

      <div class="flex-grow-1">
        <q-checkbox v-model="autoLogin" size="xs" :label="$t('LoginBoxAutoLogin')"/>
        <q-checkbox v-model="useOtp" size="xs" :label="$t('LoginBoxOtp')"/>
        <q-checkbox v-model="useSteam" size="xs" :label="$t('LoginBoxSteam')"/>
      </div>

      <div class="row justify-center">
        <q-btn color="primary" :loading="startingGame" class="col-5" @click="clickLogin">
          <q-tooltip :delay="500"> {{ $t("LoginBoxLoginTooltip") }}</q-tooltip>
          {{ $t("LoginBoxLogin") }}
          <template #loading>
            <q-spinner/>
          </template>
        </q-btn>
        <div class="q-pl-sm"/>
        <q-btn color="primary" class="col-3" @click="clickAccountSwitcher">
          <q-tooltip :delay="500"> {{ $t("OpenAccountSwitcher") }}</q-tooltip>
          <q-icon name="groups"/>
        </q-btn>
      </div>
    </q-card>

    <!-- News -->
    <q-card class="fit">
      <q-scroll-area class="fit">
        <q-list >

          <news-item v-for="(entry, index) in headline.news"
                     :key="index"
                     :entry="entry"/>

          <news-item v-for="(entry, index) in headline.pinned"
                     :key="index"
                     :entry="entry"/>

          <news-item v-for="(entry, index) in headline.topics"
                     :key="index"
                     :entry="entry"/>

        </q-list>
      </q-scroll-area>
    </q-card>

    <!-- Controls -->
    <q-card class="flex fit justify-center content-center">
      <q-btn flat class="control" @click="clickMaintenance">
        <q-tooltip :delay="500">{{ $t("MaintenanceQueue") }}</q-tooltip>
        <q-icon name="mdi-update" size="3rem" color="primary"/>
      </q-btn>
      <q-btn flat class="control" type="a" target="_blank" href="https://is.xivup.com/">
        <q-tooltip :delay="500">{{ $t("WorldStatus") }}</q-tooltip>
        <q-icon name="mdi-earth" size="3rem" color="primary"/>
      </q-btn>
      <q-btn flat color="primary" class="control" @click="clickSettings">
        <q-tooltip :delay="500">{{ $t("Settings") }}</q-tooltip>
        <q-icon name="mdi-cog" size="3rem" color="primary"/>
      </q-btn>
    </q-card>
  </div>
</template>

<script lang="ts" setup>
import {inject, onMounted, Ref, ref} from 'vue'
import {CONFIG_ROUTE} from '@/router/route'
import {backend, constants} from '@/services/'
import {Headline, HeadlineEntry, LauncherSettings} from '@/services/backend'
import NewsItem from '@/components/news-item.vue'

const settings = inject(constants.SETTINGS_KEY) as Ref<LauncherSettings>

// region Headline


// endregion

// region Headline

type BannerData = {
  lsb_banner: string,
  link: string,
  loaded: boolean,
  srcData: string,
}

const headline = ref({}) as Ref<Headline>

const bannerIndex = ref(0)
const bannerPlaceholder = '/static/banner_placeholder.png'
const bannerData = ref([]) as Ref<BannerData[]>

const newsData = ref([]) as Ref<HeadlineEntry[]>

async function getHeadline() {
  headline.value = await backend.getHeadline(settings.value.game_language)

  bannerData.value = headline.value.banner.map((banner) => {
    return {...banner, loaded: false, srcData: ''}
  })

  newsData.value = [].concat(
    headline.value.news,
    headline.value.topics,
    headline.value.pinned,

  )

  for (const banner of bannerData.value) {
    const data = await backend.getBannerImageData(banner.lsb_banner)
    banner.srcData = `data:image;base64, ${data}`
    banner.loaded = true
  }



}
onMounted(getHeadline)

// endregion

const username = ref('')
const password = ref('')
const autoLogin = ref(true)
const useOtp = ref(true)
const useSteam = ref(true)
const startingGame = ref(false)


function clickLogin() {
  startingGame.value = true
}

function clickAccountSwitcher() {
  console.log('clickAccountSwitcher')
}

function clickMaintenance() {
  console.log('clickMaintenance')
}

async function clickSettings() {
  await CONFIG_ROUTE.push()
}

</script>

<style lang="sass" scoped>
$window-width: 830px
$window-height: 340px

// 640 x 250
$image-ratio: 2.56
$image-width: 520px
$image-height: $image-width / $image-ratio

$padding: 10px
$table-gap: 10px

$left-col: $image-width
$right-col: $window-width - $left-col - $table-gap - ($padding * 2)
$top-row: $image-height
$bottom-row: $window-height - $top-row - $table-gap - ($padding * 2)

#content
  display: grid
  padding: $padding
  row-gap: $table-gap
  column-gap: $table-gap
  grid-template-columns: $left-col $right-col
  grid-template-rows: $top-row $bottom-row

.control
  border-radius: 5px
  width: 77px
  height: 77px

.q-checkbox
  height: 20px
</style>
