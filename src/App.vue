<script setup lang="ts">
import panel from "./components/panel.vue"
import topPanel from "./components/topPanel.vue"
import shellContext from "./components/contexts/shell.vue"
import notification from "./components/notify..vue"
import appPad from "./components/appPad.vue";
</script>

<style src="./styles/vars.css"></style>
<style src="./styles/main.css"></style>
<style src="./styles/notify.css"></style>

<template>
  <topPanel />
  <panel />
  <notification />
  <appPad />

  <shellContext />
</template>
