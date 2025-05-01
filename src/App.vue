<template>
  <div id="app">
    <SkyBox class="fullscreen" />
  </div>
</template>

<script setup lang="ts">
import SkyBox from './components/Scene.vue'
</script>

<style>
#app {
  width: 100vw;
  height: 100vh;
}

.fullscreen {
  width: 100%;
  height: 100%;
}
</style>