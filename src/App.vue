<script setup lang="ts">
import panel from "./components/panel.vue"
import topPanel from "./components/topPanel.vue"
import shellContext from "./components/contexts/shell.vue"
import notification from "./components/notify..vue"
import launcher from "./components/launcher.vue";
import { nyx_client } from "./webClient.ts";
import { showNotification } from "./components/notify..vue";


let client = new nyx_client("Lepus")
client.connect()
client.OnReceive = (rawData) => {
  if (typeof rawData == "string") {
    let data = JSON.parse(rawData)
    if (data.payload) {
      if (data.payload.request) {
        switch (data.payload.request) {
          case "notify": {
            let request = data.payload
            if (Object.prototype.hasOwnProperty.call(request, "title") && Object.prototype.hasOwnProperty.call(request, "subject") && Object.prototype.hasOwnProperty.call(request, "options")) {
              try {
                showNotification(request.title, request.subject, request.options)
              }
              catch (e) {
                console.log("ERROR AT {NOTIFY}: ", e)
              }
            }
            break;
          }
          case "workspace": {}
        }
      }
    }
  }
}
</script>

<style src="./styles/vars.css"></style>
<style src="./styles/main.css"></style>
<style src="./styles/notify.css"></style>

<template>
  <topPanel />
  <panel />
  <notification/>
  <launcher/>

  <shellContext />
</template>
