<script setup lang="ts">
import { ref } from 'vue';
let notifyBox = ref<HTMLElement | null>()
function hideNotification() {
    if (notifyBox.value instanceof HTMLElement) {
        notifyBox.value.remove()
    }
}
</script>

<template>
    <div class="notification notifywin" ref="notifyBox" @dblclick="hideNotification">
        <div class="notification-title">From Nocutra Space</div>
        <div class="notification-main">
            <div class="notification-subject">Hi!</div>
            <div class="notification-options">
                <div class="notification-button">Hey!</div>
                <div class="notification-button">Goodbye!</div>
            </div>
        </div>
    </div>
</template>
