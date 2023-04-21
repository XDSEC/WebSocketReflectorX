<template>
    <transition-group class="toast toast-end" name="slide-fade" tag="div">
      <message-alert v-for="toast in toasts.messages" :id="toast.id" :key="toast.id" :accept-message="toast.acceptMessage"
                     :level="toast.level" :message="toast.message" :persist-time="toast.persistTime" :reject-message="toast.rejectMessage"
                     class="transition-all" @accept="toast.accept"
                     @reject="toast.reject"/>
    </transition-group>
  </template>
  
  <script lang="ts" setup>
  import {useToastStore} from '../stores/toast'
  import MessageAlert from './MessageAlert.vue'
  import '../styles/toast.scss'
  
  const toasts = useToastStore()
  </script>
  