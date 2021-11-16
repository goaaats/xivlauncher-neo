<template>
  <q-card class="flex full-width full-height justify-center content-center">
    <q-btn flat @click="clickMaintenance" class="control">
      <q-tooltip :delay="500"> {{ $t("MaintenanceQueue") }}</q-tooltip>
      <q-icon name="update" size="3rem" color="primary"/>
    </q-btn>
    <q-btn flat @click="clickWorldStatus" class="control">
      <q-tooltip :delay="500"> {{ $t("WorldStatus") }}</q-tooltip>
      <q-icon name="public" size="3rem" color="primary"/>
    </q-btn>
    <q-btn flat @click="clickSettings" color="primary" class="control">
      <q-tooltip :delay="500"> {{ $t("Settings") }}</q-tooltip>
      <q-icon name="settings" size="3rem" color="primary"/>
    </q-btn>
  </q-card>
</template>

<script>
import {defineComponent} from "vue"
import {QBtn, QCard, QIcon} from "quasar"
import {useI18n} from "vue-i18n"

function clickMaintenance() {
  console.log("clickMaintenance")
}

function clickWorldStatus() {
  console.log("clickWorldStatus")
}

function clickSettings() {
  console.log("clickSettings")
}

export default defineComponent({
  name: "main-controls",
  components: {
    QBtn, QCard, QIcon,
  },
  setup() {
    const {t} = useI18n()

    return {
      t,
      clickMaintenance, clickWorldStatus, clickSettings,
    }
  },
})
</script>

<style lang="sass" scoped>
.control
  border-radius: 5px
  width: 77px
  height: 77px
</style>
