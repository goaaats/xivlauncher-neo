<template>
  <q-layout id="content" view="hhh lpr fff">
    <q-header elevated class="flex row">
      <q-tabs v-model="currentTab" dense :no-caps="true">
        <q-tab :name="1" :label="$t('ConfigGame')"/>
        <q-tab :name="2" :label="$t('ConfigLanguage')"/>
        <q-tab :name="3" :label="$t('ConfigAutoLaunch')"/>
        <q-tab :name="4" :label="$t('ConfigInGame')"/>
        <q-tab :name="5" :label="$t('ConfigPlugins')"/>
        <q-tab :name="6" :label="$t('ConfigPatching')"/>
        <q-tab :name="7" :label="$t('ConfigAbout')"/>
      </q-tabs>
    </q-header>

    <q-page-container>
      <q-page class="flex row dalamud-bg">

        <q-tab-panels v-model="currentTab" animated class="flex-grow-1 translucent">

          <!-- Game -->
          <q-tab-panel :name="1" class="column">
            <span class="ws-wrap">{{ $t("ConfigChooseGamePath") }}</span>
            <div>
              <q-input v-model="config.game_path" clearable bottom-slots :label="$t('ConfigGameDirectory')"
                       :spellcheck="false" maxlength="1000"
                       class="col-grow" @update:modelValue="onUpdateGameDir">
                <template #append>
                  <q-icon v-if="!config.game_path" name="mdi-magnify" class="cursor-pointer" @click="onClickGameDirSearch"/>
                </template>
                <template #hint>
                  <p v-if="config.game_path && !gameSubDirsFound" class="text-warning">
                    {{ $t("ConfigGameDirectoryWarning") }}
                  </p>
                  <p v-if="config.game_path && gameSubDirsFound" class="text-positive">
                    {{ $t("ConfigGameDirectoryCorrect") }}
                  </p>
                </template>
              </q-input>
            </div>

            <div class="row q-pb-sm">
              <q-input v-model="config.extra_game_args" clearable
                       :label="$t('ConfigGameAdditionalArguments')"
                       :spellcheck="false"
                       maxlength="1000"
                       class="col-grow">
              </q-input>
            </div>

            <div class="row full-width no-wrap">
              <div class="column col-md-8" style="flex: 1">
                <q-checkbox v-model="config.enable_steam_integration" dense
                            class="q-pb-sm"
                            :label="$t('ConfigGameSteam')"/>
                <q-checkbox v-model="config.enable_otp_server" dense
                            class="q-pb-sm"
                            :label="$t('ConfigGameOtpServer')"/>
                <q-checkbox v-model="config.encrypt_args" dense
                            class="q-pb-sm"
                            :label="$t('ConfigGameArgEncryption')"/>
                <div class="q-pb-sm">
                  <q-checkbox v-model="config.enable_uid_cache" dense
                              class="q-pr-sm"
                              :label="$t('ConfigGameUidCache')"/>
                  <q-btn icon="mdi-lock-reset" size="sm" :no-caps="true" dense
                         :label="$t('ConfigGameUidCacheReset')">
                    <q-tooltip :delay="500">{{ $t("ConfigGameUidCacheHint") }}</q-tooltip>
                  </q-btn>
                </div>

                <div>
                  <q-btn :no-caps="true" color="primary" :label="$t('ConfigGameIntegrityCheck')">
                    <q-tooltip :delay="500">{{ $t('ConfigGameIntegrityCheckHint') }}</q-tooltip>
                  </q-btn>
                </div>
              </div>
              <div class="column col-md-4" style="flex: 1">
                <span>{{ $t('ConfigGameDirectX') }}</span>
                <q-option-group v-model="config.use_dx11"
                                :options="directXOptions"
                                color="primary" inline dense/>
                <p v-if="!config.use_dx11" class="text-warning ">
                  {{ $t('ConfigGameDirectX9Warning') }}
                </p>
              </div>
            </div>
            <div>

            </div>
          </q-tab-panel>

          <!-- Language -->
          <q-tab-panel :name="2" class="column">
            <span>{{ $t("ConfigChooseLauncherLanguage") }}</span>
            <q-select v-model="launcherLanguageChoice" filled options-dense
                      :options="launcherLanguageOptions"
                      :label="$t('ConfigLauncherLanguage')"
                      behavior="dialog"
                      @update:model-value="onLauncherLanguageChange"/>
            <br/>

            <span>{{ $t("ConfigChooseGameLanguage") }}</span>
            <q-select v-model="gameLanguageChoice" filled options-dense
                      :options="gameLanguageOptions"
                      :label="$t('ConfigGameLanguage')"
                      behavior="dialog"
                      @update:model-value="onGameLanguageChange"/>
            <br/>

            <a href="https://crowdin.com/project/ffxivquicklauncher" target="_blank" class="a-none">
              {{ $t('ConfigLocalizationHelp') }}
            </a>
          </q-tab-panel>

          <!-- Addons -->
          <q-tab-panel :name="3" class="column q-pb-sm">
            <div class="q-pb-xs">
              {{ $t('ConfigAddonNotice') }}
            </div>

            <q-card class="bg-grey-9 flex-grow-1">
              <q-scroll-area class="fit">
                <q-list separator>
                  <q-item v-for="(item, index) in config.addons"
                          :key="index" v-ripple clickable dense
                          class="q-px-none" @click="onClickEditAddon(item)">
                    <q-item-label>
                      <q-toggle v-model="item.is_enabled" tabindex="-1"/>
                    </q-item-label>
                    <q-item-section>
                      <div v-if="item.is_enabled">
                        {{ item.path }}
                      </div>
                      <div v-if="!item.is_enabled" style="text-decoration: line-through">
                        {{ item.path }}
                      </div>
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-scroll-area>
            </q-card>

            <div class="row justify-center q-pt-sm q-pb-xs">
              <q-btn color="secondary" :no-caps="true" class="q-py-none"
                     :label="$t('ConfigAddonAddNew')"
                     @click="onClickAddAddon">
                <q-tooltip :delay="500">{{ $t("ConfigAddonAddNewHint") }}</q-tooltip>
              </q-btn>
            </div>

            <q-dialog v-model="showEditAddonDialog" class="" transition-show="fade" transition-hide="fade">
              <q-card class="bg-grey-9 full-width">
                <q-card-section>
                  <q-input v-model="editAddonDialogItem.path" clearable
                           :label="$t('ConfigAddonApplicationPath')"
                           :spellcheck="false" maxlength="1000" class="col-grow">
                    <template #append>
                      <q-icon v-if="!editAddonDialogItem.path" name="mdi-search" class="cursor-pointer"
                              @click="onClickAppPathSearch"/>
                    </template>
                  </q-input>
                  <q-input v-model="editAddonDialogItem.command_line" clearable
                           :label="$t('ConfigAddonCommandLine')"
                           :spellcheck="false" maxlength="1000" class="col-grow">
                  </q-input>
                  <div class="column q-pt-md q-gutter-sm">
                    <q-checkbox v-model="editAddonDialogItem.run_as_admin" dense
                                :label="$t('ConfigAddonRunAsAdmin')"/>
                    <q-checkbox v-model="editAddonDialogItem.run_on_close" dense
                                :label="$t('ConfigAddonRunOnClose')"/>
                    <q-checkbox v-model="editAddonDialogItem.kill_after_close" dense
                                :label="$t('ConfigAddonKillAfterClose')"/>
                  </div>

                </q-card-section>
                <q-card-actions align="right">
                  <q-btn v-close-popup :no-caps="true" color="positive"
                         :label="$t('ConfigAddonSave')"
                         @click="onClickEditAddonSave"/>
                  <q-btn v-if="!showEditAddonDialogNew" v-close-popup :no-caps="true" color="negative"
                         :label="$t('ConfigAddonDelete')"
                         @click="onClickEditAddonDelete"/>
                </q-card-actions>
              </q-card>
            </q-dialog>
          </q-tab-panel>

          <!-- Dalamud -->
          <q-tab-panel :name="4" class="column">
            <p>{{ $t("ConfigDalamudNotice") }}</p>
            <div>
              <q-checkbox v-model="config.enable_dalamud" dense
                          :label="$t('ConfigDalamudEnable')"/>
            </div>

            <div v-if="config.enable_dalamud">
              <span>{{ $t("ConfigDalamudInGameHint") }}</span>
              <q-input v-model.number="config.dalamud_injection_delay_ms" type="number" style="max-width: 150px"
                       :label="$t('ConfigDalamudInjectionDelay') + ' (ms)'"
                       @wheel="onWheelInjectionDelay"/>
              <br/>
              <span>{{ $t("ConfigUniversalisHint") }}</span>
              <q-checkbox v-model="config.opt_out_mb_collection" dense
                          :label="$t('ConfigUniversalisOptOut')"/>
            </div>
          </q-tab-panel>

          <!-- Plugins -->
          <q-tab-panel :name="5" class="column">
            <span>{{ $t('ConfigPluginsDescription') }}</span>
            <span>{{ $t("ConfigPluginsHint") }}</span>
            <q-card class="bg-grey-9 flex-grow-1">
              <q-scroll-area class="fit">
                <q-list separator>
                  <q-item v-for="(item, index) in plugins"
                          :key="index" dense class="q-px-none">
                    <q-item-label>
                      <q-toggle v-model="item.manifest.Disabled"
                                :true-value="false" :false-value="true"
                                tabindex="-1" @update:model-value="updatePlugin(item)"/>
                    </q-item-label>
                    <q-item-section>
                      <div v-if="!item.manifest.Disabled">
                        {{ item.plugin.name }} {{ item.plugin.version }}
                      </div>
                      <div v-if="item.manifest.Disabled" style="text-decoration: line-through">
                        {{ item.plugin.name }} {{ item.plugin.version }}
                      </div>
                    </q-item-section>

                    <q-menu context-menu>
                      <q-btn v-close-popup color="negative" icon="mdi-delete" :no-caps="true"
                             :label="$t('ConfigPluginDelete')"
                             @click="removePlugin(item)"/>
                    </q-menu>
                  </q-item>
                </q-list>
              </q-scroll-area>
            </q-card>
          </q-tab-panel>

          <!-- Patching -->
          <q-tab-panel :name="6" class="column">
            <div>
              <q-checkbox v-model="config.ask_before_patching" dense class="q-mb-sm"
                          :label="$t('ConfigPatchAskBefore')"/>
            </div>

            <div>
              <q-checkbox v-model="config.keep_patches" dense
                          :label="$t('ConfigPatchKeepAfter')"/>
            </div>

            <div class="row q-mb-sm">
              <q-input v-model="config.patch_path"
                       clearable
                       :label="$t('ConfigPatchPath')"
                       :spellcheck="false" maxlength="1000"
                       class="col-grow">
                <template #append>
                  <q-icon v-if="!config.patch_path" name="mdi-search" class="cursor-pointer" @click="onClickPatchDirSearch"/>
                </template>
              </q-input>
            </div>

            <q-input v-model.number="downloadSpeedLimitMBs" type="number" style="max-width: 200px"
                     :label="$t('ConfigPatchSpeedLimit') + ' (MB/s)'"
                     @wheel="onWheelDownloadSpeedLimit"/>
          </q-tab-panel>

          <!-- About -->
          <q-tab-panel :name="7" class="row">
            <div class="column col-8">
              <div class="ws-wrap">{{ $t("ConfigCredits") }}</div>
              <a href="https://github.com/goaaats/xivlauncher-neo/blob/master/LICENSE"
                 target="_blank" class="a-none"> {{ $t("ConfigLicense") }}</a>
            </div>

            <div class="column col-4">
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-discord"
                     :label="$t('ConfigJoinDiscord')" type="a" target="_blank"
                     href="https://discord.gg/3NMcUV5"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-github"
                     label="GitHub" type="a" target="_blank"
                     href="https://github.com/goaaats/FFXIVQuickLauncher"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-information"
                     :label="$t('ConfigOpenFaq')" type="a" target="_blank"
                     href="https://goatcorp.github.io/faq/"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-wrench"
                     :label="$t('ConfigStartBackupTool')"
                     @click="onClickStartBackupTool"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-launch"
                     :label="$t('ConfigStartOriginalLauncher')"
                     @click="showStartOriginalLauncherDialog=true"/>
            </div>

            <q-dialog v-model="showStartOriginalLauncherDialog" transition-show="fade" transition-hide="fade">
              <q-card class="bg-grey-9">
                <q-card-actions align="right" class="justify-center">
                  <q-btn v-close-popup class="bg-primary" :no-caps="true" style="width: 100px"
                         icon="mdi-steam" label="Steam"
                         @click="onClickStartOriginalLauncher(true)"/>
                  <q-btn v-close-popup class="bg-primary" :no-caps="true" style="width: 100px"
                         icon="mdi-gamepad-variant" :label="$t('ConfigLaunchOriginalNormal')"
                         @click="onClickStartOriginalLauncher(false)"/>
                </q-card-actions>
              </q-card>
            </q-dialog>
          </q-tab-panel>

        </q-tab-panels>

        <q-page-sticky position="bottom-right" :offset="[9,9]">
          <div class="q-gutter-sm">
            <q-btn push round color="primary" icon="mdi-folder-open"
                   @click="openDalamudPluginDir">
              <q-tooltip :delay="500">
                {{ $t("ConfigOpenPluginsFolder") }}
              </q-tooltip>
            </q-btn>

            <q-btn push round color="primary" icon="mdi-check"
                   @click="onCompleteConfig">
              <!-- Set the anchor or it will clip into the corner -->
              <q-tooltip :delay="500" anchor="top left" self="bottom middle">
                {{ $t("ConfigSaveSettings") }}
              </q-tooltip>
            </q-btn>
          </div>
        </q-page-sticky>

      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import {inject, onMounted, Ref, ref} from 'vue'
