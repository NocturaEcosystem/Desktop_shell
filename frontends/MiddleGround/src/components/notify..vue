<script setup lang="ts">
import { ref } from 'vue'
const notifyBox = ref<HTMLElement | null>()
const title = ref<HTMLElement | null>()
const subj = ref<HTMLElement | null>()
const appName = ref<HTMLElement | null>()
const options= ref<string[]>([])
const show = ref<boolean>(false)
function hideNotification() {
    show.value = false
}

function showNotification(app_name: string, app_title: string, subject: string, app_options: string[] | null = null) {
    if (title.value && subj.value && appName.value) {
        appName.value.innerHTML = app_name
        title.value.innerHTML = app_title
        subj.value.innerHTML = subject
        if (app_options) {
            options.value = app_options
        }
    }
    show.value = true
    console.log("x")

}

defineExpose({
    showNotification,
    hideNotification
})

</script>



<template>
    <div class="notification notifywin" ref="notifyBox" @dblclick="hideNotification" v-show="show">
        <div class="notification-app-details">
            <div class="classy-icon"></div>
            <div class="app-name" ref="appName"></div>
        </div>
        <div class="notification-title" ref="title"></div>
        <div class="notification-subject" ref="subj"></div>
        <div class="notification-main">   
            <div class="notification-options">
                <div v-for="(item, index) in options" :key="index" class="notification-button">
                    {{ item }}
                </div>
            </div>
        </div>
    </div>
</template>
