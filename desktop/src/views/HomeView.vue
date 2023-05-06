<template>
  <div class="flex-1 flex flex-col justify-center items-center">
    <img alt="WSRX" :src="logo" class="w-24 h-24"/>
    <h1 class="self-center text-xl mt-4 font-bold">WebSocket Reflector X</h1>
    <div class="input-group p-6 max-w-xl">
      <button class="btn bg-base-content/5 backdrop-blur mt-8 border-none tooltip font-normal"
              :data-tip="$t('bindAddr') + setting.bindAddr" @click="setting.toggleGlobalAddr">
        <globe24-regular class="w-6 h-6" v-if="setting.isBindGlobalAddr"/>
        <lock-closed24-regular class="w-6 h-6" v-else/>
      </button>
      <input class="input bg-base-content/5 backdrop-blur mt-8 flex-1" placeholder="[ws/wss]://..."
             v-model="newConnection"/>
      <button class="btn bg-base-content/5 backdrop-blur mt-8 border-none" @click="addConnection">
        <send24-regular class="w-6 h-6"/>
      </button>
    </div>
    <div class="h-12"></div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import logo from '../assets/logo.svg'
import {Send24Regular, LockClosed24Regular, Globe24Regular} from '@vicons/fluent'
import {invoke} from '@tauri-apps/api/tauri'
import {useRouter} from 'vue-router'
import {useToastStore} from '../stores/toast'
import {useSettingStore} from "../stores/setting";

const newConnection = ref('')
const router = useRouter()
const toast = useToastStore()
const setting = useSettingStore()


// check if the connection is a valid websocket url
const isValidUrl = (url: string) => {
  try {
    let urlObj = new URL(url)
    if (urlObj.protocol !== 'ws:' && urlObj.protocol !== 'wss:') {
      return false
    }
    return true
  } catch (e) {
    return false
  }
}

const addConnection = () => {
  if (isValidUrl(newConnection.value)) {
    invoke('add_ws_connection', {targetAddr: newConnection.value, bindAddr: setting.bindAddr}).then(() => {
      newConnection.value = ''
      router.push('/connections')
    }).catch((err) => {
      toast.showMessage('error', err, 5000)
    })
  } else {
    toast.showMessage('error', 'Invalid URL', 5000)
  }
}

</script>