import {useQuasar} from 'quasar'
import {backend, constants, i18n} from '@/services/'
import {AddonEntry, LauncherConfig, PluginEntry} from '@/services/backend'
import {MAIN_ROUTE} from '@/services/router'
import {isGamePathValid, showFileDialog} from '@/util'

const $q = useQuasar()

const config = inject(constants.CONFIG_KEY) as Ref<LauncherConfig>

const currentTab = ref(1)

// region Game
const directXOptions = [
  {label: 'DirectX 11', value: true},
  {label: 'DirectX 9', value: false},
]

const gameSubDirsFound = ref(false)

async function onUpdateGameDir(path: string | null) {
  gameSubDirsFound.value = await isGamePathValid(path)
}

async function onClickGameDirSearch() {
  config.value.game_path = await showFileDialog(true)
  gameSubDirsFound.value = false
  await onUpdateGameDir(config.value.game_path)
}

onMounted(() => onUpdateGameDir(config.value.game_path))

// endregion

// region Language
const launcherLanguageOptions = i18n.getLauncherLanguageOptions()
const launcherLanguageChoice = ref(i18n.convertLauncherLanguage(config.value.launcher_language))
const gameLanguageOptions = i18n.getGameLanguageOptions()
const gameLanguageChoice = ref(i18n.convertGameLanguage(config.value.game_language))

