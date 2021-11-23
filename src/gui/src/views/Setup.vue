<template>
  <div id="content" class="flex fit justify-center row dalamud-bg">
    <q-carousel ref="carousel" v-model="currentSlide" control-color="primary" animated class="fit transparent"
                transition-prev="slide-right" transition-next="slide-left" transition-duration="500">

      <q-carousel-slide :name="1" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseLauncherLanguage") }}</p>
              <q-select v-model="launcherLanguageChoice" filled options-dense
                        :options="launcherLanguageOptions" :label="$t('SetupLauncherLanguage')"
                        behavior="dialog" @update:model-value="onLauncherLanguageChange"/>
            </div>
            <br/>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseGameLanguage") }}</p>
              <q-select v-model="gameLanguageChoice" filled options-dense
                        :options="gameLanguageOptions"
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
              <q-input v-model="gameDir" clearable bottom-slots :label="$t('SetupGameDirectory')"
                       :spellcheck="false" maxlength="1000"
                       class="col-grow" @update:modelValue="onUpdateGameDir">
                <template #append>
                  <q-icon v-if="!gameDir" name="search" class="cursor-pointer" @click="onClickGameDirSearch"/>
                </template>
                <template #hint>
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

      <q-carousel-slide v-if="foundAct" :name="3" class="flex justify-center row q-py-lg translucent">
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
                <q-input v-model="actPath" clearable :label="$t('SetupActPath')" maxlength="1000" class="col-grow">
                  <template #append>
                    <q-icon v-if="!actPath" name="search" class="cursor-pointer" @click="onClickActPathSearch"/>
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

      <template #control>
        <q-carousel-control position="bottom-right" class="q-gutter-xs">
          <q-btn v-if="currentSlide !== 1" push round color="primary"
                 icon="arrow_left" @click="$refs.carousel.previous()"/>
          <q-btn v-if="currentSlide !== 4" push round color="primary"
                 icon="arrow_right" :disabled="currentSlide === 2 && !gameDir"
                 @click="$refs.carousel.next()"/>
          <q-btn v-if="currentSlide === 4" push round color="positive"
                 icon="done" @click="onCompleteSetup"/>
        </q-carousel-control>
      </template>

    </q-carousel>
  </div>
</template>

<script lang="ts">
import {inject, onMounted, Ref, ref} from 'vue'
import {dialog, fs} from '@tauri-apps/api'
import router from '@/router'
import * as i18n from '@/services/i18n'
import * as backend from '@/services/backend'
import * as constants from '@/services/constants'
import * as route from '@/router/route'

export default {
  name: 'SetupView',
  setup() {
    const settings = inject(constants.SETTINGS_KEY) as Ref<backend.LauncherSettings>
    const addons = inject(constants.ADDONS_KEY) as Ref<backend.AddonEntry[]>

    // Current slide
    const currentSlide = ref(1)

    // Slide 1
    const launcherLanguageChoice = ref(i18n.getDefaultLauncherLanguageOption())
    const gameLanguageChoice = ref(i18n.getDefaultGameLanguageOption())
    const launcherLanguageOptions = i18n.getLauncherLanguageOptions()
    const gameLanguageOptions = i18n.getGameLanguageOptions()

    async function onLauncherLanguageChange(lang: string) {
      const locale = i18n.convertLauncherLanguage(lang)
      await i18n.setLanguage(locale)
    }

    // Slide 2
    const gameDir = ref('')
    const gameSubDirsFound = ref(false)
    const useSteam = ref(false)

    async function onClickGameDirSearch() {
      gameDir.value = await showFileDialog(true)
      gameSubDirsFound.value = false
      await onUpdateGameDir(gameDir.value)
    }

    async function onUpdateGameDir(path: string | null) {
      if (!path) {
        gameSubDirsFound.value = false
        return
      }

      gameSubDirsFound.value = await fs.readDir(path).then((children) => {
        const bootExists = children.some((child) => child.name === 'boot')
        const gameExists = children.some((child) => child.name === 'game')
        return bootExists && gameExists
      }).catch(() => false)
    }

    // Slide 3
    const actPath = ref('')
    const foundAct = ref(false)
    const enableACT = ref(false)

    onMounted(async () => await getActPath())

    async function getActPath() {
      const path = await backend.getAdvancedCombatTrackerPath()
      if (path) {
        foundAct.value = true
        actPath.value = path
      }
    }

    async function onClickActPathSearch() {
      actPath.value = await showFileDialog(false)
    }

    // Slide 4
    const enableDalamud = ref(false)

    // Shared
    async function showFileDialog(dirOnly: boolean): Promise<string> {
      const result = await dialog.open({
        directory: dirOnly,
        multiple: false,
      })

      // This should never happen with multiple: false
      if (Array.isArray(result))
        throw 'Invalid result'

      return result
    }

    async function onCompleteSetup() {
      settings.value.launcher_language = i18n.convertLauncherLanguage(launcherLanguageChoice.value)
      settings.value.game_language = i18n.convertGameLanguage(gameLanguageChoice.value)
      settings.value.game_path = gameDir.value
      settings.value.enable_steam_integration = useSteam.value
      settings.value.enable_dalamud = enableDalamud.value
      await backend.setSettings(settings.value)

      if (enableACT.value) {
        addons.value.push({
          is_enabled: true,
          path: actPath.value,
          command_line: '',
          run_as_admin: false,
          run_on_close: false,
          kill_after_close: false,
        })
        await backend.setAddons(addons.value)
      }

      await router.push(route.MAIN_ROUTE)
    }

    return {
      t: i18n.t, currentSlide,
      launcherLanguageChoice, launcherLanguageOptions, onLauncherLanguageChange,
      gameLanguageChoice, gameLanguageOptions,
      gameDir, gameSubDirsFound, onClickGameDirSearch, onUpdateGameDir,
      useSteam,
      actPath, foundAct, enableACT, onClickActPathSearch,
      enableDalamud,
      onCompleteSetup,
    }
  },
}
</script>

<style lang="sass" scoped>
</style>
