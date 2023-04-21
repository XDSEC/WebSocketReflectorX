<template>
    <div class="flex-1 flex flex-col p-8">
        <div class="flex flex-row items-center space-x-6">
            <img alt="WSRX" :src="logo" class="w-12 h-12" />
            <h1 class="self-center text-2xl font-bold">WebSocket Reflector X</h1>
        </div>
        <p class="text-sm mt-4 opacity-80">
            Idealism is that you will probably never receive something back,<br />
            but nonetheless still decide to give.<span class="blink text-primary">_</span>
        </p>
        <div class="flex-1"></div>
        <p class="text-sm opacity-60">{{ $t('authorTips') }}:
            <a href="https://github.com/Reverier-Xu" class="underline" target="_blank">Reverier-Xu</a>
            <a href="mailto:reverier.xu@woooo.tech" target="_blank">&nbsp;&lt;reverier.xu@woooo.tech&gt;</a>
        </p>
        <p class="text-sm opacity-60">{{ $t('versionTips') }}: {{ appVersion }}</p>
        <p class="text-sm opacity-60">{{ $t('licenseTips') }}: Mozilla Public License v2</p>
        <p class="text-sm opacity-60">{{ $t('repoTips') }}:
            <a href="https://github.com/XDSEC/WebSocketReflectorX" class="underline" target="_blank">
                github.com/XDSEC/WebSocketReflectorX
            </a>
        </p>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import logo from '../assets/logo.svg'
import { getVersion } from '@tauri-apps/api/app'
import '../styles/effect.scss'

const appVersion = ref('')

onMounted(() => {
    getVersion().then((version) => {
        appVersion.value = version
    })
})
</script>
