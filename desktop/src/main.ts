import { createApp } from 'vue'
import App from './App.vue'
import { createPinia } from 'pinia'
import router from './router'
import { i18n } from './i18n'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import './styles/base.scss'
import '@fontsource/jetbrains-mono'
import 'overlayscrollbars/overlayscrollbars.css'

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)
app.use(router)
app.use(i18n)

app.mount('#app')
