<script setup lang="ts">
</script>

<template>
    <div class="time-context">
        <div class="notifications">
            <div class="notification">
                <div class="notification-title">From Nocutra Space</div>
                <div class="notification-main">
                    <div class="notification-subject">Hi!</div>
                    <div class="notification-options">
                        <div class="notification-button">Hey!</div>
                        <div class="notification-button">Goodbye!</div>
                    </div>
                </div>
            </div>
        </div>

        <div class="cards">
            <div class="deviceext card">
                <div class="device" data-on="true">
                    <img src="/tauri.svg" class="device-icon">
                    <div>30%</div>
                </div>
            </div>

            <div class="apps card"></div>
        </div>
    </div>
</template>

<style src="../../styles/contexts/time.css"></style>