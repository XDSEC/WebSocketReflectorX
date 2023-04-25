<template>
    <div class="flex-1 flex flex-col space-y-3">
        <h1 class="text-base flex flex-row items-center p-8 pb-0">
            <code20-regular class="w-5 h-5 mr-2 text-info" />
            {{ $t('logs') }}
        </h1>
        <div class="flex-1 relative">
            <div class="absolute top-0 left-0 w-full h-full">
                <overlay-scrollbars-component :options="{ scrollbars: { theme: theme.scrollbarStyle, autoHide: 'scroll' } }"
                    class="w-full h-full print:h-auto print:overflow-auto p-8 pt-0" defer>
                    <div class="flex flex-col space-y-1">
                        <div v-for="log in logs"
                            class="flex flex-row rounded-sm hover:bg-base-content/5 backdrop-blur p-2 border-opacity-10 border-b border-base-content items-center">
                            <info16-regular class="w-8 h-8 m-3 text-info" v-if="log.level === 'info'" />
                            <error-circle16-regular class="w-8 h-8 m-3 text-error" v-else />
                            <div class="flex flex-col flex-1">
                                <h2 class="text-base flex-1">
                                    {{ log.message }}
                                </h2>
                                <p class="text-base opacity-60 flex-1">{{ log.addr }}</p>
                            </div>
                        </div>
                    </div>
                </overlay-scrollbars-component>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { useThemeStore } from '../stores/theme'
import { invoke } from '@tauri-apps/api/tauri'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'
import { Info16Regular, ErrorCircle16Regular, Code20Regular } from '@vicons/fluent'

const theme = useThemeStore()

interface Log {
    level: 'error' | 'info'
    addr: string
    message: string
}

const logs = ref<Log[]>([])
let timer = 0

onMounted(() => {
    invoke('get_logs').then(resp => {
        logs.value = JSON.parse(resp as string)
    })

    timer = setInterval(() => {
        invoke('get_logs').then(resp => {
            logs.value = JSON.parse(resp as string)
        })
    }, 1000) as unknown as number
})

onBeforeUnmount(() => {
    clearInterval(timer)
})
</script>
