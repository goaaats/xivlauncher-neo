<template>
  <div id="content" class="flex fit justify-center row dalamud-bg">
    <q-carousel ref="carousel" v-model="currentSlide" control-color="primary" animated class="fit transparent"
                transition-prev="slide-right" transition-next="slide-left" transition-duration="500">

      <!-- Language -->
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

      <!-- Game -->
      <q-carousel-slide :name="2" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <p class="q-mb-xs ws-wrap">{{ $t("SetupChooseGamePath") }}</p>
            <div class="row">
              <q-input v-model="gameDir" clearable bottom-slots :label="$t('SetupGameDirectory')"
                       :spellcheck="false" maxlength="1000"
                       class="col-grow" @update:modelValue="onUpdateGameDir">
                <template #append>
                  <q-icon v-if="!gameDir" name="mdi-magnify" class="cursor-pointer" @click="onClickGameDirSearch"/>
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

      <!-- ACT -->
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
                    <q-icon v-if="!actPath" name="mdi-search" class="cursor-pointer" @click="onClickActPathSearch"/>
                  </template>
                </q-input>
              </div>
            </div>
          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <!-- Dalamud -->
      <q-carousel-slide :name="4" class="flex justify-center row q-py-lg translucent">
        <q-card class="col-8">
          <q-card-section>
            <p class="q-mb-xs ws-wrap">{{ $t("SetupDalamudNotice") }}</p>
            <q-checkbox v-model="enableDalamud" :label="$t('SetupDalamudCheckBox')"/>
          </q-card-section>
        </q-card>
      </q-carousel-slide>

      <!-- Controls -->
      <template #control>
        <q-carousel-control position="bottom-right" class="q-gutter-xs">
          <q-btn v-if="currentSlide !== 1" push round color="primary"
                 icon="mdi-arrow-left" @click="$refs.carousel.previous()"/>
          <q-btn v-if="currentSlide !== 4" push round color="primary"
                 icon="mdi-arrow-right" :disabled="currentSlide === 2 && !gameDir"
                 @click="$refs.carousel.next()"/>
          <q-btn v-if="currentSlide === 4" push round color="positive"
                 icon="mdi-check" @click="onCompleteSetup"/>
        </q-carousel-control>
      </template>

    </q-carousel>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref} from 'vue'
import {backend, i18n, store} from '@/services/'
import {MAIN_ROUTE} from '@/services/router'
import {isGamePathValid, showFileDialog} from '@/util'

const config = store.CONFIG.inject()

const currentSlide = ref(1)

// region Language
const launcherLanguageChoice = ref(i18n.getDefaultLauncherLanguageOption())
const gameLanguageChoice = ref(i18n.getDefaultGameLanguageOption())
const launcherLanguageOptions = i18n.getLauncherLanguageOptions()
const gameLanguageOptions = i18n.getGameLanguageOptions()

async function onLauncherLanguageChange(lang: string) {
  const locale = i18n.convertLauncherLanguage(lang)
  await i18n.setLanguage(locale)
}

// endregion

// region Game
const gameDir = ref('')
const gameSubDirsFound = ref(false)
const useSteam = ref(false)

async function onClickGameDirSearch() {
  gameDir.value = await showFileDialog(true)
  gameSubDirsFound.value = false
  await onUpdateGameDir(gameDir.value)
}

async function onUpdateGameDir(path: string | null) {
  gameSubDirsFound.value = await isGamePathValid(path)
}

// endregion

// region ACT
const actPath = ref('')
const foundAct = ref(false)
const enableACT = ref(false)

onMounted(getActPath)

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

// endregion

// region Slide 4
const enableDalamud = ref(false)

// endregion

async function onCompleteSetup() {
  config.value.launcher_language = i18n.convertLauncherLanguage(launcherLanguageChoice.value)
  config.value.game_language = i18n.convertGameLanguage(gameLanguageChoice.value)
  config.value.game_path = gameDir.value
  config.value.enable_steam_integration = useSteam.value
  config.value.enable_dalamud = enableDalamud.value

  if (enableACT.value) {
    config.value.addons.push({
      is_enabled: true,
      path: actPath.value,
      command_line: '',
      run_as_admin: false,
      run_on_close: false,
      kill_after_close: false,
    })
  }

  await backend.saveConfig(config.value)

  await MAIN_ROUTE.push()
}
</script>

<style lang="sass" scoped>
</style>
