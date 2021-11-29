import {invoke} from '@tauri-apps/api/tauri'
import {i18n} from '@/services'

export type Headline = {
  news: HeadlineEntry[]
  topics: HeadlineEntry[]
  pinned: HeadlineEntry[]
  banner: BannerEntry[]
}

export type BannerEntry = {
  lsb_banner: string
  link: string
}

export type HeadlineEntry = {
  date: string,
  title: string,
  url: string,
  id: string,
  tag?: string,
}

/**
 * Get the headlines for the given language
 * @param langKey: Language key, see i18n module
 */
export async function getHeadline(langKey: string): Promise<Headline> {
  const code = i18n.getLangCode(langKey)
  return await invoke('get_headline', {langcode: code})
}

/**
 * Get the base64 image data for a banner image
 * @param url - Url to fetch
 */
export async function getBannerImageData(url: string): Promise<string> {
  return await invoke('get_banner_image_data', {url: url})
}

/**
 * Start the integrity tool
 */
export async function startIntegrityTool() {
  return await invoke('start_integrity_tool')
}

/**
 * Start the backup tool
 */
export async function startBackupTool() {
  return await invoke('start_backup_tool')
}

/**
 * Start the original launcher
 * @param useSteam: Use a steam account
 */
export async function startOriginalLauncher(useSteam: boolean) {
  return await invoke('start_original_launcher', {use_steam: useSteam})
}
