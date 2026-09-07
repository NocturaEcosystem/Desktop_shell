<script setup lang="ts">
import timeContext from "../components/contexts/time.vue"
import sysContext from "../components/contexts/sys.vue"
import { ref, onMounted, onActivated } from 'vue'

let showTime = ref(false)
let showSys = ref(false)
let currentWorkspace = 0
let workspace = ref<HTMLElement | null>(null)
let date = ref<HTMLElement | null>(null);

onMounted(() => {
    setInterval(() => {
        if (date.value) {
            let time = new Date();
            date.value.innerText = `${time.toLocaleTimeString("en-US", { month: "short", day: "2-digit", hour: "2-digit", minute: "2-digit", hour12: true}).replace(",", "")} `
        }
    }, 1000)
})

function activate(event: Event, space: number) {
  if (event.currentTarget) {
    currentWorkspace = space;
    (event.currentTarget as HTMLElement).classList.add("active");
  }
}

</script>

<template>
    <div class="topPanel">
        <div class="topPanel-left">
            <div class="panelWorkspace" ref="workspace">
                <div class="workspace" @click="activate($event, 1)">1</div>
                <div class="workspace">2</div>
                <div class="workspace">3</div>
                <div class="workspace">+</div>
            </div>
        </div>
        <div class="topPanel-center">
            <div class="panelTime" @click="showTime = !showTime" ref="date"></div>
        </div>
        <div class="topPanel-right">
            <div class="sys-icons" @click="showSys = !showSys">Icons will be placed here</div>
        </div>
    </div>
    <timeContext v-show="showTime" />
    <sysContext  v-show="showSys" />
</template>

<style src="../styles/topPanel.css"></style>