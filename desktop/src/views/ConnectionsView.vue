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
                    <div class="text-base self-center opacity-60" v-if="activeConnections.length === 0">{{ $t('empty') }}</div>
                    <div v-for="connection in activeConnections" :key="connection.id"
                        class="flex flex-row rounded-sm hover:bg-base-content/5 backdrop-blur p-2 border-opacity-10 border-b border-base-content group">
                        <div class="flex flex-col flex-1">
                            <h2 class="flex flex-row flex-wrap items-center space-x-4">
                                <span>{{ connection.id }}</span>
                                <span class="opacity-80">-&gt; localhost:{{ connection.port }}</span>
                                <span class="rounded-full bg-base-content/5 pl-3 pr-3 text-sm text-success">
                                    {{ connection.latency }} ms
                                </span>
                                <span class="flex-1"></span>
                            </h2>
                            <p class="text-sm opacity-60">{{ connection.url }}</p>
                        </div>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex">
                            <copy20-regular class="w-5 h-5 text-success" />
                        </button>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex">
                            <dismiss20-regular class="w-5 h-5" />
                        </button>
                    </div>
                    <h1 class="text-base flex flex-row items-center !mt-8">
                        <plug-disconnected20-regular class="w-5 h-5 mr-2 text-warning" />
                        {{ $t('inactiveConnections') }}
                    </h1>
                    <div class="text-base self-center opacity-60" v-if="inactiveConnections.length === 0">{{ $t('empty') }}</div>
                    <div v-for="connection in inactiveConnections" :key="connection.id"
                        class="flex flex-row rounded-sm hover:bg-base-content/5 backdrop-blur p-2 border-opacity-10 border-b border-base-content group">
                        <div class="flex flex-col flex-1">
                            <h2 class="flex flex-row items-center space-x-4">
                                <span>{{ connection.id }}</span>
                                <span class="rounded-full bg-base-content/5 pl-3 pr-3 text-sm text-warning">
                                    {{ connection.latency }} ms
                                </span>
                            </h2>
                            <p class="text-sm opacity-60">{{ connection.url }}</p>
                        </div>
                        <button class="btn btn-sm btn-square btn-ghost hidden group-hover:inline-flex">
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
import { ref } from 'vue'

export interface Connection {
    id: string
    url: string
    port: number
    latency: number
}

const theme = useThemeStore()

const activeConnections = ref<Connection[]>([])
const inactiveConnections = ref<Connection[]>([])
</script>
