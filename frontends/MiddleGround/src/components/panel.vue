<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import "../styles/panel.css"
import "../styles/contexts/shell.css"
import { writeTextFile, BaseDirectory, readTextFile, mkdir, exists } from "@tauri-apps/plugin-fs";

let global_name = "" // the name of the app we might unpin from panel
let global_exec = "" // the exec-code of the app we might unpin from panel (as a secondary
//                      verifier)

const panel = ref<any>(null);
let interval: number;
const context = ref<HTMLElement | null>()
const PANELNAME = "panel.json"
const options = { baseDir: BaseDirectory.AppConfig };

onMounted(async () => {
    await mkdir('', { ...options, recursive: true }).catch(() => {});
    if (!(await exists(PANELNAME, options))) {
        await writeTextFile(PANELNAME, '{}', options);
    }
    const configText = await readTextFile(PANELNAME, options);
    let prevConfig = JSON.parse(configText);
    panel.value = prevConfig;

    async function checkUpdate() {
        const configText = await readTextFile(PANELNAME, options);
        let config = JSON.parse(configText);

        if (JSON.stringify(config) !== JSON.stringify(prevConfig)) {
            prevConfig = config;
            panel.value = config;
        }
    }

    interval = window.setInterval(checkUpdate, 5000);
});

async function unpin() {
    const index = panel.value.findIndex(
        (app: any) => app.name === global_name && app.exec === global_exec // exec just to ensure if 2 apps have the same name,
//                                                                  The correct index is selected
    );
    panel.value.splice(index, 1)
    await writeTextFile(PANELNAME, JSON.stringify(panel.value, null, 2), options);
}

function showContext(event: MouseEvent, name: string, exec: string) {
    if (context.value) {
        context.value.style.left = `${event.clientX}px`
        context.value.style.top = `${event.clientY}px`;
        context.value.style.transform = "translateY(-100%)";
        context.value.classList.add("show");
        global_exec = exec
        global_name = name
    }
}

function hideContext() {
    if (context.value) {
        context.value.classList.remove("show")
    }
}
defineExpose({
    hideContext
})
onUnmounted(() => {
    clearInterval(interval);
});
</script>

<template>
    <div class="panel">
         <div v-for="(app, index) in panel" :key="index"
           class="app-wrap"
           :data-name="app.name"
           :data-exec="app.exec"
           @contextmenu="showContext($event, app.name, app.exec)"
         >
            <img class="app-ico" :src="app.icon">
        </div>
    </div>

    <div class="shell-context" ref="context">
      <div class="shell-button" @click="unpin()">
        <img src="/tauri.svg" class="shell-button-ico">
        <div class="shell-button-desc">Unpin app</div>
      </div>

      <div class="shell-button">
        <img src="/tauri.svg" class="shell-button-ico" ref="">
        <div class="shell-button-desc">View Information</div>
      </div>

    </div>
</template>
