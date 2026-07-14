<script setup lang="ts">
import panel from "./components/panel.vue"
import topPanel from "./components/topPanel.vue"
import timeContext from "./components/contexts/time.vue"
import sysContext from "./components/contexts/sys.vue"
import shellContext from "./components/contexts/shell.vue"
</script>

<style src="./styles/vars.css"></style>
<style src="./styles/main.css"></style>

<template>
  <topPanel />
  <panel />

  <shellContext />
</template>
