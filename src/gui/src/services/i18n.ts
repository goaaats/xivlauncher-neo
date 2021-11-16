import {createI18n, I18n, Locale} from "vue-i18n"
import {Ref, ref} from "vue"

/**
 * I18n instance.
 */
const i18n: Ref<I18n | null> = ref(null)

/**
 * Default pre-loaded language.
 */
export const defaultLocale = "en"

/**
 * Supported languages.
 */
export const supportedLocales = [
    "de", "en", "es", "fr", "it",
    "ja", "ko", "no", "pt", "ru",
    "si", "tw", "zh",
]

/**
 * Tracks which languages have been loaded from file.
 */
const loadedLanguages = [defaultLocale]

/**
 * Setup an i18n instance.
 * @return i18n - instance.
 */
export async function setupI18n(): Promise<I18n> {
    const localeMessages = await loadLanguage(defaultLocale)
    const messages = {
        [defaultLocale]: localeMessages,
    }

    return i18n.value = createI18n({
        locale: defaultLocale,  // TODO: load default locale from settings
        fallbackLocale: defaultLocale,
        messages: messages,
    })
}

/**
 * Change the locale, loading if necessary.
 * @param lang - Two character language code.
 */
export async function setLanguage(lang: Locale): Promise<void> {
    if (i18n.value === null)
        throw "I18n has not been setup yet"

    if (!supportedLocales.find(sl => sl == lang))
        throw "Unsupported locale"

    if (i18n.value.global.locale === lang)
        return

    // If the language was already loaded
    if (!loadedLanguages.includes(lang)) {
        const messages = await loadLanguage(lang)
        i18n.value.global.setLocaleMessage(lang, messages)
        loadedLanguages.push(lang)
    }

    i18n.value.global.locale = lang
    document.querySelector("html")
        ?.setAttribute("lang", lang)
}

/**
 * Load a language
 * @param lang - Two character language code.
 * @return messages - Language messages.
 */
async function loadLanguage(lang: Locale) {
    const resp = await fetch(`/static/loc/xl_${lang}.json`)
    const json: { [k: string]: { [k: string]: [v: string] } } = await resp.json()
    return Object.fromEntries(
        Object.entries(json)
            .map(([key, val]) => ([key, val.message])))
}
