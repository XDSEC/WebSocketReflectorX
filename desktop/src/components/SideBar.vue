<template>
    <div
        :class="['flex flex-col bg-base-content/5 p-0 pb-2 space-y-1', theme.sideBarExpanded ? 'w-64' : 'w-12']">
        <div :class="['btn no-animation m-0 btn-ghost rounded-none', theme.sideBarExpanded ? 'justify-start' : 'btn-sm']">
            <span v-if="theme.sideBarExpanded">WebSocket Reflector X</span>
            <img v-else alt="WSRX" :src="logo" class="w-5 h-5" />
        </div>
        <router-link
            :class="['btn no-animation btn-sm rounded-none btn-ghost text-base normal-case font-normal', theme.sideBarExpanded ? 'justify-start' : undefined]"
            to="/" active-class="active-btn">
            <home16-regular class="w-5 h-5" />
            <span class="ml-4" v-if="theme.sideBarExpanded">{{ $t('home') }}</span>
        </router-link>
        <router-link
            :class="['btn no-animation btn-sm rounded-none btn-ghost text-base normal-case font-normal', theme.sideBarExpanded ? 'justify-start' : undefined]"
            to="/connections" active-class="active-btn">
            <link16-regular class="w-5 h-5" />
            <span class="ml-4" v-if="theme.sideBarExpanded">{{ $t('connections') }}</span>
        </router-link>
        <router-link
            :class="['btn no-animation btn-sm rounded-none btn-ghost text-base normal-case font-normal', theme.sideBarExpanded ? 'justify-start' : undefined]"
            to="/settings" active-class="active-btn">
            <settings16-regular class="w-5 h-5" />
            <span class="ml-4" v-if="theme.sideBarExpanded">{{ $t('settings') }}</span>
        </router-link>
        <div class="flex-1"></div>
        <router-link class="btn no-animation btn-sm rounded-none btn-ghost text-base normal-case font-normal justify-start"
            to="/about" active-class="active-btn">
            <info16-regular class="w-5 h-5" />
            <span class="ml-4" v-if="theme.sideBarExpanded">{{ $t('about') }}</span>
        </router-link>
    </div>
</template>

<script setup lang="ts">
import { Home16Regular, Settings16Regular, Link16Regular, Info16Regular } from '@vicons/fluent'
import '../styles/effect.scss'
import { useThemeStore } from '../stores/theme'
import logo from '../assets/logo.svg'

const theme = useThemeStore()
</script>
