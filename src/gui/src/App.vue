<template>
  <TitleBar title="XIVLauncher" />
  <div class="content">
    <Test />
    <img alt="Vue logo" src="./assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js + TypeScript App in a Rust Workspace." />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import HelloWorld from './components/HelloWorld.vue';
import {TitleBar} from './components/TitleBar/TitleBar';
import {Test} from './components/Test';

export default defineComponent({
  name: 'App',
  components: {
    HelloWorld,
    Test,
    TitleBar,
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
  margin: 8px;
  margin-top: 30px;
  overflow-y: auto;
}
</style>