async function onLauncherLanguageChange(langValue: string) {
  config.value.launcher_language = i18n.convertLauncherLanguage(langValue)
  await i18n.setLanguage(config.value.launcher_language)
}

function onGameLanguageChange(langValue: string) {
  config.value.game_language = i18n.convertLauncherLanguage(langValue)
}

// endregion

// region Addons
const editAddonDialogItem = ref({}) as Ref<AddonEntry>
const showEditAddonDialog = ref(false)
const showEditAddonDialogNew = ref(false)

function onClickAddAddon() {
  _showAddonDialog({
    is_enabled: true,
    path: '',
    command_line: '',
    run_as_admin: false,
    run_on_close: false,
    kill_after_close: false,
  }, true)
}

function onClickEditAddon(entry: backend.AddonEntry) {
  _showAddonDialog(entry, false)
}

function _showAddonDialog(entry: backend.AddonEntry, isNew: boolean) {
  showEditAddonDialogNew.value = isNew
  editAddonDialogItem.value = entry
  showEditAddonDialog.value = true
}

async function onClickAppPathSearch() {
  editAddonDialogItem.value.path = await showFileDialog(false)
}

function onClickEditAddonSave() {
  const exists = config.value.addons.includes(editAddonDialogItem.value)
  if (!exists) {
    // Insert is it does not exist
    config.value.addons.push(editAddonDialogItem.value)
  }
}

