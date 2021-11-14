import {createI18n, I18n, Locale} from "vue-i18n"

/**
 * Default pre-loaded language
 */
export const defaultLocale = "en"

/**
 * Supported languages
 */
export const supportedLocales = [
    "de", "en", "es", "fr", "it",
    "ja", "ko", "no", "pt", "ru",
    "si", "tw", "zh"
]

/**
 * Tracks which languages have been loaded from file
 */
const loadedLanguages = [defaultLocale]

/**
 * Setup an i18n instance
 * @return i18n - instance
 */
export function setupI18n(): I18n {
    const messages = {
        [defaultLocale]: loadLanguage(defaultLocale)
    }

    return createI18n({
        locale: defaultLocale,  // TODO: load default locale from settings
        fallbackLocale: defaultLocale,
        messages: messages,
    })
}

/**
 * Change the locale, loading if necessary
 * @param i18n - I18n instance
 * @param lang - Two character language code
 */
export async function setLanguage(i18n: I18n, lang: Locale): Promise<void> {
    if (!supportedLocales.find(sl => sl == lang))
        throw "Unsupported locale"

    if (i18n.global.locale === lang)
        return

    // If the language was already loaded
    if (!loadedLanguages.includes(lang)) {
        const messages = await loadLanguage(lang)
        i18n.global.setLocaleMessage(lang, messages)
        loadedLanguages.push(lang)
    }

    i18n.global.locale = lang
    document.querySelector("html")!
        .setAttribute("lang", lang)
}

/**
 * Load a language
 * @param lang - Two character language code
 * @return messages - Language messages
 */
function loadLanguage(lang: Locale): Record<string, string> {
    const locJson = require(`@/assets/loc/xl_${lang}.json`)

    return Object.fromEntries(
        Object.entries(locJson).map(
            ([key, value]) =>
                ([key, locJson[key]["message"]])))
}
