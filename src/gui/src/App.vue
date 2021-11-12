<template>
  <div class="content">
    <MainWindow msg="Welcome to Your Vue.js + TypeScript App in a Rust Workspace." />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import MainWindow from './views/Main.vue';

export default defineComponent({
  name: 'App',
  components: {
    MainWindow,
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

body {
  margin: 0;
  padding: 0;
  overflow-y: hidden;
}

.content {
  margin: 30px 8px 8px;
  overflow-y: auto;
}
</style>