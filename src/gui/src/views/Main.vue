<template>
  <div id="content">

    <!-- Banner -->
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

    <!-- Login -->
    <q-card class="q-px-md q-py-sm">
      <div>
        <q-input v-model="currentAccount.username" dense :label="$t('MainUsernamePlaceholder')" maxlength="100"></q-input>
        <q-input v-model="password" dense :label="$t('MainPasswordPlaceholder')" maxlength="100" type="password"/>
      </div>

      <div class="q-pt-sm q-pb-xs">
        <q-checkbox v-model="config.use_autologin" size="xs" :label="$t('MainLoginAutomatically')"/>
        <q-checkbox v-model="currentAccount.use_otp" size="xs" :label="$t('MainUseOtp')"/>
        <q-checkbox v-model="currentAccount.use_steam" size="xs" :label="$t('MainUseSteam')"/>
      </div>

      <div class="row justify-center">
        <q-btn color="primary"
               :loading="loggingIn"
               :no-caps="true"
               class="col-5"
               :label="$t('MainLogIn')"
               @click="beginLogin">
          <q-tooltip :delay="500"> {{ $t("MainLoginHint") }}</q-tooltip>
          <template #loading>
            <q-spinner/>
          </template>
        </q-btn>
        <div class="q-pl-sm"/>
        <q-btn color="primary"
               class="col-3"
               @click="openAccountSwitcher">
          <q-tooltip :delay="500"> {{ $t("MainOpenAccountSwitcher") }}</q-tooltip>
          <q-icon name="mdi-account-group"/>
        </q-btn>
      </div>
    </q-card>

    <!-- News -->
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

    <!-- Controls -->
    <q-card class="flex fit justify-center content-center">
      <q-btn flat class="control" @click="onClickMaintenance">
        <q-tooltip :delay="500">{{ $t("MainMaintenanceQueueWait") }}</q-tooltip>
        <q-icon name="mdi-update" size="3rem" color="primary"/>
      </q-btn>
      <q-btn flat class="control" type="a" target="_blank" href="https://is.xivup.com/">
        <q-tooltip :delay="500">{{ $t("MainWorldStatus") }}</q-tooltip>
        <q-icon name="mdi-earth" size="3rem" color="primary"/>
      </q-btn>
      <q-btn flat color="primary" class="control" :to="CONFIG_ROUTE.path">
        <q-tooltip :delay="500">{{ $t("MainSettings") }}</q-tooltip>
        <q-icon name="mdi-cog" size="3rem" color="primary"/>
      </q-btn>
    </q-card>

    <!-- Dialogs -->
    <q-dialog v-model="showAccountSwitcherDialog">
      <q-card>
        <q-card-section class="row q-pa-sm">
          <span v-if="config.accounts.length === 0">
            {{ $t('MainNoAccountsAvailable') }}
          </span>

          <!--
          This is enough room for exactly 5 rows
          before it starts scrolling
          -->
          <q-scroll-area v-else
                         style="width: 250px; height: 220px">
            <q-list>
              <q-item v-for="entry in config.accounts"
                      :key="entry.username"
                      clickable dense
                      class="q-px-none"
                      @click="selectAccount(entry)"
                      @contextmenu="openEditAccount($event, entry)">
                <q-item-section avatar>
                  <q-img v-if="entry.thumbnail_url" :src="entry.thumbnail_url"/>
                  <q-img v-else src="/static/default_profile.png"/>
                </q-item-section>
                <q-item-label :lines="1"
                              class="self-center col-grow ">
                  {{ entry.username }}
                </q-item-label>
                <q-item-section side class="q-pr-none">
                  <q-icon v-if="entry.use_steam" name="mdi-steam" size="xs"/>
                  <q-icon v-else size="xs"/>

                  <q-icon v-if="entry.use_otp" name="mdi-key" size="xs" class="flip-horizontal"/>
                  <q-icon v-else size="xs"/>
                </q-item-section>
              </q-item>

            </q-list>
          </q-scroll-area>
        </q-card-section>
      </q-card>
    </q-dialog>

    <q-dialog v-model="showAccountEditDialog">
      <q-card>
        <q-card-section class="flex column">
          <q-input v-model="editAccount.username"
                   :label="$t('MainAccountUsernamePlaceholder')"
                   :spellcheck="false" maxlength="100"
                   dense
                   class="q-pb-sm"/>

          <div class="column q-pb-sm">
            <q-checkbox v-model="editAccount.save_password"
                        :label="$t('MainAccountSavePassword')"
                        dense/>
            <q-checkbox v-model="editAccount.use_otp"
                        :label="$t('MainAccountUseOtp')"
                        dense/>
            <q-checkbox v-model="editAccount.use_steam"
                        :label="$t('MainAccountUseSteam')"
                        dense/>
          </div>

          <div class="row q-pb-sm">
            <q-img v-if="editAccount.thumbnail_url"
                   :src="editAccount.thumbnail_url"
                   width="40px" height="40px"/>
            <q-img v-else src="/static/default_profile.png"
                   width="40px" height="40px"/>

            <q-input v-model="editAccount.character_name"
                     dense
                     :label="$t('MainAccountCharacterName')"
                     :spellcheck="false" maxlength="100"
                     class="col-grow q-px-sm"/>
            <q-input v-model="editAccount.character_world"
                     dense
                     :label="$t('MainAccountCharacterWorld')"
                     :spellcheck="false" maxlength="100"
                     class="col-4 q-pr-xs"/>
            <div class="block self-center q-gutter-xs">
              <q-btn color="primary" icon="mdi-autorenew" dense
                     @click="updateAccountPicture">
                <q-tooltip :delay="500">{{ $t('MainAccountUpdate') }}</q-tooltip>
              </q-btn>
              <q-btn color="primary" icon="mdi-restore" dense
                     @click="resetAccountPicture">
                <q-tooltip :delay="500">{{ $t('MainAccountReset') }}</q-tooltip>
              </q-btn>
            </div>
          </div>

          <div class="row q-gutter-sm">
            <q-btn :label="$t('MainAccountCreateShortcut')"
                   :disable="editAccount.thumbnail_url.length === 0"
                   color="primary"
                   :no-caps="true"
                   @click="createAccountShortcut"/>
            <q-btn v-close-popup
                   :label="$t('MainAccountDelete')"
                   color="negative"
                   :no-caps="true"
                   @click="deleteAccount"/>
          </div>

        </q-card-section>
      </q-card>
    </q-dialog>

    <q-dialog v-model="showMaintenanceDialog" persistent>
      <q-card>
        <q-card-section class="row items-center q-pb-none">
          <div class="text-body1">{{ $t('MainWaitingForMaintenance') }}</div>
        </q-card-section>

        <q-card-section>
          <div v-if="loginServerReady" class="flex row justify-center">
            <!--
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/blm.victory.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/rdm.victory.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/rog.victory.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/war.victory.gif" class="flip-horizontal"/>
            -->
          </div>
          <div v-else class="flex row justify-center">
            <!--
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/blm.walking.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/rdm.walking.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/rog.walking.gif" class="flip-horizontal"/>
            <q-img fit="contain" width="48px" height="48px" src="/static/sprites/war.walking.gif" class="flip-horizontal"/>
            -->
          </div>
        </q-card-section>

        <q-card-actions align="center">
          <q-btn color="primary"
                 :label="$t('MainWaitingForMaintenanceCancel')"
                 :no-caps="true"
                 @click="onClickMaintenanceCancel"/>
        </q-card-actions>
      </q-card>
    </q-dialog>

  </div>
