<template>
  <div id="content" class="flex fit justify-center items-center dalamud-bg">

    <div>
      <q-inner-loading>
        <q-spinner-oval color="warning" size="5em"/>
      </q-inner-loading>
    </div>

  </div>
</template>

<script lang="ts">
import {QSpinnerOval} from 'quasar'

export default {
  name: 'LoadingView',
  components: {
    QSpinnerOval,
  },
}
</script>

<style lang="sass" scoped>
</style>
