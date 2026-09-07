<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
let Launcher = ref<HTMLElement | null>(null)
const keys = new Set<string>()

interface appsStructure {
  name: string,
  exec: string,
  desc: string,
  icon: string
}

let apps = ref<Record<string, appsStructure[]>>({});

function shortcuts(e: KeyboardEvent) {
	console.log([...keys])
  keys.add(e.key)
  console.log(Launcher.value)
  e.preventDefault()
  if (keys.has("Super") && keys.has("a")) {
    if (Launcher?.value instanceof HTMLElement) {
      if (Launcher.value.classList.contains("show")) {
				Launcher.value.classList.remove("show")
        setTimeout(() => {
          if (Launcher.value) {
            Launcher.value.style.display = "none"
          }
        }, 500)
			} else {
        Launcher.value.style.display = "flex"
        setTimeout(() => {
          if (Launcher.value) {
				  Launcher.value.classList.add("show")
          }
        }, 5)  // for smotheness, a little delay so the launcher dosent flicker
			}
    }
  }
}

function keyUp(e: KeyboardEvent) {
	console.log([...keys])
	e.preventDefault()
	keys.delete(e.key)
}

onMounted(async() => {
  const rawData: Record<string, appsStructure[]> = await invoke('get_apps');
  for (const category in rawData) {
    if (Array.isArray(rawData[category])) {
      rawData[category] = rawData[category].map(app => ({
        ...app,
        icon: app.icon ? `app-icon://localhost${app.icon}` : '/tauri.svg'
      }));
    }
  }
  apps.value = rawData
  console.log("APPS", apps.value);
  console.log("UNAPPS", rawData);
  window.addEventListener("keydown", shortcuts)
  window.addEventListener("keyup", keyUp)
})

onUnmounted(() =>{
  window.addEventListener("keydown", shortcuts)
  window.removeEventListener("keyup", keyUp)
})
</script>

<template>
    <div class="launcher" ref="Launcher">
        <div class="search-bar">
            <div>Search...</div>
        </div>

      <div v-for="(appList, categoryName) in apps" :key="categoryName">
        <template v-if="appList && appList.length > 0">       
          <div class="app-type">{{ categoryName }}</div>
          <div class="line lineEx"></div>
          <div class="launcherAppList">
            <div v-for="app_child in appList" 
                :key="app_child.name" 
                class="app-wrap"
                :data-name="app_child.name"
                :data-exec="app_child.exec"
            >
              <img :src="app_child.icon" class="appPadIco app-ico"/>
            </div>
          </div>
        </template>
      </div>

        
    </div>
</template>

<style src="../styles/launcher.css"></style>

