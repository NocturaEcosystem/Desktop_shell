<script setup lang="ts">
import timeContext from "../components/contexts/time.vue"
import sysContext from "../components/contexts/sys.vue"
import { ref, onMounted, onActivated } from 'vue'

let showTime = ref(false)
let showSys = ref(false)
let currentWorkspace = 0
let workspace = ref<HTMLElement | null>(null)
let changeWorkspace = (num: number) => {}

onMounted(() => {
    changeWorkspace = (num: number) => {
        const work_el = workspace.value?.children[num] as HTMLDivElement | undefined
        const prev_el = workspace.value?.children[currentWorkspace] as HTMLDivElement | undefined
        if (work_el && prev_el) {
            work_el.classList.add('active')
            prev_el.classList.remove('active')
            currentWorkspace = num
        }
    }
    changeWorkspace(currentWorkspace)
})

function handleWorkspceSrcoll() {
    currentWorkspace += 1
    changeWorkspace(currentWorkspace)
}
</script>

<template>
    <div class="topPanel">
        <div class="topPanel-left">
            <div class="panelWorkspace" ref="workspace" @scroll="handleWorkspceSrcoll();">
                <div>1</div>
                <div>2</div>
                <div>3</div>
                <div>+</div>
            </div>
        </div>
        <div class="topPanel-center">
            <div class="panelTime" @click="showTime = !showTime">Jul 12 2:09PM</div>
        </div>
        <div class="topPanel-right">
            <div class="sys-icons" @click="showSys = !showSys">Icons will be placed here</div>
        </div>
    </div>
    <timeContext v-show="showTime" />
    <sysContext  v-show="showSys" />
</template>

<style src="../styles/topPanel.css"></style>