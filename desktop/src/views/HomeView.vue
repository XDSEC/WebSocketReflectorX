<template>
    <div class="flex-1 flex flex-col justify-center items-center">
        <img alt="WSRX" :src="logo" class="w-24 h-24" />
        <h1 class="self-center text-xl mt-4 font-bold">WebSocket Reflector X</h1>
        <div class="input-group p-6 max-w-xl">
            <input class="input bg-base-content/5 backdrop-blur mt-8 flex-1" placeholder="[ws/wss]://..."
                v-model="newConnection" />
            <button class="btn bg-base-content/5 backdrop-blur mt-8 border-none">
                <send24-regular class="w-6 h-6" />
            </button>
        </div>
        <div class="h-24"></div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import logo from '../assets/logo.svg'
import {Send24Regular} from '@vicons/fluent'
const newConnection = ref('')

</script>
