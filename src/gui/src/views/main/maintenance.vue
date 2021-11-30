<template>
  <q-dialog v-model="showMaintenanceDialog"
            persistent>
    <q-card>
      <q-card-section class="row items-center q-pb-none">
        <div class="text-body1">{{ $t('MainWaitingForMaintenance') }}</div>
      </q-card-section>

      <q-card-section class="flex row justify-center q-py-none">
        <q-img v-if="loginServerReady"
               fit="contain"
               width="200px"
               height="200px"
               src="/static/maintenance.gif"/>
        <!--
        TODO Some sort of update found
        -->
        <q-img v-else
               fit="contain"
               width="200px"
               height="200px"
               src="/static/maintenance.gif"/>
      </q-card-section>

      <q-card-actions class="q-pt-none" align="center">
        <q-btn color="primary"
               :label="$t('MainWaitingForMaintenanceCancel')"
               :no-caps="true"
               @click="onClickCancel"/>
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import {onMounted, ref} from 'vue'
import {backend, store} from '@/services/'

const loginServerReady = ref(false)
const showMaintenanceDialog = store.SHOW_MAINTENANCE_DIALOG.inject()

function onClickCancel() {
  showMaintenanceDialog.value = false
}

// TODO a whole bunch of stuff
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
onMounted(() => {
  void delay(5000).then(async () => {
    await onComplete()
  })
})

async function onComplete() {
  // TODO a whole bunch of stuff
  showMaintenanceDialog.value = false
  loginServerReady.value = true
  await backend.playVictoryBeep()
}

</script>

<style lang="sass" scoped>
</style>