function onClickEditAddonDelete() {
  const exists = config.value.addons.includes(editAddonDialogItem.value)
  if (exists) {
    // Delete if it exists
    config.value.addons = config.value.addons.filter((entry) => entry != editAddonDialogItem.value)
  }
}

// endregion

// region Dalamud
function onWheelInjectionDelay(e: WheelEvent) {
  const delta = e.deltaY < 0 ? 100 : -100

  let value = config.value.dalamud_injection_delay_ms + delta
  if (value < 0)
    value = 0

  config.value.dalamud_injection_delay_ms = value
}

// endregion

// region Plugins
type PluginManifest = {
  Disabled: boolean,
  [k: string]: unknown,
}

type PluginData = {
  plugin: PluginEntry,
  manifest: PluginManifest,
}

const plugins = ref([]) as Ref<PluginData[]>

async function getPlugins() {
  const values = await backend.getPlugins()
  plugins.value = values.map((plugin) => {
    const manifest = JSON.parse(plugin.manifest_json) as PluginManifest
    return {plugin, manifest}
  })
}

async function updatePlugin(item: PluginData) {
  // Update our copy
  item.plugin.manifest_json = JSON.stringify(item.manifest, null, 2)
  // Send it to disk
  await backend.updatePlugin(item.plugin)
}

