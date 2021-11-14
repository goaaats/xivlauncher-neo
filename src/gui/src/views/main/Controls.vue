<template>
  <q-card id="main-controls">
    <div class="centered">
      <q-btn flat @click="clickMaintenance" class="control-button">
        <q-tooltip :delay="500"> {{ $t("MaintenanceQueue") }}</q-tooltip>
        <q-icon name="update" size="3rem" color="info"/>
      </q-btn>
      <q-btn flat @click="clickWorldStatus" class="control-button">
        <q-tooltip :delay="500"> {{ $t("WorldStatus") }}</q-tooltip>
        <q-icon name="public" size="3rem" color="info"/>
      </q-btn>
      <q-btn flat @click="clickSettings" class="control-button">
        <q-tooltip :delay="500"> {{ $t("Settings") }}</q-tooltip>
        <q-icon name="settings" size="3rem" color="info"/>
      </q-btn>
    </div>
  </q-card>
</template>

<style lang="sass" scoped>
#main-controls
  display: flex
  flex-grow: 1

.centered
  display: flex
  align-items: center
  justify-content: center
  width: 100%
  height: 100%

.control-button
  border-radius: 5px
  width: 77px
  height: 77px

</style>

<script>
import {defineComponent} from "vue"
import {QBtn, QCard, QIcon} from "quasar"
import {useI18n} from "vue-i18n"

function clickMaintenance() {
}

function clickWorldStatus() {
}

function clickSettings() {
}

export default defineComponent({
  name: "main-controls",
  components: {
    QBtn, QCard, QIcon
  },
  setup() {
    const {t} = useI18n()

    return {
      t,
      clickMaintenance, clickWorldStatus, clickSettings,
    }
  }
})
</script>


