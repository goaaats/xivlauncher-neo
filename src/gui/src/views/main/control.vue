<template>
  <q-card class="flex fit justify-center content-center">

    <q-btn flat class="control"
           @click="onClickMaintenance">
      <q-tooltip :delay="500">{{ $t("MainMaintenanceQueueWait") }}</q-tooltip>
      <q-icon name="mdi-update" size="3rem" color="primary"/>
    </q-btn>

    <q-btn flat class="control"
           type="a" target="_blank" href="https://is.xivup.com/">
      <q-tooltip :delay="500">{{ $t("MainWorldStatus") }}</q-tooltip>
      <q-icon name="mdi-earth" size="3rem" color="primary"/>
    </q-btn>

    <q-btn flat class="control"
           :to="CONFIG_ROUTE.path">
      <q-tooltip :delay="500">{{ $t("MainSettings") }}</q-tooltip>
      <q-icon name="mdi-cog" size="3rem" color="primary"/>
    </q-btn>

  </q-card>
</template>

<script lang="ts" setup>
import {CONFIG_ROUTE} from '@/services/router'
import {store} from '@/services'

const showMaintenanceDialog = store.SHOW_MAINTENANCE_DIALOG.inject()

function onClickMaintenance() {
  showMaintenanceDialog.value = true
}

</script>

<style lang="sass" scoped>
.control
  border-radius: 5px
  width: 77px
  height: 77px

</style>