async function removePlugin(item: PluginData) {
  await backend.removePlugin(item.plugin)
  plugins.value = plugins.value.filter((i) => i != item)

  // Notify the user
  $q.notify({
    type: 'message',
    message: `${i18n.t('ConfigPluginDeleted')} ${item.plugin.name} v${item.plugin.version}`,
  })
}

onMounted(getPlugins)

// endregion

// region Patching
const downloadSpeedLimitMBs = ref(config.value.download_speed_limit_bytes / 1024 / 1024)

async function onClickPatchDirSearch() {
  config.value.patch_path = await showFileDialog(true)
}

function onWheelDownloadSpeedLimit(e: WheelEvent) {
  const delta = e.deltaY < 0 ? 1 : -1

  let value = downloadSpeedLimitMBs.value + delta
  if (value < 0)
    value = 0

  downloadSpeedLimitMBs.value = value
  config.value.download_speed_limit_bytes = value * 1024 * 1024
}

// endregion

// region About
const showStartOriginalLauncherDialog = ref(false)

async function onClickStartBackupTool() {
  await backend.startBackupTool()
}

async function onClickStartOriginalLauncher(useSteam: boolean) {
  await backend.startOriginalLauncher(useSteam)
}

// endregion

async function openDalamudPluginDir() {
  await backend.openDalamudPluginDir()
}

async function onCompleteConfig() {
  await backend.saveConfig(config.value)
  await MAIN_ROUTE.push()
}
</script>

<style lang="sass" scoped>
</style>
