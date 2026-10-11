<script setup lang="ts">
import "./styles/vars.css"
import "./styles/main.css"
import shell from "./components/shell.vue";
import { onMounted, ref } from "vue";
let Shell = ref<InstanceType<typeof shell> | null>()
onMounted(() => {
    if (Shell.value) {
        Shell.value.updateDOMbg()
    }
})
</script>


<template>
<shell ref="Shell"/>
</template>
