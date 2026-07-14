<script setup lang="ts">
import timeContext from "../components/contexts/time.vue"
import sysContext from "../components/contexts/sys.vue"
import { onMounted, onUnmounted, ref } from 'vue'



let showTime = ref(false)
let showSys = ref(false)
</script>

<template>
    <div class="topPanel">
        <div class="topPanel-left">
            <div class="panelWorkspace">
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