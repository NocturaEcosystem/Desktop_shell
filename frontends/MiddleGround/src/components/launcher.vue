<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from '@tauri-apps/api/core'
let Launcher = ref<HTMLElement | null>(null)
import "../styles/launcher.css"
import "../styles/contexts/shell.css"
import Fuse from "fuse.js";

interface appsStructure {
  name: string,
  exec: string,
  desc: string,
  icon: string
}
const searchQuery = ref<String>("")
let defaultApp: Record<string, appsStructure[]>;
let apps = ref<Record<string, appsStructure[]>>({});
let showContext = ref<Boolean>(true)
let context = ref<HTMLElement | null>(null);

function showLauncher() {
  if (Launcher?.value instanceof HTMLElement) {
    if (Launcher.value.classList.contains("show")) {
      Launcher.value.classList.remove("show")
      hideContext()
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
  defaultApp = rawData
  apps.value = rawData
  console.log("APPS", apps.value);
  console.log("UNAPPS", rawData);
})


defineExpose({
  showLauncher
})

function showContextForApp(event: MouseEvent) {
  event.preventDefault()
  if (context.value) {
    showContext.value = true
    context.value.style.left = `${event.clientX}px`
    context.value.style.top = `${event.clientY}px`
    context.value.classList.add("show")
  }
}

function hideContext() {
  if (context.value) {
    context.value.classList.remove("show")
  }
}


apps = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  if (query == "") {
    return defaultApp
  }
  //apps
  let res: Record<string, appsStructure[]> = {};
  for (const [catName, catArr] of Object.entries(defaultApp)) {
    const option = {
      keys: ['name'],
      threshold: 0.4
    }
    const fuse = new Fuse(catArr, option)
    const searchResult = fuse.search(query)
    const filter = searchResult.map((result) => result.item)
    if (filter.length != 0) {
      res[catName] = filter
    }
  }
  return res
})
</script>

<template>
    <div class="launcher" ref="Launcher" style="display: none;" @click="hideContext">
        <div class="search-bar">
            <input style='all: unset; margin-left: 12px; width: 100%;' placeholder="Search..."
                   v-model="searchQuery"></input>
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
              <img :src="app_child.icon" class="appPadIco app-ico" @contextmenu="showContextForApp($event)"/>
            </div>
          </div>
        </template>
      </div>
    </div>

    <div class="shell-context" ref="context">

      <div class="shell-button">
        <img src="/tauri.svg" class="shell-button-ico">
        <div class="shell-button-desc">Pin app</div>
      </div>

      <div class="shell-button">
        <img src="/tauri.svg" class="shell-button-ico">
        <div class="shell-button-desc">View Information</div>
      </div>

    </div>
</template>

