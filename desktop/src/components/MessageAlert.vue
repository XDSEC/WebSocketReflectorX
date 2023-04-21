<template>
  <div class="alert rounded-md bg-neutral shadow-lg relative">
    <div class="inline-flex flex-1">
      <info24-regular v-if="props.level === 'none' || props.level === 'info'" class="stroke-info w-6 h-6" />
      <warning24-regular v-if="props.level === 'warning'" class="stroke-warning w-6 h-6" />
      <checkmark-circle24-regular v-if="props.level === 'success'" class="stroke-success w-6 h-6" />
      <dismiss-circle24-regular v-if="props.level === 'error'" class="stroke-error w-6 h-6" />
      <span>{{ props.message }}</span>
      <div class="flex-1"></div>
      <button v-if="rejectMessage" class="btn btn-sm btn-ghost" @click="reject">{{ rejectMessage }}</button>
      <button v-if="acceptMessage" class="btn btn-sm btn-primary" @click="accept">{{ acceptMessage }}</button>
    </div>
    <div v-if="persistTime" class="absolute left-4 right-4 bottom-0 h-px">
      <div ref="progress" :class="[
        'w-full', 'h-full', 'border-b', 'transition-all', 'ease-linear',
        { 'border-b-error': props.level === 'error' },
        { 'border-b-warning': props.level === 'warning' },
        { 'border-b-success': props.level === 'success' },
        { 'border-b-info': props.level === 'info' || props.level === 'none' },
      ]" :style="{ transitionDuration: `${persistTime}ms` }"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useToastStore } from '../stores/toast'
import { Info24Regular, Warning24Regular, CheckmarkCircle24Regular, DismissCircle24Regular } from '@vicons/fluent'
import { onMounted, ref } from 'vue'

const toast = useToastStore()

const emits = defineEmits<{
  (e: 'reject'): void,
  (e: 'accept'): void,
}>()

const reject = () => {
  emits('reject')
  setTimeout(() => toast.removeMessage(props.id), 200)
}

const accept = () => {
  emits('accept')
  setTimeout(() => toast.removeMessage(props.id), 200)
}

const props = defineProps<{
  id: string,
  message: string,
  level: 'none' | 'info' | 'success' | 'warning' | 'error',
  rejectMessage?: string,
  acceptMessage?: string,
  persistTime?: number,
}>()

const progress = ref<HTMLElement>()

onMounted(() => {
  if (props.persistTime && props.persistTime > 0) {
    setTimeout(() => {
      progress.value?.classList.remove('w-full')
      progress.value?.classList.add('w-0')
    }, 100)
  }
})
</script>
  