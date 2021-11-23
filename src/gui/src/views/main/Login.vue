<template>
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
</template>

<script lang="ts">
import {defineComponent, ref} from 'vue'
import {QCard, QBtn, QCheckbox, QInput} from 'quasar'

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

export default defineComponent({
  name: 'MainLogin',
  components: {
    QBtn, QCard, QCheckbox, QInput,
  },
  setup() {
    return {
      username, password,
      autoLogin, useOtp, useSteam,
      startingGame,
      clickLogin, clickAccountSwitcher,
    }
  },
})
</script>

<style lang="sass" scoped>
.q-checkbox
  height: 20px

</style>
