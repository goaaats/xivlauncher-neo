import {createI18n, I18n} from 'vue-i18n'
import {Ref, ref} from 'vue'
import * as backend from '@/services/backend'

/**
 * I18n instance.
 */
const i18n = ref(null) as Ref<I18n | null>

/**
 * The default locale, as specified by the system locale, or
 * DEFAULT_LOCALE if not supported.
 */
let DEFAULT_LOCALE: string

/**
 * The fallback language.
 */
const FALLBACK_LOCALE = 'en'

/**
 * Supported languages.
 */
const SUPPORTED_LOCALES = [
  'de', 'en', 'es', 'fr', 'it',
  'ja', 'ko', 'no', 'pt', 'ru',
  'tw', 'zh',
]

/**
 * A mapping of ISO 639-1 locale language codes to an available language in English.
 */
export const LANGUAGE_LOCALE_MAP: { [k: string]: string } = {
  'ja': 'Japanese',
  'en': 'English',
  'de': 'German',
  'fr': 'French',
  'it': 'Italian',
  'es': 'Spanish',
  'pt': 'Portuguese',
  'ko': 'Korean',
  'no': 'Norwegian',
  'ru': 'Russian',
  'si': 'Sinhala',
  'tw': 'TraditionalChinese',
  'zh': 'SimplifiedChinese',
}

/**
 * A mapping of available launcher languages to their translated display value.
 */
const LAUNCHER_LANGUAGES: { [k: string]: string } = {
  'Japanese': '日本語',
  'English': 'English',
  'German': 'Deutsch',
  'French': 'Français',
  'Italian': 'Italiano',
  'Spanish': 'Español',
  'Portuguese': 'Português',
  'Korean': '한국어',
  'Norwegian': 'Norsk',
  'Russian': 'русский',
  'TraditionalChinese': '繁體中文',
  'SimplifiedChinese': '简体中文',
}

/**
 * A mapping of available game languages to their translated display value.
 */
const GAME_LANGUAGES: { [k: string]: string } = {
  'Japanese': '日本語',
  'English': 'English',
  'German': 'Deutsch',
  'French': 'Français',
}

/**
 * Tracks which languages have been loaded from file.
 */
const loadedLanguages: string[] = []

/**
 * Setup an i18n instance.
 * @return i18n - instance.
 */
export async function setupI18n(): Promise<I18n> {
  // Set the initial language based on the system locale
  // It will get set to the saved language later
  const systemLocale = await backend.getSystemLocale()

  let locale = systemLocale.split('-')[0]
  if (locale === 'zh') {
    // zh-cn is equivalent to our "zh"
    // So we only need to check for zh-tw
    if (systemLocale === 'zh-tw') {
      locale = 'tw'
    }
  }

  if (!(locale in SUPPORTED_LOCALES)) {
    locale = FALLBACK_LOCALE
  }

  DEFAULT_LOCALE = locale

  const localeMessages = await loadLanguage(locale)
  const messages = {
    [locale]: localeMessages,
  }

  const instance = createI18n({
    locale, messages, fallbackLocale: FALLBACK_LOCALE,
  })

  return i18n.value = instance as I18n
}

/**
 * Change the locale, loading if necessary.
 * @param language - Two character language code.
 */
export async function setLanguage(language: string) {
  if (i18n.value === null)
    throw 'I18n has not been setup yet'

  const validLanguages = Object.values(LANGUAGE_LOCALE_MAP)
  if (!validLanguages.includes(language))
    throw `Invalid language: ${language}`

  const locale = getKeyByValue(LANGUAGE_LOCALE_MAP, language)
  if (!SUPPORTED_LOCALES.includes(locale))
    throw `Unsupported locale: ${locale}`

  if (i18n.value.global.locale === locale)
    return

  // If the language was already loaded
  if (!loadedLanguages.includes(locale)) {
    const messages = await loadLanguage(locale)
    i18n.value.global.setLocaleMessage(locale, messages)
    loadedLanguages.push(locale)
  }

  i18n.value.global.locale = locale
  document.querySelector('html')
    ?.setAttribute('lang', locale)
}

/**
 * Load a language.
 * @param locale - Two character language code.
 * @return messages - Language messages.
 */
async function loadLanguage(locale: string) {
  const resp = await fetch(`/static/loc/xl_${locale}.json`)
  const localization = await resp.json() as LocalizationFileFormat

  // map { key : val.message }
  return Object.fromEntries(
    Object.entries(localization)
      .map(([key, val]) => ([key, val.message])))
}

/**
 * Localize a message.
 * @param message - Message to localize.
 */
export function t(message: string): string {
  if (!i18n.value)
    throw 'I18n has not been setup yet'

  return i18n.value.global.t(message)
}

/**
 * Get the default launcher language to be used when the setup
 * needs to be shown.
 */
export function getDefaultLauncherLanguageOption(): string {
  let lang = LANGUAGE_LOCALE_MAP[DEFAULT_LOCALE]
  if (!(lang in LAUNCHER_LANGUAGES)) {
    lang = LANGUAGE_LOCALE_MAP[FALLBACK_LOCALE]
  }

  return LAUNCHER_LANGUAGES[lang]
}

/**
 * Get the default game language to be used when the setup needs
 * to be shown.
 */
export function getDefaultGameLanguageOption(): string {
  let lang = LANGUAGE_LOCALE_MAP[DEFAULT_LOCALE]
  if (!(lang in GAME_LANGUAGES)) {
    lang = LANGUAGE_LOCALE_MAP[FALLBACK_LOCALE]
  }

  return GAME_LANGUAGES[lang]
}

/**
 * Get the display values for the launcher language.
 */
export function getLauncherLanguageOptions(): string[] {
  return Object.values(LAUNCHER_LANGUAGES)
}

/**
 * Get the display values for the game language.
 */
export function getGameLanguageOptions(): string[] {
  return Object.values(GAME_LANGUAGES)
}

/**
 * Convert a launcher language one of two ways:
 * - As a display option, converting to the setting value.
 * - As a setting value, converting to the display option.
 * @param value - Language value to convert.
 */
export function convertLauncherLanguage(value: string): string {
  if (Object.keys(LAUNCHER_LANGUAGES).includes(value))
    return LAUNCHER_LANGUAGES[value]

  if (Object.values(LAUNCHER_LANGUAGES).includes(value))
    return getKeyByValue(LAUNCHER_LANGUAGES, value)

  throw `Invalid conversion value: ${value}`
}

/**
 * Convert a game language one of two ways:
 * - As a display option, converting to the setting value.
 * - As a setting value, converting to the display option.
 * @param value - Language value to convert.
 */
export function convertGameLanguage(value: string): string {
  if (Object.keys(GAME_LANGUAGES).includes(value))
    return GAME_LANGUAGES[value]

  if (Object.values(GAME_LANGUAGES).includes(value))
    return getKeyByValue(GAME_LANGUAGES, value)

  throw `Invalid conversion value: ${value}`
}

function getKeyByValue(mapping: { [key: string]: string }, value: string): string {
  const result = Object.keys(mapping).find(key => mapping[key] === value)
  if (!result)
    throw `Invalid lookup value: ${value}`

  return result
}

type LocalizationFileFormat = {
  readonly [k: string]: {
    readonly message: string,
    readonly description: string,
  }
}
