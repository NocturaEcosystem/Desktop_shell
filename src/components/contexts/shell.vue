<script setup lang="ts">

import { ref, onMounted, onUnmounted } from 'vue'

const ShellContext = ref<HTMLElement | null>(null)

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
</script>

<template>
    <div class="shell-context" ref="ShellContext">
        <div class="shell-button">
            <img src="/tauri.svg" class="shell-button-ico">
            <div class="shell-button-desc">Open Logs</div>
        </div>
    </div>
</template>

<style src="../../styles/contexts/shell.css"></style>