</template>

<script lang="ts" setup>
import {inject, onMounted, Ref, ref} from 'vue'
import {backend, constants} from '@/services/'
import {AccountEntry, BannerEntry, Headline, HeadlineEntry, LauncherConfig} from '@/services/backend'
import {CONFIG_ROUTE} from '@/services/router'

const config = inject(constants.CONFIG_KEY) as Ref<LauncherConfig>

// region Headline

type BannerData = BannerEntry & {
  loaded: boolean,
  srcData: string,
}

type HeadlineData = HeadlineEntry & {
  type: string
}

const bannerIndex = ref(0)
const bannerData = ref([]) as Ref<BannerData[]>
const headlineData = ref([]) as Ref<HeadlineData[]>

async function getHeadline() {
  let headline: Headline

  headline = await backend.getHeadline(config.value.game_language)

  bannerData.value = headline.banner.map((banner) => {
    return {...banner, loaded: false, srcData: ''}
  })

  function mapHeadline(entry: HeadlineEntry, type: string): HeadlineData {
    return {...entry, type}
  }

  headlineData.value = ([] as HeadlineData[]).concat(
    headline.news.map(e => mapHeadline(e, 'news')),
    headline.pinned.map(e => mapHeadline(e, 'pinned')),
    headline.topics.map(e => mapHeadline(e, 'topic')),
  ).sort((e1, e2) => e2.date.localeCompare(e1.date))

  for (const banner of bannerData.value) {
    const data = await backend.getBannerImageData(banner.lsb_banner)
    banner.srcData = `data:image;base64, ${data}`
    banner.loaded = true
  }
}

