import {createI18n} from 'vue-i18n'
// User defined lang
import enLocale from './en'
import zhLocale from './zh'

const messages = {
    en: {
        ...enLocale,
    },
    zh: {
        ...zhLocale,
    },
}

interface LanguageStore {
    code: string
}

const getLocale = () => {
    const languageStore = localStorage.getItem('language')
    if (languageStore) {
        const store = JSON.parse(languageStore) as LanguageStore
        return store.code
    }
    const language = navigator.language.toLowerCase()
    // console.log(language)
    const locales = Object.keys(messages)
    for (const locale of locales) {
        if (language.indexOf(locale) > -1) {
            return locale
        }
    }
    return 'en'
}

export const i18n = createI18n({
    locale: getLocale(),
    messages,
})
