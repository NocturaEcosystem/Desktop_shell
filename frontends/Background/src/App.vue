<script setup lang="ts">
import "./styles/vars.css"
import "./styles/main.css"
import { nyx_client } from "./webClient";
import { readFile } from "@tauri-apps/plugin-fs";
import { ref } from "vue";

let currentBackgroundUrl: string | undefined;

async function setBackground(path: string) {
    try {
        const bytes = await readFile(path);
        const blob = new Blob([bytes], { type: "image/png" });
        const url = URL.createObjectURL(blob);

        document.body.style.backgroundImage = `url("${url}")`;
        document.body.style.backgroundSize = "cover";
        document.body.style.backgroundPosition = "center";
        document.body.style.backgroundRepeat = "no-repeat";

        document.body.style.backgroundColor = "";

        if (currentBackgroundUrl) {
            URL.revokeObjectURL(currentBackgroundUrl);
        }

        currentBackgroundUrl = url;
    } 
    catch (error) {
        console.error("Failed to load background:", error);

        document.body.style.removeProperty("background-image");

        const color = getComputedStyle(document.documentElement)
            .getPropertyValue("--superPrimaryContrast")
            .trim();

        document.body.style.backgroundColor = color;
        currentBackgroundUrl = undefined;
    }

}
let client = new nyx_client("Lepus_background");
client.connect()

client.OnReceive = (msg: unknown) => {
  try {
    let text: string;
    if (typeof msg === "string") {
      text = msg;
    } else if (msg instanceof ArrayBuffer) {
      text = new TextDecoder().decode(msg);
    } else if (ArrayBuffer.isView(msg)) {
      text = new TextDecoder().decode(msg);
    } else {
      console.error("Unsupported message type:", msg);
      return;
    }
    const data = JSON.parse(text);
    if (data.payload.Background) {
        setBackground(data.payload.Background)
    }
  } catch (err) {
    console.error("ERROR: couldnt parse data:", err);
  }
};

</script>

<template>
</template>
