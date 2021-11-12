<template>
  <div class="main-view">
    <div class="mdc-card xiv-news">
      hello world
    </div>
    <button class="mdc-button foo-button">
      <div class="mdc-button__ripple"></div>
      <span class="mdc-button__label">Button2</span>
    </button>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';

export default defineComponent({
  name: 'MainWindow',
  props: {
    msg: String,
  },
});
</script>

<style scoped>
.xiv-news {
  width: 544px;
  height: 213px;
}

h3 {
  margin: 40px 0 0;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}
</style>
