<script setup lang="ts">

import { ref, onMounted, onUnmounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { mkdir, copyFile } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from '@tauri-apps/api/path';

import "../styles/shell.css"
import { readFile, exists } from '@tauri-apps/plugin-fs'
import { nyx_client } from '../webClient';

async function updateDOMbg() {
  const main = await appDataDir()
  const path = await join(main, 'assets', 'bg.png')

  if (!(await exists(path))) {
    document.body.style.removeProperty("background-image");
    const superPrimaryContrast = getComputedStyle(document.documentElement).getPropertyValue('--superPrimaryContrast').trim()
    document.body.style.backgroundColor = superPrimaryContrast
    return;
  }

  const bytes = await readFile(path)
  const url = URL.createObjectURL(
    new Blob([bytes], { type: "image/png"})
  )
  document.body.style.removeProperty("background-color");
  document.body.style.backgroundImage = `url("${url}")`
}

const ShellContext = ref<HTMLElement | null>(null)
const nyxclient = new nyx_client("Lepus_widgets");
nyxclient.connect()

function contextShow(e: MouseEvent) {
	console.log("many, ShellContext", ShellContext.value)
	if (ShellContext.value) {
		ShellContext.value.style.left = `${e.clientX}px`
		ShellContext.value.style.top = `${e.clientY}px`
		ShellContext.value.classList.add("show")
	}
}

function contextHide(e: Event) {
	e.preventDefault()
	if (ShellContext.value) {
		if ( ShellContext.value.classList.contains("show") ) {
			ShellContext.value.classList.remove("show")
		}
	}
}


let showContext = (e: MouseEvent) => {
  e.preventDefault()
  if (e.target === document.body) {
    contextShow(e)
  }
}

let hideContext = (e: MouseEvent) => {
    contextHide(e)
  }

onMounted(() => {
  window.addEventListener("contextmenu", showContext)
  window.addEventListener("click", hideContext)
})
onUnmounted(() => {
  window.removeEventListener("contextmenu", showContext)
  window.removeEventListener("click", hideContext)
})

async function updateBackground(destination: string) {
  console.log("nyxclient")
  nyxclient.send("Lepus_background", {
    "Background": destination
  })
}

async function selectBg() {
  try {
    const file = await open({
      multiple: false,
      directory: false,
      title: "Select a file",
      filters: [
        {
          name: "Images",
          extensions: ["png"],
        },
      ],
    });
    if (file) {
      console.log("Selected file:", file);
      const assetsDir = await join(await appDataDir(), 'assets')
      await mkdir(assetsDir, { recursive: true })

      const destination = await join(assetsDir, 'bg.png')
      await copyFile(file, destination)
      updateBackground(destination)
    }

  } catch (error) {
    console.error("Failed to open file dialog:", error);
  }
}

defineExpose({
  updateDOMbg
})

</script>

<template>
    <div class="shell-context" ref="ShellContext">
        <div class="shell-button">
            <img src="/tauri.svg" class="shell-button-ico">
            <div class="shell-button-desc">Open Logs</div>
        </div>
        <div class="shell-button" @click="selectBg()">
            <img src="/tauri.svg" class="shell-button-ico">
            <div class="shell-button-desc">Change Wallpaper</div>
        </div>
        <div class="shell-button">
            <img src="/tauri.svg" class="shell-button-ico">
            <div class="shell-button-desc">Open Terminal</div>
        </div>
        <div class="shell-button">
            <img src="/tauri.svg" class="shell-button-ico">
            <div class="shell-button-desc">Display Settings</div>
        </div>
    </div>
</template>

