<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

const slider = ref<boolean>(false);
const option = ref<boolean>(false);
const osd = ref<boolean>(false);
const slide = ref<HTMLInputElement | null>(null);
const slide_number = ref<string>("null")
let options = [false, true, false]
const osd_element = ref<HTMLElement | null>(null)

const delayTime = (parseFloat(getComputedStyle(document.documentElement)
  .getPropertyValue("--reactTime")
  .trim()) * 1000) / 5;

let volume = 0
let brightness = 0

/* 
    Explanation for code above

        getComputedStyle(document.documentElement)
        .getPropertyValue("--reactTime").trim()

    This part basically just gets the reactTime variable (stored in css check ./styles/var.css)

        parseFloat
    
    This makes 0.5s (str) -> 0.5 (int)

        * 1000 then / 5

    * 1000 to convert seconds to ms and / 5 because in css we used:

        transition: all calc(var(--reactTime) / 5) ease;
*/

function showRangeOSD(range: number[], active: number, symbol: string | null) {
    if (!symbol) {
        symbol = "%"
    }
    slider.value = true
    console.log(slide.value)
    if (range.length == 2 && slide.value) {
        if (active > range[0] && active < range[1]) {
            slide.value.min = String(range[0])
            slide.value.max = String(range[1])
            slide.value.value = String(active)
            slide_number.value = `${active}${symbol}`
            osd.value = true
            osd_element.value?.classList.remove("deactivate")
        }
    }
}

function showDottedOSD(collection: boolean[]) {
    option.value = true
    options = collection
    osd.value = true
}

function hideOSD() {
    setTimeout(() => {
        osd_element.value?.classList.add("deactivate")
        console.log(delayTime)
        setTimeout(() => {
            osd.value = false
        }, delayTime)
    }, 2000)
}

function keyed(keys: Set<string>){
    function has(key: string) {
        return [...keys].some(ckey => ckey === key)
    }
    if (has("AudioVolumeUp")) {
        volume += 5
        if (volume > 100) {
            volume = 100
        }
        showRangeOSD([0, 100], volume, "%")
    }
    else if (has("AudioVolumeDown")) {
        volume -= 5
        if (volume < 0) {
            volume = 0
        }
        showRangeOSD([0, 100], volume, "%")
    }
    else if (has("AudioVolumeMute")) {
        volume = 0
        showRangeOSD([0, 100], volume, "%")
    }
    else if (has("BrightnessUp")) {
        brightness += 5
        if (brightness > 100) {
            brightness = 100
        }
        showRangeOSD([0, 100], brightness, "%")
    }
    else if (has("BrightnessDown")) {
        brightness -= 5
        if (brightness < 0) {
            brightness = 0
        }
        showRangeOSD([0, 100], brightness, "%")
    }
}




defineExpose({
    showRangeOSD,
    showDottedOSD,
    hideOSD,
    keyed
})
</script>

<template>
    <div class="osd" v-show="osd" ref="osd_element">
        <div class="slider-wrapper" v-show="slider"> 
            <div class="icon-osd"></div>
            <div class="slider-osd-container">
                <input type="range" min="1" max="100" class="slider-osd" ref="slide">
                <div class="slider-number"> {{ slide_number }}</div>
            </div>
        </div>
        <div class="option-wrapper" v-show="option">
            <div v-for="(item, index) in options" :key="index" :class="item === true ? 'dot-options active' : 'dot-options'"></div>
        </div>
    </div>

</template>

<style src="../styles/osd.css"></style>

