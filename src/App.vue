<script setup lang="ts">
import panel from "./components/panel.vue"
import topPanel from "./components/topPanel.vue"
import shellContext from "./components/contexts/shell.vue"
import notification from "./components/notify..vue"
import launcher from "./components/launcher.vue";
import { nyx_client } from "./webClient.ts";
import { showNotification } from "./components/notify..vue";
import osd from "./components/osd.vue";
import { onMounted, onUnmounted, ref } from "vue";
import "./styles/vars.css"
import "./styles/main.css"
import "./styles/notify.css"


let keys = new Set<string>()

let launcher_ref = ref<InstanceType<typeof launcher> | null>(null)
let top_ref = ref<InstanceType<typeof topPanel> | null>(null)
let osd_ref = ref<InstanceType<typeof osd> | null>(null)

let client = new nyx_client("Lepus")
client.connect()
client.OnReceive = (rawData) => {
  if (typeof rawData == "string") {
    let data = JSON.parse(rawData)
    if (Object.prototype.hasOwnProperty.call(data, "payload")) {
      if (Object.prototype.hasOwnProperty.call(data.payload, "request")) {
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

function keyUp(e: KeyboardEvent) {
  e.preventDefault()
  keys.delete(e.key)
  console.log([...keys])
}

function keydown(e: KeyboardEvent) {
  e.preventDefault()
  keys.add(e.key)
  console.log([...keys])
  shortcuts(keys)
}


function shortcuts(keys: Set<string>) {
  function has(key: string) {
    return [...keys].some(ckey => ckey === key)
  }
  let nKey = Number([...keys].find(ckey => /^[0-9]$/.test(ckey)));
  if (has("Super") && has("a")) {
    if (launcher_ref.value) {
      launcher_ref.value.showLauncher()
    }
  } else if (has("Super") && nKey && top_ref.value) {
    top_ref.value.activate(nKey)
  } else if (has("Super") && has("=") && top_ref.value) {
    top_ref.value.makeNewSpace()
  }
  if (osd_ref.value) {
    osd_ref.value.keyed(keys)
  }
}

onMounted(() => {
  window.addEventListener("keydown", keydown)
  window.addEventListener("keyup", keyUp)
})

onUnmounted(() => {
  window.removeEventListener("keydown", keydown)
  window.removeEventListener("keyup", keyUp)
})



</script>


<template>
  <topPanel ref="top_ref"/>
  <panel />
  <notification/>
  <launcher ref="launcher_ref"/>

  <shellContext />
  <osd ref="osd_ref"/>
</template>
