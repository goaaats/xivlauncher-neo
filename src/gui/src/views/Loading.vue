<template>
  <div id="content" class="flex fit justify-center items-center">

    <div>
      <q-inner-loading>
        <q-spinner-oval color="warning" size="5em"/>
      </q-inner-loading>
    </div>

  </div>
</template>

<script lang="ts">
import {QSpinnerOval} from "quasar"

export default {
  name: "loading-view",
  components: {
    QSpinnerOval,
  },
}
</script>

<style lang="sass" scoped>
#content
  //noinspection CssUnknownTarget
  background: url("/static/logo.png") no-repeat fixed center
  background-size: contain
</style>