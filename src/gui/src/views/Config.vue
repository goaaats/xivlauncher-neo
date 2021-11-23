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
          <q-tab-panel :name="1">
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
          </q-tab-panel>

          <q-tab-panel :name="2">
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("ConfigChooseLauncherLanguage") }}</p>
              <q-select v-model="launcherLanguageChoice" filled options-dense
                        :options="launcherLanguageOptions" :label="$t('ConfigLauncherLanguage')"
                        behavior="dialog" @update:model-value="onLauncherLanguageChange"/>
            </div>
            <br/>
            <div>
              <p class="q-mb-xs ws-wrap">{{ $t("ConfigChooseGameLanguage") }}</p>
              <q-select v-model="gameLanguageChoice" filled options-dense
                        :options="gameLanguageOptions"
                        :label="$t('ConfigGameLanguage')" behavior="dialog"/>
            </div>
            <br/>
            <div>
              <a href="https://crowdin.com/project/ffxivquicklauncher" target="_blank" class="a-none">{{ $t('ConfigLocalizationHelp') }}</a>
            </div>

          </q-tab-panel>

          <q-tab-panel :name="3" class="flex column q-pb-sm">
            <div class="q-pb-xs">
              {{ $t('ConfigAddonNotice') }}
            </div>

            <q-card class="bg-grey-9 flex-grow-1">
              <q-scroll-area class="fit">
                <q-list separator>
                  <q-item v-for="(item, index) in addons"
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
                      <q-icon v-if="!editAddonDialogItem.path" name="search" class="cursor-pointer"
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

          <q-tab-panel :name="4" class="flex column">
            <p>{{ $t("ConfigDalamudNotice") }}</p>
            <q-checkbox v-model="enableDalamud" dense
                        :label="$t('ConfigDalamudEnable')"/>
            <div v-if="enableDalamud">
              <span>{{ $t("ConfigDalamudInGameHint") }}</span>
              <q-input v-model.number="injectionDelay" type="number" style="max-width: 150px"
                       :label="$t('ConfigDalamudInjectionDelay') + ' (ms)'"
                       @wheel="onWheelInjectionDelay"/>
              <br/>
              <span>{{ $t("ConfigUniversalisHint") }}</span>
              <q-checkbox v-model="optOutMb" dense
                          :label="$t('ConfigUniversalisOptOut')"/>
            </div>

          </q-tab-panel>

          <q-tab-panel :name="5" class="flex column">
            <div>{{ $t('ConfigPluginsDescription') }}</div>
            <div>{{ $t("ConfigPluginsHint") }}</div>

            <q-card class="bg-grey-9 flex-grow-1">
              <q-scroll-area class="fit">
                <q-list separator>
                  <q-item v-for="(item, index) in plugins"
                          :key="index" dense class="q-px-none">
                    <q-item-label>
                      <q-toggle v-model="item.json.Disabled"
                                :true-value="false" :false-value="true"
                                tabindex="-1" @update:model-value="updatePlugin(item)"/>
                    </q-item-label>
                    <q-item-section>
                      <div v-if="!item.json.Disabled">
                        {{ item.plugin.name }} {{ item.plugin.version }}
                      </div>
                      <div v-if="item.json.Disabled" style="text-decoration: line-through">
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

          <q-tab-panel :name="6">
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
          </q-tab-panel>

          <q-tab-panel :name="7" class="row">
            <div class="column col-8">
              <p>
            <span class="ws-wrap">
            {{ $t("ConfigCredits") }}
            </span>
                <br/>
                <a href="https://github.com/goaaats/xivlauncher-neo/blob/master/LICENSE" target="_blank" class="a-none">
                  {{ $t("ConfigLicense") }}
                </a>
              </p>
            </div>

            <div class="column col-4">
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-discord"
                     :label="$t('ConfigJoinDiscord')"
                     type="a" target="_blank" href="https://discord.gg/3NMcUV5"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-github"
                     label="GitHub" type="a" target="_blank"
                     href="https://github.com/goaaats/FFXIVQuickLauncher"/>
              <q-btn flat align="left" class="row full-width" :no-caps="true" icon="mdi-information"
                     :label="$t('ConfigOpenFaq')"
                     type="a" target="_blank" href="https://goatcorp.github.io/faq/"/>
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

              <!--
              <q-card style="width: 300px">
                <q-card-section class="justify-center">
                  <div class="text-h6 text-center">
                    {{ $t('ConfigLaunchAsSteam') }}
                  </div>
                </q-card-section>

                <q-card-actions align="right" class="justify-center">
                  <q-btn v-close-popup class="bg-primary" :no-caps="true"
                         :label="$t('ConfigLaunchAsSteamYes')"
                         @click="onClickStartOriginalLauncher(true)"/>
                  <q-btn v-close-popup class="bg-primary" :no-caps="true"
                         :label="$t('ConfigLaunchAsSteamNo')"
                         @click="onClickStartOriginalLauncher(false)"/>
                </q-card-actions>
              </q-card>
              -->
            </q-dialog>
          </q-tab-panel>
        </q-tab-panels>

        <q-page-sticky position="bottom-right" :offset="[9,9]">
          <div class="q-gutter-sm">
            <q-btn push round color="primary" icon="mdi-folder-open"
                   @click="onClickOpenPluginsFolder">
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

