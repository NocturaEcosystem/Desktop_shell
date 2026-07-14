<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'

const panel = ref<any>(null);
let interval: number;

const PANELSETTINGPATH = "../../public/panel.json"

onMounted(async () => {
    const data = await fetch(PANELSETTINGPATH);
    let prevConfig = await data.json();

    panel.value = prevConfig;

    async function checkUpdate() {
        const data = await fetch(PANELSETTINGPATH);
        const config = await data.json();

        if (JSON.stringify(config) !== JSON.stringify(prevConfig)) {
            prevConfig = config;
            panel.value = config;
        }
    }

    interval = window.setInterval(checkUpdate, 5000);
});

onUnmounted(() => {
    clearInterval(interval);
});
</script>

<template>
    <div class="panel">
         <div v-for="app in panel" :key="app.id"
           class="app-wrap"
           :data-name="app.name"
           :data-exec="app.exec"
         >
            <img class="app-ico" :src="app.icon">
        </div>
    </div>
</template>

<style src="../styles/panel.css"></style>