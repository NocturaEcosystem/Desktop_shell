<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
let ShellContext = ref<HTMLElement | null>(null)

function contextShow(e: MouseEvent) {
	e.preventDefault()
	if (ShellContext.value instanceof HTMLElement) {
		ShellContext.value.style.left = `${e.clientX}px`
		ShellContext.value.style.top = `${e.clientY}px`
		ShellContext.value.classList.add("show")
	}
}

function contextHide(e: MouseEvent) {
	e.preventDefault()
	if (ShellContext.value instanceof HTMLElement) {
			if ( ShellContext.value.classList.contains("show") ) {
				ShellContext.value.classList.remove("show")
			}
	}
}

onMounted(() => {
	window.addEventListener("contextmenu", contextShow)
	window.addEventListener("click", contextHide)
})

onUnmounted(() => {
	window.addEventListener("contextmenu", contextShow)
	window.removeEventListener("click", contextHide)
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