onMounted(getHeadline)

// endregion

// region Login
const password = ref('')
const loggingIn = ref(false)
const showAccountSwitcherDialog = ref(false)
const showAccountEditDialog = ref(false)
const currentAccount = ref({}) as Ref<AccountEntry>
const editAccount = ref({}) as Ref<AccountEntry>

async function addEmptyAccount(): Promise<AccountEntry> {
  const account = {
    character_name: '',
    character_world: '',
    save_password: false,
    thumbnail_url: '',
    use_otp: false,
    use_steam: false,
    username: ''
  }
  config.value.accounts.push(account)
  await backend.saveConfig(config.value)

  return account
}

async function setupAccount() {
  // Make a new account
  if (config.value.accounts.length === 0) {
    currentAccount.value = await addEmptyAccount()

    // Different account, save to disk
    config.value.current_account_id = currentAccount.value.username
    await backend.saveConfig(config.value)

    return
  }

  // Account name match
  const foundAccount = config.value.accounts.find((entry) => entry.username === config.value.current_account_id)
  if (foundAccount) {
    currentAccount.value = foundAccount
    return
  }

  // No match, load the first instead
  currentAccount.value = config.value.accounts[0]

  // Different account, save to disk
  config.value.current_account_id = currentAccount.value.username
  await backend.saveConfig(config.value)
}

onMounted(setupAccount)

function beginLogin() {
  loggingIn.value = true
  // TODO
}

function openAccountSwitcher() {
  showAccountSwitcherDialog.value = true
}

async function selectAccount(entry: AccountEntry) {
  currentAccount.value = entry
  config.value.current_account_id = entry.username

  // Save to disk
  await backend.saveConfig(config.value)

  // Close the dialog
  showAccountSwitcherDialog.value = false
}

function openEditAccount(event: PointerEvent, entry: AccountEntry) {
  event.preventDefault()  // Right click
  editAccount.value = entry
  showAccountEditDialog.value = true
}

function updateAccountPicture() {
  console.log('TODO')
}

function resetAccountPicture() {
  editAccount.value.thumbnail_url = ''
}

async function createAccountShortcut() {
  await backend.createAccountShortcut(editAccount.value)
  // TODO Success notification
}

async function deleteAccount() {
  // Remove the account from the list
  config.value.accounts = config.value.accounts.filter(entry => entry != editAccount.value)

  // Add a new empty account if necessary
  if (config.value.accounts.length === 0) {
    await addEmptyAccount()
  }

  // If the current account is the deleted account, swap
  if (currentAccount.value === editAccount.value) {
    await selectAccount(config.value.accounts[0])
  }

  // Drop the reference
  editAccount.value = {} as AccountEntry

  // Save to disk
  await backend.saveConfig(config.value)
}

// endregion

// region Controls

const loginServerReady = ref(false)
const showMaintenanceDialog = ref(false)

function onClickMaintenance() {
  loginServerReady.value = false
  showMaintenanceDialog.value = true

  // TODO a whole bunch of stuff
  const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
  void delay(5000).then(onMaintenanceComplete)
}

function onClickMaintenanceCancel() {
  showMaintenanceDialog.value = false
}

async function onMaintenanceComplete() {
  // TODO a whole bunch of stuff
  loginServerReady.value = true
  await backend.playVictoryBeep()
  showMaintenanceDialog.value = false
}

// endregion


</script>

<style lang="sass" scoped>
$window-width: 830px
$window-height: 340px

$image-ratio: 640 / 250
$image-width: 545px
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
