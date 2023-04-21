import {ref} from 'vue'
import {defineStore} from 'pinia'
import {v4 as uuidv4} from 'uuid'

export interface Message {
    id: string,
    level: 'none' | 'info' | 'success' | 'warning' | 'error'
    message: string
    persistTime?: number
    reject?: () => void
    rejectMessage?: string
    accept?: () => void
    acceptMessage?: string
}

export const useToastStore = defineStore('toast', {
    persist: false,
    state: () => {
        const messages = ref([] as Message[])
        const showMessage = (
            level: 'none' | 'info' | 'success' | 'warning' | 'error',
            message: string,
            persistTime?: number,
            reject?: () => void,
            rejectMessage?: string,
            accept?: () => void,
            acceptMessage?: string,
        ) => {
            const id = uuidv4()
            const msgObj = {
                id,
                level,
                message,
                persistTime,
                reject,
                rejectMessage,
                accept,
                acceptMessage,
            }
            messages.value.push(msgObj)
            if (persistTime && persistTime > 0) {
                setTimeout(() => {
                    messages.value = messages.value.filter((m) => m.id !== id)
                }, persistTime)
            }
            return id
        }
        const removeMessage = (id: string) => {
            messages.value = messages.value.filter((m) => m.id !== id)
        }
        const clearMessage = () => {
            messages.value = []
        }
        return {
            messages,
            showMessage,
            clearMessage,
            removeMessage,
        }
    },
})
