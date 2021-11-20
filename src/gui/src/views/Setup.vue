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
                        v-model="launcherLanguage" :options="launcherLanguageOptions"
                        :label="$t('SetupLauncherLanguage')" behavior="dialog"/>
            </div>
            <br/>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseGameLanguage") }}</p>
              <q-select filled options-dense
                        v-model="gameLanguage" :options="gameLanguageOptions"
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
              <q-input clearable bottom-slots :label="$t('SetupGameDirectory')" :spellcheck="false"
                       v-model="gameDir" @update:modelValue="onGameDirUpdate"
                       maxlength="1000" class="col-grow">
                <template v-slot:append>
                  <q-icon name="search" @click="clickGameDirSearch" v-if="!gameDir" class="cursor-pointer"/>
                </template>
                <template v-slot:hint>
                  <p v-if="gameDir && !gameSubDirsFound" class="text-warning">
                    {{ $t("SetupGameDirectoryWarning") }}
                  </p>
                  <p v-if="gameDir && gameSubDirsFound" class="text-positive">
                    {{ $t("SetupGameDirectoryCorrect") }}
                  </p>
                </template>
              </q-input>
            </div>
            <br/>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupSteamNotice") }}</p>
              <q-checkbox v-model="useSteam" :label="$t('SetupSteamCheckBox')"/>
            </div>
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
          <q-btn push round color="primary" icon="arrow_left"
                 @click="$refs.carousel.previous()" v-if="slide !== 1"/>
          <q-btn push round color="primary" icon="arrow_right"
                 @click="$refs.carousel.next()" v-if="slide !== 4"
                 :disabled="slide === 2 && !gameDir"/>
          <q-btn push round color="positive" icon="done"
                 @click="onCompleteSetup" v-if="slide === 4"/>
        </q-carousel-control>
      </template>

    </q-carousel>
  </div>
</template>

<script lang="ts">
import {ref} from "vue"
import {useI18n} from "vue-i18n"
import {QBtn, QCard, QCardSection, QCarousel, QCarouselControl, QCarouselSlide, QCheckbox, QIcon, QInput, QSelect} from "quasar"
import {dialog, fs} from "@tauri-apps/api"
import {setLanguage} from "@/services/i18n"
import {getAddons, getAdvancedCombatTrackerPath, getSettings, getSystemLocale, setAddons, setSettings} from "@/services/backend"
import {useRouter} from "vue-router"

const slide = ref(1)
const launcherLanguage = ref("English")
const gameLanguage = ref("English")
const gameDir = ref("")
const gameSubDirsFound = ref(false)
const useSteam = ref(false)
const actPath = ref("")
const foundAct = ref(false)
const enableACT = ref(false)
const enableDalamud = ref(false)

const launcherLanguageChoices: { [k: string]: string } = {
  "Japanese": "日本語",
  "English": "English",
  "German": "Deutsch",
  "French": "Français",
  "Italian": "Italiano",
  "Spanish": "Español",
  "Portuguese": "Português",
  "Korean": "한국어",
  "Norwegian": "Norsk",
  "Russian": "русский",
  "SimplifiedChinese": "简体中文",
}
const gameLanguageChoices: { [k: string]: string } = {
  "Japanese": "日本語",
  "English": "English",
  "German": "Deutsch",
  "French": "Français",
}

function getKeyByValue(object: { [key: string]: string }, value: string): string {
  return Object.keys(object).find(key => object[key] === value) || "en"
}

export default {
  name: "setup-view",
  components: {
    QBtn, QCard, QCardSection,
    QCarousel, QCarouselControl, QCarouselSlide,
    QCheckbox, QIcon, QInput, QSelect,
  },
  setup() {
    const t = useI18n()
    const router = useRouter()

    getSystemLocale().then(async (systemLocale) => {
      const loc = systemLocale.split("-")[0]

      // Default to English
      if (loc in launcherLanguageChoices) {
        launcherLanguage.value = launcherLanguageChoices[loc]
        await setLanguage(loc)
      }

      // Default to English
      if (loc in gameLanguageChoices) {
        gameLanguage.value = gameLanguageChoices[loc]
      }
    })

    const launcherLanguageOptions = Object.values(launcherLanguageChoices)
    const gameLanguageOptions = Object.values(gameLanguageChoices)

    getAdvancedCombatTrackerPath().then(path => {
      if (path) {
        foundAct.value = true
        actPath.value = path
      }
    })

    async function clickGameDirSearch() {
      gameDir.value = await showFileDialog(true)
      gameSubDirsFound.value = false
      await onGameDirUpdate(gameDir.value)
    }

    async function onGameDirUpdate(path: string | null) {
      if (!path) {
        gameSubDirsFound.value = false
        return
      }

      gameSubDirsFound.value = await fs.readDir(path).then((children) => {
        const bootExists = children.some((child) => child.name === "boot")
        const gameExists = children.some((child) => child.name === "game")
        return bootExists && gameExists
      }).catch(() => false)
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
      const launcherLang = getKeyByValue(launcherLanguageChoices, launcherLanguage.value)
      console.log(`DONE game=${gameLang} launcher=${launcherLang}`)

      const settings = await getSettings()
      settings.launcher_language = launcherLanguage.value
      settings.client_language = gameLanguage.value
      settings.game_path = gameDir.value
      settings.enable_steam_integration = useSteam.value
      settings.enable_dalamud = enableDalamud.value
      await setSettings(settings)

      if (enableACT.value) {
        const addons = await getAddons()
        addons.push({
          is_enabled: true,
          path: actPath.value,
          command_line: "",
          run_as_admin: false,
          run_on_close: false,
          kill_after_close: false,
        })
        await setAddons(addons)
      }

      await router.push("/main")
    }

    return {
      t, onLauncherLanguageChange,
      clickGameDirSearch, onGameDirUpdate,
      clickActPathSearch, onCompleteSetup,
      slide, launcherLanguage, gameLanguage,
      launcherLanguageOptions, gameLanguageOptions,
      gameDir, gameSubDirsFound, useSteam,
      actPath, foundAct, enableACT,
      enableDalamud,
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