<script setup lang="ts">
let notifyBox = ref<HTMLElement | null>()
function hideNotification() {
    if (notifyBox.value instanceof HTMLElement) {
        notifyBox.value.remove()
    }
}
hideNotification()

</script>

<script lang="ts">
import { ref } from 'vue';

let title = ref<HTMLElement | null>()
let subj = ref<HTMLElement | null>()
let options= ref<string[]>([])
let show = ref<boolean>(false)

let notifyBox = ref<HTMLElement | null>()
function hideNotification() {
    options.value = []
    if (notifyBox.value instanceof HTMLElement) {
        notifyBox.value.remove()
    }
}

export function showNotification(app_title: string, subject: string, app_options: string[] | null = null) {
    hideNotification()
    if (!show.value) {
        show.value = true
    }
    if (title.value && subj.value) {
        title.value.innerHTML = `From ${app_title}`
        subj.value.innerHTML = subject
        if (app_options) {
            options.value = app_options
        }
    }
}

</script>

<template>
    <div class="notification notifywin" ref="notifyBox" @dblclick="hideNotification" v-show="show">
        <div class="notification-title" ref="title"></div>
        <div class="notification-main">
            <div class="notification-subject" ref="subj"></div>
            <div class="notification-options">
                <div v-for="(item, index) in options" :key="index" class="notification-button">
                    {{ item }}!
                </div>
            </div>
        </div>
    </div>
</template>
