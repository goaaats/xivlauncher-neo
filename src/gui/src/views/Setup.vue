<template>
  <div id="content" class="flex fit justify-center row">
    <q-carousel ref="carousel" v-model="slide" control-color="primary"
                animated class="fit transparent"
                transition-prev="slide-right" transition-next="slide-left" transition-duration="500">

      <q-carousel-slide :name="1" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseLauncherLanguage") }}</p>
              <q-select filled options-dense
                        @update:model-value="onLauncherLanguageChange"
                        v-model="launcherLanguage" :options="launcherLanguages"
                        :label="$t('SetupLauncherLanguage')" behavior="dialog"/>
            </div>
            <br/>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseGameLanguage") }}</p>
              <q-select filled options-dense
                        v-model="gameLanguage" :options="gameLanguages"
                        :label="$t('SetupGameLanguage')" behavior="dialog"/>
            </div>
          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <q-carousel-slide :name="2" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseGamePath") }}</p>
            <div class="row">
              <q-input clearable v-model="gameDir" :label="$t('SetupGameDirectory')" maxlength="1000" class="col-grow">
                <template v-slot:append>
                  <q-icon name="search" @click="clickGameDirSearch" v-if="!gameDir" class="cursor-pointer"/>
                </template>
              </q-input>
            </div>
            <br/>
            <p class="q-mb-xs ws-wrap">{{ $t("SetupSteamNotice") }}</p>
            <q-checkbox v-model="useSteam" :label="$t('SetupSteamCheckBox')"/>
          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <q-carousel-slide :name="3" class="flex justify-center row q-py-lg translucent" v-if="foundAct">
        <q-card class="col-8">
          <q-card-section>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupActDetected") }}</p>
              <q-checkbox v-model="enableACT" :label="$t('SetupActLaunchCheckBox')"/>
            </div>
            <br/>
            <div v-if="enableACT">
              <p class="q-mb-xs ws-wrap">{{ $t("SetupActPathNotice") }}</p>
              <div class="row">
                <q-input clearable v-model="actPath" :label="$t('SetupActPath')" maxlength="1000" class="col-grow">
                  <template v-slot:append>
                    <q-icon name="search" @click="clickActPathSearch" v-if="!actPath" class="cursor-pointer"/>
                  </template>
                </q-input>
              </div>
            </div>

          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <q-carousel-slide :name="4" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <p class="q-mb-xs ws-wrap">{{ $t("SetupDalamudNotice") }}</p>
            <q-checkbox v-model="enableDalamud" :label="$t('SetupDalamudCheckBox')"/>
          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <template v-slot:control>
        <q-carousel-control position="bottom-right" class="q-gutter-xs">
          <q-btn push round color="primary" icon="arrow_left" @click="$refs.carousel.previous()" v-if="slide !== 1" transition-show="jump-down" transition-hide="jump-up"/>
          <q-btn push round color="primary" icon="arrow_right" @click="$refs.carousel.next()" v-if="slide !== 4"/>
          <q-btn push round color="positive" icon="done" @click="onCompleteSetup" v-if="slide === 4"/>
        </q-carousel-control>
      </template>

    </q-carousel>
  </div>
</template>

<script lang="ts">
import {ref} from "vue"
import {useI18n} from "vue-i18n"
import {
  QBtn, QCard, QCardSection,
  QCarousel, QCarouselControl, QCarouselSlide,
  QCheckbox, QIcon, QInput, QSelect,
} from "quasar"
import {dialog} from "@tauri-apps/api"
import {setLanguage} from "@/services/i18n"
import {getAdvancedCombatTrackerPath, getSystemLocale} from "@/services/backend"

const slide = ref(1)
const launcherLanguage = ref("English")
const gameLanguage = ref("English")
const systemLocale = ref("en-us")
const gameDir = ref("")
const useSteam = ref(false)
const actPath = ref("")
const foundAct = ref(false)
const enableACT = ref(false)
const enableDalamud = ref(false)

const launcherLanguageChoices: { [k: string]: string } = {
  "ja": "日本語",
  "en": "English",
  "de": "Deutsch",
  "fr": "Français",
  "it": "Italiano",
  "es": "Español",
  "pt": "Português",
  "ko": "한국어",
  "no": "Norsk",
  "ru": "русский",
  "zh": "简体中文",
}
const gameLanguageChoices: { [k: string]: string } = {
  "jp": "日本語",
  "en": "English",
  "de": "Deutsch",
  "fr": "Français",
}

function getKeyByValue(object: { [key: string]: string }, value: string): string {
  return Object.keys(object).find(key => object[key] === value) || "en"
}

async function clickGameDirSearch() {
  gameDir.value = await showFileDialog(true)
}

async function clickActPathSearch() {
  actPath.value = await showFileDialog(false)
}

async function showFileDialog(dirOnly: boolean): Promise<string> {
  const result = await dialog.open({
    directory: dirOnly,
    multiple: false,
  })

  // This should never happen with multiple: false
  if (Array.isArray(result))
    throw "Invalid result"

  return result
}

async function onLauncherLanguageChange(newLang: string) {
  const code = getKeyByValue(launcherLanguageChoices, newLang)
  await setLanguage(code)
}

async function onCompleteSetup() {
  let gameLang = getKeyByValue(gameLanguageChoices, gameLanguage.value)
  if (gameLang === "en") {
    if (systemLocale.value.endsWith("-us") ||
        systemLocale.value.endsWith("-ca") ||
        systemLocale.value.endsWith("-mx")) {
      gameLang = "en-us"
    } else {
      gameLang = "en-gb"
    }
  }

  const launcherLang = getKeyByValue(launcherLanguageChoices, launcherLanguage.value)
  console.log(`DONE game=${gameLang} launcher=${launcherLang}`)

  // TODO: These all get passed <somewhere> to init the configuration
  // const _gameDir = gameDir.value
  // const _useSteam = useSteam.value
  // const _enableACT = enableACT.value
  // const _actPath = actPath.value
  // const _enableDalamud = enableDalamud.value
}

export default {
  name: "setup-view",
  components: {
    QBtn, QCard, QCardSection,
    QCarousel, QCarouselControl, QCarouselSlide,
    QCheckbox, QIcon, QInput, QSelect,
  },
  methods: {},
  setup() {
    const t = useI18n()

    getSystemLocale().then(async (sysLoc) => {
      systemLocale.value = sysLoc
      const loc = sysLoc.split("-")[0]

      if (loc in launcherLanguageChoices) {
        launcherLanguage.value = launcherLanguageChoices[loc]
        await setLanguage(loc)
      }

      if (loc in gameLanguageChoices) {
        gameLanguage.value = gameLanguageChoices[loc]
      }
    })

    getAdvancedCombatTrackerPath().then(path => {
      if (path) {
        foundAct.value = true
        actPath.value = path
      }
    })

    return {
      t, onLauncherLanguageChange,
      clickGameDirSearch, clickActPathSearch,
      slide, onCompleteSetup,
      launcherLanguage, gameLanguage,
      launcherLanguages: Object.values(launcherLanguageChoices),
      gameLanguages: Object.values(gameLanguageChoices),
      gameDir, useSteam, actPath, foundAct, enableACT, enableDalamud,
    }
  },
}
</script>

<style lang="sass" scoped>
#content
  //noinspection CssUnknownTarget
  background: url("/static/logo.png") no-repeat fixed center
  background-size: contain

.translucent
  opacity: .9

.ws-wrap
  white-space: pre-wrap

</style>