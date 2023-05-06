import {defineStore} from 'pinia'
import {computed, ref} from "vue";

export const useSettingStore = defineStore('setting', {
    persist: true,
    state: () => {
        const bindAddr = computed(() => {
            return isBindGlobalAddr.value ? '0.0.0.0':'127.0.0.1'
        })
        const isBindGlobalAddr = ref(false)
        const toggleGlobalAddr = () => {
            isBindGlobalAddr.value = !isBindGlobalAddr.value
        }
        return {
            bindAddr,
            isBindGlobalAddr,
            toggleGlobalAddr
        }
    }
})