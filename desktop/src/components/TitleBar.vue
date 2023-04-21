<template>
    <div data-tauri-drag-region class="flex flex-row h-8">
        <button class="btn no-animation btn-sm m-0 btn-ghost rounded-none" @click="theme.sideBarExpanded = !theme.sideBarExpanded">
            <navigation16-regular class="w-4 h-4" />
        </button>
        <div data-tauri-drag-region class="flex-1"></div>
        <button class="btn no-animation btn-sm m-0 btn-ghost rounded-none" @click="theme.toggleTheme">
            <weather-sunny16-regular class="w-4 h-4" v-if="theme.theme === 'light'" />
            <weather-moon16-regular class="w-4 h-4" v-else />
        </button>
        <button class="btn no-animation btn-sm m-0 btn-ghost rounded-none" @click="minimize">
            <subtract16-regular class="w-5 h-5" />
        </button>
        <button class="btn no-animation btn-sm m-0 btn-ghost rounded-none" @click="toggleMaximize">
            <square-multiple16-regular class="w-4 h-4" v-if="isMaximized" />
            <maximize16-regular class="w-4 h-4" v-else />
        </button>
        <button class="btn no-animation btn-sm btn-error hover:bg-red-500 m-0 btn-ghost rounded-none" @click="close">
            <dismiss16-regular class="w-4 h-4" />
        </button>
    </div>
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import { Dismiss16Regular, WeatherSunny16Regular, WeatherMoon16Regular, Subtract16Regular, Maximize16Regular, SquareMultiple16Regular, Navigation16Regular } from '@vicons/fluent'
import { ref } from 'vue'
import { useThemeStore } from '../stores/theme'

const close = () => appWindow.close()
const isMaximized = ref(false)
const toggleMaximize = () => appWindow.toggleMaximize()
const minimize = () => appWindow.minimize()
const theme = useThemeStore()

window.addEventListener('resize', () => {
    appWindow.isMaximized().then((maximized) => {
        isMaximized.value = maximized
    })
})

</script>
