<template>
  <q-card class="flex fit justify-center content-center">
    <q-btn flat class="control" @click="clickMaintenance">
      <q-tooltip :delay="500">{{ $t("MaintenanceQueue") }}</q-tooltip>
      <q-icon name="update" size="3rem" color="primary"/>
    </q-btn>
    <q-btn flat class="control" type="a" href="https://is.xivup.com/" @click="clickWorldStatus">
      <q-tooltip :delay="500">{{ $t("WorldStatus") }}</q-tooltip>
      <q-icon name="public" size="3rem" color="primary"/>
    </q-btn>
    <q-btn flat color="primary" class="control" @click="clickSettings">
      <q-tooltip :delay="500">{{ $t("Settings") }}</q-tooltip>
      <q-icon name="settings" size="3rem" color="primary"/>
    </q-btn>
  </q-card>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {useRouter} from 'vue-router'
import * as routes from '@/router/route'

export default defineComponent({
  name: 'MainControls',
  setup() {
    const router = useRouter()

    function clickMaintenance() {
      console.log('clickMaintenance')
    }

    function clickWorldStatus() {
      console.log('clickWorldStatus')
    }

    async function clickSettings() {
      await router.push(routes.CONFIG_ROUTE)
    }

    return {
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
