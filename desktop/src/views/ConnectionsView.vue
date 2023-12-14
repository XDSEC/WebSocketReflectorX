<template>
    <div class="flex-1 relative">
        <div class="absolute w-full h-full top-0 left-0">
            <overlay-scrollbars-component :options="{ scrollbars: { theme: theme.scrollbarStyle, autoHide: 'scroll' } }"
                class="w-full h-full print:h-auto print:overflow-auto" defer>
                <div class="p-8 flex flex-col space-y-3">
                    <h1 class="text-base flex flex-row items-center">
                        <flash20-regular class="w-5 h-5 mr-2 text-success" />
                        {{ $t('activeConnections') }}
                    </h1>
                    <div class="text-base self-center opacity-60" v-if="activeConnections.length === 0">{{ $t('empty') }}
                    </div>
                    <div v-for="connection in activeConnections" :key="connection.id"
                        class="flex flex-row rounded-sm hover:bg-base-content/5 backdrop-blur p-2 border-opacity-10 border-b border-base-content group">
                        <div class="flex flex-col flex-1">
                            <h2 class="flex flex-row flex-wrap items-center space-x-4">
                                <span>{{ connection.id }}</span>
                                <span class="opacity-80">-&gt; localhost:{{ connection.port }}</span>
                                <span class="rounded-full bg-base-content/5 pl-3 pr-3 text-sm text-success">
                                    {{ connection.latency === 0 ? "--" : connection.latency }} ms
                                </span>
                                <span class="flex-1"></span>
                            </h2>
                            <p class="text-sm opacity-60">{{ connection.url }}</p>
                        </div>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex"
                            @click="copyLocalLink(`localhost:${connection.port}`)">
                            <copy20-regular class="w-5 h-5 text-success" />
                        </button>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex"
                            @click="closeConnection(connection.id)">
                            <dismiss20-regular class="w-5 h-5" />
                        </button>
                    </div>
                    <h1 class="text-base flex flex-row items-center !mt-8">
                        <plug-disconnected20-regular class="w-5 h-5 mr-2 text-warning" />
                        {{ $t('inactiveConnections') }}
                    </h1>
                    <div class="text-base self-center opacity-60" v-if="inactiveConnections.length === 0">{{ $t('empty') }}
                    </div>
                    <div v-for="connection in inactiveConnections" :key="connection.id"
                        class="flex flex-row rounded-sm hover:bg-base-content/5 backdrop-blur p-2 border-opacity-10 border-b border-base-content group">
                        <div class="flex flex-col flex-1">
                            <h2 class="flex flex-row items-center space-x-4">
                                <span>{{ connection.id }}</span>
                                <span class="rounded-full bg-base-content/5 pl-3 pr-3 text-sm text-warning">
                                    {{ connection.latency === 0 ? "--" : connection.latency }} ms
                                </span>
                            </h2>
                            <p class="text-sm opacity-60">{{ connection.url }}</p>
                        </div>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex"
                            @click="closeConnection(connection.id)">
                            <dismiss20-regular class="w-5 h-5" />
                        </button>
                    </div>
                </div>
            </overlay-scrollbars-component>
        </div>
    </div>
</template>

<script setup lang="ts">
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'
import { useThemeStore } from '../stores/theme'
import { Flash20Regular, Dismiss20Regular, Copy20Regular, PlugDisconnected20Regular } from '@vicons/fluent'
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useToastStore } from '../stores/toast'
import { writeText } from '@tauri-apps/api/clipboard'


export interface Connection {
    id: string
    url: string
    port: number
    latency: number
}

const theme = useThemeStore()
const toast = useToastStore()

const activeConnections = ref<Connection[]>([])
const inactiveConnections = ref<Connection[]>([])
let timer = 0

const refreshConnections = () => {
    invoke('refresh_latency').then(() => {
        invoke('get_alive_connections').then((connections) => {
            activeConnections.value = JSON.parse(connections as string)
        }).catch((err) => {
            toast.showMessage('error', err, 5000)
        })
        invoke('get_dead_connections').then((connections) => {
            inactiveConnections.value = JSON.parse(connections as string)
        }).catch((err) => {
            toast.showMessage('error', err, 5000)
        })
    }).catch((err) => {
        toast.showMessage('error', err, 5000)
    })
}

const closeConnection = (id: string) => {
    invoke('close_connection', { id }).then(() => {
        refreshConnections()
    }).catch((err) => {
        toast.showMessage('error', err, 5000)
    })
}

const copyLocalLink = (url: string) => {
    writeText(url).then((res) => {
        toast.showMessage('success', 'Local link copied!', 5000)
    }).catch((err) => {
        toast.showMessage('error', 'Copy failed', 5000)
    })
}

onMounted(() => {
    invoke('get_alive_connections').then((connections) => {
        activeConnections.value = JSON.parse(connections as string)
    }).catch((err) => {
        toast.showMessage('error', err, 5000)
    })
    invoke('get_dead_connections').then((connections) => {
        inactiveConnections.value = JSON.parse(connections as string)
    }).catch((err) => {
        toast.showMessage('error', err, 5000)
    })
    invoke('refresh_latency')

    timer = setInterval(() => {
        refreshConnections()
    }, 10000) as unknown as number
})

onBeforeUnmount(() => {
    clearInterval(timer)
})
</script>
