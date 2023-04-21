<template>
  <div :class="['w-full', 'h-full', 'flex', 'flex-row', isMaximized ? undefined : 'border border-base-content/10']">
    <side-bar></side-bar>
    <div class="flex-1 flex flex-col">
      <title-bar></title-bar>
      <router-view></router-view>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useThemeStore } from './stores/theme'
import SideBar from './components/SideBar.vue'
import TitleBar from './components/TitleBar.vue'
import { ref } from 'vue'
import { appWindow } from '@tauri-apps/api/window'

const isMaximized = ref(false)

const theme = useThemeStore()
theme.init()

window.addEventListener('resize', () => {
  appWindow.isMaximized().then((maximized) => {
    isMaximized.value = maximized
  })
})
document.addEventListener('contextmenu', event => event.preventDefault())
</script>
