<script setup lang="ts">
import timeContext from "../components/contexts/time.vue"
import sysContext from "../components/contexts/sys.vue"
import { ref, onMounted, onActivated } from 'vue'

let showTime = ref(false)
let showSys = ref(false)
let currentWorkspace = 0
let workspace = ref<HTMLElement | null>(null)
let workspaces = ref([
    {
        "num": 1,
        "active": true,
        "id": 1021
    },
    {
        "num": 2,
        "active": false,
        "id": 1021
    }
])
let date = ref<HTMLElement | null>(null);

onMounted(() => {
    setInterval(() => {
        if (date.value) {
            let time = new Date();
            date.value.innerText = `${time.toLocaleTimeString("en-US", { month: "short", day: "2-digit", hour: "2-digit", minute: "2-digit", hour12: true}).replace(",", "")} `
        }
    }, 1000)
})

function activate(space: number) {
    if (workspaces.value) {
        workspaces.value.forEach((val) => {
            val.active = false
        })
        if (workspaces.value[space-1]) {
            workspaces.value[space-1].active = true
        }
        console.log("WORKSPACES", workspaces)
    }

}

function makeNewSpace() {
    if (workspaces.value) {
        if (workspaces.value.length >= 9){
            return
        }
        let newNum = (workspaces.value.length + 1) // length result is 1 indexed so no need for another +1
        workspaces.value.push({
            num: newNum,
            active: false,
            id: 1011,
        })
    }
}

</script>

<template>
    <div class="topPanel">
        <div class="topPanel-left">
            <div class="panelWorkspace" ref="workspace">
                <div
                    v-for="(num, index) in workspaces"
                    :key="index"
                    :class="num.active == true ? 'workspace active' : 'workspace' "
                    @click="activate(num.num)"
                >
                    {{ num.num }}
                </div>

                <div class="workspace" @click="makeNewSpace()">+</div>
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