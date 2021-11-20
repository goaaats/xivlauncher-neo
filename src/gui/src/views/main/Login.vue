<template>
  <q-card class="q-px-md q-py-sm">
    <div>
      <q-input dense v-model="username" :label="$t('LoginBoxUsername')" maxlength="100"></q-input>
      <q-input dense v-model="password" :label="$t('LoginBoxPassword')" maxlength="100" type="password"/>
    </div>

    <div class="flex-grow-1">
      <q-checkbox size="xs" v-model="autoLogin" :label="$t('LoginBoxAutoLogin')"/>
      <q-checkbox size="xs" v-model="useOtp" :label="$t('LoginBoxOtp')"/>
      <q-checkbox size="xs" v-model="useSteam" :label="$t('LoginBoxSteam')"/>
    </div>

    <div class="row justify-center">
      <q-btn color="primary" @click="clickLogin" :loading="startingGame" class="col-5">
        <q-tooltip :delay="500"> {{ $t("LoginBoxLoginTooltip") }}</q-tooltip>
        {{ $t("LoginBoxLogin") }}
        <template v-slot:loading>
          <q-spinner/>
        </template>
      </q-btn>
      <div class="q-pl-sm"/>
      <q-btn color="primary" @click="clickAccountSwitcher" class="col-3">
        <q-tooltip :delay="500"> {{ $t("OpenAccountSwitcher") }}</q-tooltip>
        <q-icon name="groups"/>
      </q-btn>
    </div>
  </q-card>
</template>

<script lang="ts">
import {defineComponent, ref} from "vue"
import {QCard, QBtn, QCheckbox, QInput} from "quasar"

const username = ref("")
const password = ref("")
const autoLogin = ref(true)
const useOtp = ref(true)
const useSteam = ref(true)
const startingGame = ref(false)

async function clickLogin() {
  startingGame.value = true
}

async function clickAccountSwitcher() {
  console.log("clickAccountSwitcher")
}

export default defineComponent({
  name: "main-login",
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
