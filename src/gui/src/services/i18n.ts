import {createI18n, I18n} from 'vue-i18n'
import {Ref, ref} from 'vue'
import {backend, log} from '@/services'

/**
 * I18n instance.
 */
const i18n = ref(null) as Ref<I18n | null>

/**
 * The default locale, as specified by the system locale, or
 * DEFAULT_LOCALE if not supported.
 */
let DEFAULT_CODE: string

class Language {
  /** Configuration key */
  public key: string

  /** Translated display option */
  public option: string

  /** 2 character pseudo-locale */
  public code: string

  /** Is this a game language */
  public isGame: boolean

  /** // Has this been loaded */
  public isLoaded: boolean

  constructor(key: string, option: string, code: string, isGame = false) {
    this.key = key
    this.option = option
    this.code = code
    this.isGame = isGame
    this.isLoaded = false
  }
}

const SUPPORTED_LANGUAGES: Language[] = [
  new Language('Japanese', '日本語', 'ja', true),
  new Language('English', 'English', 'en', true),
  new Language('German', 'Deutsch', 'de', true),
  new Language('French', 'Français', 'fr', true),
  new Language('Italian', 'Italiano', 'it'),
  new Language('Spanish', 'Español', 'es'),
  new Language('Portuguese', 'Português', 'pr'),
  new Language('Korean', '한국어', 'ko'),
  new Language('Norwegian', 'Norsk', 'no'),
  new Language('Russian', 'русский', 'ru'),
  new Language('TraditionalChinese', '繁體中文', 'tw'),
  new Language('SimplifiedChinese', '简体中文', 'zh'),
]

const FALLBACK_LANGUAGE = SUPPORTED_LANGUAGES
  .find((i) => i.code == 'en') as Language

/**
 * Setup an i18n instance.
 * @return i18n - instance.
 */
export async function setupI18n(): Promise<I18n> {
  // Set the initial language based on the system locale
  // It will get set to the saved language later
  const systemLocale = await backend.getSystemLocale()

  let code = systemLocale.split('-')[0]
  if (code === 'zh') {
    // zh-cn is equivalent to our "zh"
    // So we only need to check for zh-tw
    if (systemLocale === 'zh-tw') {
      code = 'tw'
    }
  }

  const supported = SUPPORTED_LANGUAGES.find((lang) => lang.code == code)
  if (!supported)
    code = FALLBACK_LANGUAGE.code

  DEFAULT_CODE = code

  const localeMessages = await loadLanguage(code)
  const messages = {[code]: localeMessages}
  const instance = createI18n({
    locale: code, messages, fallbackLocale: FALLBACK_LANGUAGE.code,
  })

  return i18n.value = instance as I18n
}

/**
 * Change the locale, loading if necessary.
 * @param langKey - Language key, the English native word
 */
export async function setLanguage(langKey: string) {
  if (!i18n.value)
    throw 'I18n has not been setup yet'

  const lang = SUPPORTED_LANGUAGES.find((lang) => lang.key == langKey)
  if (!lang)
    throw `Invalid language: ${langKey}`

  if (i18n.value.global.locale === lang.code)
    // Already set
    return

  // If the language was already loaded
  if (!lang.isLoaded) {
    const messages = await loadLanguage(lang.code)
    i18n.value.global.setLocaleMessage(lang.code, messages)
    lang.isLoaded = true
  }

  i18n.value.global.locale = lang.code
  document.querySelector('html')
    ?.setAttribute('lang', lang.code)
}

/**
 * Load a language.
 * @param code - Two character language code.
 * @return messages - Language messages.
 */
async function loadLanguage(code: string): Promise<{ [k: string]: string }> {
  const file = `xl_${code}.json`
  log.debug(`Loading localization ${file}`)

  const resp = await fetch(`/static/loc/${file}`)
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
  let defaultLang = SUPPORTED_LANGUAGES
    .find((lang) => lang.code == DEFAULT_CODE)

  if (!defaultLang)
    defaultLang = FALLBACK_LANGUAGE

  return defaultLang.option
}

/**
 * Get the default game language to be used when the setup needs
 * to be shown.
 */
export function getDefaultGameLanguageOption(): string {
  let defaultLang = SUPPORTED_LANGUAGES
    .find((lang) => lang.isGame && lang.code == DEFAULT_CODE)

  if (!defaultLang)
    defaultLang = FALLBACK_LANGUAGE

  return defaultLang.option
}

/**
 * Get the display values for the launcher language.
 */
export function getLauncherLanguageOptions(): string[] {
  return SUPPORTED_LANGUAGES
    .map((lang) => lang.option)
}

/**
 * Get the display values for the game language.
 */
export function getGameLanguageOptions(): string[] {
  return SUPPORTED_LANGUAGES
    .filter((lang) => lang.isGame)
    .map((lang) => lang.option)
}

/**
 * Convert a launcher language one of two ways:
 * - As a display option, converting to the setting value.
 * - As a setting value, converting to the display option.
 * @param value - Language value to convert.
 */
export function convertLauncherLanguage(value: string): string {
  const lang1 = SUPPORTED_LANGUAGES
    .find((lang) => lang.option == value)
  if (lang1) return lang1.key

  const lang2 = SUPPORTED_LANGUAGES
    .find((lang) => lang.key == value)
  if (lang2) return lang2.option

  throw `Invalid conversion value: ${value}`
}

/**
 * Convert a game language one of two ways:
 * - As a display option, converting to the setting value.
 * - As a setting value, converting to the display option.
 * @param value - Language value to convert.
 */
export function convertGameLanguage(value: string): string {
  const lang1 = SUPPORTED_LANGUAGES
    .filter((lang) => lang.isGame)
    .find((lang) => lang.option == value)
  if (lang1) return lang1.key

  const lang2 = SUPPORTED_LANGUAGES
    .filter((lang) => lang.isGame)
    .find((lang) => lang.key == value)
  if (lang2) return lang2.option

  throw `Invalid conversion value: ${value}`
}

/**
 * Get the two character lang code from the key value
 * @param langKey: Language key
 */
export function getLangCode(langKey: string): string {
  const lang = SUPPORTED_LANGUAGES.find((lang) => lang.key == langKey)
  if (!lang)
    throw `Invalid language: ${langKey}`

  return lang.code
}

type LocalizationFileFormat = {
  readonly [k: string]: {
    readonly message: string,
    readonly description: string,
  }
}