<script lang="ts">
import {inject, onMounted, Ref, ref} from 'vue'
import {dialog} from '@tauri-apps/api'
import * as constants from '@/services/constants'
import * as backend from '@/services/backend'
import * as i18n from '@/services/i18n'
import {useQuasar} from 'quasar'

export default {
  name: 'ConfigView',
  setup() {
    const $q = useQuasar()

    const settings = inject(constants.SETTINGS_KEY) as Ref<backend.LauncherSettings>
    const addons = inject(constants.ADDONS_KEY) as Ref<backend.AddonEntry[]>

    // TODO Set to 1
    const currentTab = ref(5)

    // Tab 1
    const gameDir = ref('')

    // Tab 2
    const launcherLanguageChoice = ref(i18n.convertLauncherLanguage(settings.value.launcher_language))
    const gameLanguageChoice = ref(i18n.convertGameLanguage(settings.value.game_language))

    const launcherLanguageOptions = i18n.getLauncherLanguageOptions()
    const gameLanguageOptions = i18n.getGameLanguageOptions()

    async function onLauncherLanguageChange(lang: string) {
      const locale = i18n.convertLauncherLanguage(lang)
      await i18n.setLanguage(locale)
    }

    // Tab 3
    const editAddonDialogItem = ref({}) as Ref<backend.AddonEntry>
    const showEditAddonDialog = ref(false)
    const showEditAddonDialogNew = ref(false)

    function onClickAddAddon() {
      showAddonDialog({
        is_enabled: true,
        path: '',
        command_line: '',
        run_as_admin: false,
        run_on_close: false,
        kill_after_close: false,
      }, true)
    }

    function onClickEditAddon(entry: backend.AddonEntry) {
      showAddonDialog(entry, false)
    }

    function showAddonDialog(entry: backend.AddonEntry, isNew: boolean) {
      showEditAddonDialogNew.value = isNew
      editAddonDialogItem.value = entry
      showEditAddonDialog.value = true
    }

    async function onClickAppPathSearch() {
      editAddonDialogItem.value.path = await showFileDialog(false)
    }

    function onClickEditAddonSave() {
      const exists = addons.value.includes(editAddonDialogItem.value)
      if (!exists) {
        // Insert is it does not exist
        addons.value.push(editAddonDialogItem.value)
      }
    }

    function onClickEditAddonDelete() {
      const exists = addons.value.includes(editAddonDialogItem.value)
      if (exists) {
        // Delete if it exists
        addons.value = addons.value.filter((entry) => entry != editAddonDialogItem.value)
      }
    }

    // Tab 4
    const enableDalamud = ref(settings.value.enable_dalamud)
    const injectionDelay = ref(settings.value.dalamud_injection_delay_ms)
    const optOutMb = ref(settings.value.opt_out_mb_collection)

    function onWheelInjectionDelay(e: WheelEvent) {
      injectionDelay.value += (e.deltaY < 0 ? 100 : -100)
    }

    // Tab 5
    type PluginJsonData = {
      Disabled: boolean,
      [k: string]: unknown,
    }

    type PluginData = {
      plugin: backend.PluginEntry,
      json: PluginJsonData,
    }

    const plugins = ref([]) as Ref<PluginData[]>

    onMounted(async () => {
      const values = await backend.getPlugins()
      plugins.value = values.map((backendPlugin) => {
        return {
          plugin: backendPlugin,
          json: JSON.parse(backendPlugin.manifest) as PluginJsonData,
        }
      })
    })

    async function updatePlugin(item: PluginData) {
      // Update our copy
      item.plugin.manifest = JSON.stringify(item.json, null, 2)
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

    // Tab 6
    // Tab 7
    const showStartOriginalLauncherDialog = ref(false)

    async function onClickStartBackupTool() {
      await backend.startBackupTool()
    }

    async function onClickStartOriginalLauncher(useSteam: boolean) {
      await backend.startOriginalLauncher(useSteam)
    }

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

    function onClickOpenPluginsFolder() {
      console.log(/* TODO */)
    }

    async function onCompleteConfig() {
      settings.value.launcher_language = i18n.convertLauncherLanguage(launcherLanguageChoice.value)
      settings.value.game_language = i18n.convertGameLanguage(gameLanguageChoice.value)
      await backend.setSettings(settings.value)
    }

    return {
      t: i18n.t, currentTab,
      // 1
      gameDir,
      // 2
      launcherLanguageChoice, launcherLanguageOptions, onLauncherLanguageChange,
      gameLanguageChoice, gameLanguageOptions,
      // 3
      addons, showEditAddonDialog, showEditAddonDialogNew, editAddonDialogItem,
      onClickAddAddon, onClickEditAddon, onClickAppPathSearch,
      onClickEditAddonSave, onClickEditAddonDelete,
      // 4
      enableDalamud, injectionDelay, optOutMb,
      onWheelInjectionDelay,
      // 5
      plugins, updatePlugin, removePlugin,
      // 6
      // 7
      showStartOriginalLauncherDialog,
      onClickStartBackupTool, onClickStartOriginalLauncher,
      //
      onClickOpenPluginsFolder, onCompleteConfig,
    }
  },
}
</script>

<style lang="sass" scoped>
</style>
