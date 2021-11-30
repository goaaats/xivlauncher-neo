<template>
  <div id="content">

    <main-banner :headline="headlinePromise"/>
    <main-login/>
    <main-news :headline="headlinePromise"/>
    <main-control/>

    <maintenance-dialog/>
    <account-dialogs/>

  </div>
</template>

<script lang="ts" setup>
import {backend, store} from '@/services/'
import {Headline} from '@/services/backend'

import MainBanner from './main/banner.vue'
import MainLogin from './main/login.vue'
import MainNews from './main/news.vue'
import MainControl from './main/control.vue'
import MaintenanceDialog from './main/maintenance.vue'
import AccountDialogs from './main/account.vue'

const config = store.CONFIG.inject()

const headlinePromise = getHeadline()

async function getHeadline(): Promise<Headline> {
  return await backend.getHeadline(config.value.game_language)
}

</script>

<style lang="sass" scoped>
$window-width: 830px
$window-height: 340px

$image-ratio: 640 / 250
$image-width: 545px
$image-height: $image-width / $image-ratio

$padding: 10px
$table-gap: 10px

$left-col: $image-width
$right-col: $window-width - $left-col - $table-gap - ($padding * 2)
$top-row: $image-height
$bottom-row: $window-height - $top-row - $table-gap - ($padding * 2)

#content
  display: grid
  padding: $padding
  row-gap: $table-gap
  column-gap: $table-gap
  grid-template-columns: $left-col $right-col
  grid-template-rows: $top-row $bottom-row
</style>
