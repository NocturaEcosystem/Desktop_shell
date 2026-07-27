<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

let Launcher = ref<HTMLElement | null>(null)
const keys = new Set<string>()

function shortcuts(e: KeyboardEvent) {
	console.log([...keys])
  keys.add(e.key)
  console.log(Launcher.value)
  e.preventDefault()
  if (keys.has("Super") && keys.has("a")) {
    if (Launcher?.value instanceof HTMLElement) {
      if (Launcher.value.classList.contains("show")) {
				Launcher.value.classList.remove("show")
			} else {
				Launcher.value.classList.add("show")
			}
    }
  }
}

function keyUp(e: KeyboardEvent) {
	console.log([...keys])
	e.preventDefault()
	keys.delete(e.key)
}

onMounted(() => {
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
        
        <div class="app-type">Games</div>
        <div class="line lineEx"></div>
        <div class="launcherAppList">
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
            <div class="app-wrap">
                <img src="/tauri.svg" class="appPadIco app-ico" />
            </div>
        </div>
        
    </div>
</template>

<style src="../styles/launcher.css"></style>

