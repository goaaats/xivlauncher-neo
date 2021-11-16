<template>
  <div id="content" class="flex window-height window-width justify-center items-center">

    <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
      <span v-show="ready">ready!</span>
    </transition>

    <div>
      <q-inner-loading :showing="loading" :label="$t('Loading')">
        <q-spinner-oval color="warning" size="5em"/>
        <q-tooltip :delay="500">{{ $t("Loading") }}</q-tooltip>
      </q-inner-loading>
    </div>

  </div>
</template>

<script>
import {ref} from "vue"
import {useI18n} from "vue-i18n"
import {QSpinnerOval, QTooltip} from "quasar"

const loading = ref(true)
const ready = ref(false)

export default {
  name: "root-view",
  components: {
    QSpinnerOval, QTooltip,
  },
  setup() {
    const {t} = useI18n()

    return {t, loading, ready}
  },
  mounted() {
    // TODO check if setup is necessary
    this.$router.push("/setup")
  },
}
</script>

<style lang="sass" scoped>
#content
  //noinspection CssUnknownTarget
  background: url("/static/logo.png") no-repeat fixed center
  background-size: contain
</style>