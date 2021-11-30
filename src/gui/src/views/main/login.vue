<template>
  <q-card class="q-px-md q-py-sm">
    <q-input v-model="currentAccount.username"
             dense
             :spellcheck="false"
             :label="$t('MainUsernamePlaceholder')"
             maxlength="100"/>
    <q-input v-model="password"
             dense
             class="q-pb-xs"
             :label="$t('MainPasswordPlaceholder')"
             maxlength="100" type="password"/>

    <div class="q-py-xs">
      <q-checkbox v-model="config.use_autologin"
                  size="xs"
                  style="height: 20px"
                  :label="$t('MainLoginAutomatically')"/>
      <q-checkbox v-model="currentAccount.use_otp"
                  size="xs"
                  style="height: 20px"
                  :label="$t('MainUseOtp')"/>
      <q-checkbox v-model="currentAccount.use_steam"
                  size="xs"
                  style="height: 20px"
                  :label="$t('MainUseSteam')"/>
    </div>

    <div class="row justify-center">
      <q-btn color="primary"
             :loading="loggingIn"
             :no-caps="true"
             class="col-5"
             :label="$t('MainLogIn')"
             @click="beginLogin">
        <q-tooltip :delay="500">{{ $t("MainLoginHint") }}</q-tooltip>
        <template #loading>
          <q-spinner/>
        </template>
      </q-btn>

      <div class="q-pl-sm"/>

      <q-btn color="primary"
             class="col-3"
             @click="openAccountSwitcher">
        <q-tooltip :delay="500">{{ $t("MainOpenAccountSwitcher") }}</q-tooltip>
        <q-icon name="mdi-account-group"/>
      </q-btn>
    </div>
  </q-card>
</template>

<script lang="ts" setup>
import {onMounted, Ref, ref} from 'vue'
import {backend, store} from '@/services/'
import {AccountEntry} from '@/services/backend'

const config = store.CONFIG.inject()
const showAccountsDialog = store.SHOW_ACCOUNTS_DIALOG.inject()

const password = ref('')
const loggingIn = ref(false)

const currentAccount = ref({}) as Ref<AccountEntry>

onMounted(setupAccount)

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

function beginLogin() {
  loggingIn.value = true
  // TODO
}

function openAccountSwitcher() {
  showAccountsDialog.value = true
}

</script>

<style lang="sass" scoped>
</style>
