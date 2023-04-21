import {computed, ref} from 'vue'
import {defineStore} from 'pinia'
import { themes } from '../styles/theme'

export const useThemeStore = defineStore('theme', {
    persist: true,
    state: () => {
        const name = ref('cyber')
        const theme = ref('dark')
        const sideBarExpanded = ref(true)

        const fullStyle = computed(() => {
            return `${name.value}-${theme.value}`
        })

        const scrollbarStyle = computed(() => {
            if (theme.value === 'dark') {
                return 'os-theme-light'
            } else {
                return 'os-theme-dark'
            }
        })

        const themeDefs = computed(() => {
            return themes[fullStyle.value]
        })

        const setName = (styleName: string) => {
            name.value = styleName
            document.documentElement.setAttribute('data-theme', fullStyle.value)
        }

        const setTheme = (styleTheme: string) => {
            theme.value = styleTheme
            document.documentElement.setAttribute('data-theme', fullStyle.value)
        }

        const toggleTheme = () => {
            if (theme.value === 'dark') {
                theme.value = 'light'
            } else {
                theme.value = 'dark'
            }
            document.documentElement.setAttribute('data-theme', fullStyle.value)
        }

        const init = () => {
            document.documentElement.setAttribute('data-theme', fullStyle.value)
        }

        return {
            name,
            theme,
            sideBarExpanded,
            themeDefs,
            fullStyle,
            setName,
            setTheme,
            toggleTheme,
            scrollbarStyle,
            init,
        }
    }
})
