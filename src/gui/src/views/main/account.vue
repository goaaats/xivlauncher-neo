<template>
  <div>

    <q-dialog v-model="showAccountsDialog">
      <q-card>
        <q-card-section class="row q-pa-sm">
          <!-- This is enough room for exactly 5 rows before it starts scrolling -->
          <q-scroll-area style="width: 250px; height: 220px">
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

    <q-dialog v-model="showEditAccountDialog">
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
                   :disable="editAccount.username.length === 0"
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

  </div>
</template>

<script lang="ts" setup>
import {ref, Ref} from 'vue'
import {backend, store} from '@/services/'
import {AccountEntry} from '@/services/backend'

const config = store.CONFIG.inject()
const showAccountsDialog = store.SHOW_ACCOUNTS_DIALOG.inject()
const showEditAccountDialog = store.SHOW_EDIT_ACCOUNT_DIALOG.inject()
const currentAccount = store.CURRENT_ACCOUNT.inject()
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

async function selectAccount(entry: AccountEntry) {
  currentAccount.value = entry
  config.value.current_account_id = entry.username

  // Save to disk
  await backend.saveConfig(config.value)

  // Close the dialog
  showAccountsDialog.value = false
}

function openEditAccount(event: PointerEvent, entry: AccountEntry) {
  event.preventDefault()  // Right click
  editAccount.value = entry
  showEditAccountDialog.value = true
}

async function updateAccountPicture() {
  // TODO should this be in rust?

  const encUsername = encodeURIComponent(editAccount.value.character_name)
  const encServer = encodeURIComponent(editAccount.value.character_world)

  const url = `https://xivapi.com/character/search?name=${encUsername}&server=${encServer}`
  console.log(url)
  const resp = await fetch(url)
  if (!resp.ok) {
    // TODO
    console.log(`that is unfortunate ${resp.status} ${resp.statusText}`)
    return
  }

  type XivApiResp = {
    Pagination: {
      PageNext?: number,
      PagePrev?: number,
      PageTotal: number,
      Results: number,
      ResultsPerPage: number,
      ResultsTotal: number,
    }
    Results: {
      Avatar: string,
      Name: string,
      Server: string,
    }[]
  }

  // If pagination becomes a problem, deal with it then
  const json = await resp.json() as XivApiResp
  console.log(json)
  console.log(editAccount.value.character_name)
  const results = json.Results.filter(result => result.Name === editAccount.value.character_name)
  if (results.length === 0) {
    // TODO
    console.log('no results!')
    return
  }

  const result = results[0]
  editAccount.value.thumbnail_url = result.Avatar
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

</script>

<style lang="sass" scoped>
</style>
