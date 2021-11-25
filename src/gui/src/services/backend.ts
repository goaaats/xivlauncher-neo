import {invoke} from '@tauri-apps/api/tauri'
import {i18n} from '@/services'

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
 * Get the path to ACT installation, if it exists
 */
export async function getAdvancedCombatTrackerPath(): Promise<string> {
  return await invoke<string>('find_advanced_combat_tracker')
}

/**
 * Get the system locale
 */
export async function getSystemLocale(): Promise<string> {
  return await invoke<string>('get_system_locale')
}

/**
 * Get the launcher settings
 */
export async function getSettings(): Promise<LauncherSettings> {
  return await invoke<LauncherSettings>('get_settings')
}

/**
 * Get the addons list
 */
export async function getAddons(): Promise<AddonEntry[]> {
  return await invoke<AddonEntry[]>('get_addons')
}

/**
 * Get the accounts list
 */
export async function getAccounts(): Promise<AccountEntry[]> {
  return await invoke<AccountEntry[]>('get_accounts')
}

/**
 * Get the UID cache
 */
export async function getUidCache(): Promise<UidCacheEntry[]> {
  return await invoke<UidCacheEntry[]>('get_uid_cache')
}

/**
 * Set the launcher settings
 * @param settings: Launcher settings
 */
export async function setSettings(settings: LauncherSettings) {
  return await invoke('update_settings', {settings: settings})
}

/**
 * Set the addons list
 * @param addons: Addons list
 */
export async function setAddons(addons: AddonEntry[]) {
  return await invoke('update_addons', {addons: addons})
}

/**
 * Set the accounts list
 * @param accounts: Accounts list
 */
export async function setAccounts(accounts: AccountEntry[]) {
  return await invoke('update_accounts', {accounts: accounts})
}

/**
 * Set the UID cache list
 * @param uidCache: UID cache list
 */
export async function setUidCache(uidCache: UidCacheEntry[]) {
  return await invoke('update_uid_cache', {uid_cache: uidCache})
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

export type LauncherSettings = {
  game_path: string,
  use_dx11: boolean,
  use_autologin: boolean,
  enable_uid_cache: boolean,
  extra_game_args: string,
  enable_dalamud: boolean,
  enable_otp_server: boolean,
  enable_steam_integration: boolean,
  game_language: string,
  launcher_language: string,
  current_account_id: string,
  encrypt_args: boolean,
  patch_path: string,
  ask_before_patching: boolean,
  download_speed_limit_bytes: number,
  dalamud_injection_delay_ms: number,
  keep_patches: boolean,
  opt_out_mb_collection: boolean,
  has_admin_complaints: boolean,
  last_version: string,
  has_shown_auto_launch_warning: boolean,
}

export type AddonEntry = {
  is_enabled: boolean,
  path: string,
  command_line: string,
  run_as_admin: boolean,
  run_on_close: boolean,
  kill_after_close: boolean,
}

export type AccountEntry = {
  character_name: string,
  character_world: string,
  thumbnail_url: string,
  username: string,
  save_password: boolean,
  use_steam: boolean,
  use_otp: boolean,
}

export type UidCacheEntry = {
  username: string,
  unique_id: string,
  region: number,
  expansion_level: number,
  creation_date: string,
}

/**
 * Get a list of all installed plugins
 */
export async function getPlugins(): Promise<PluginEntry[]> {
  return await invoke('get_plugins')
}

/**
 * Update the manifest of a single plugin
 * @param entry - Plugin entry
 */
export async function updatePlugin(entry: PluginEntry) {
  return await invoke('update_plugin', {entry: entry})
}

/**
 * Remove a single plugin
 * @param entry - Plugin entry
 */
export async function removePlugin(entry: PluginEntry) {
  return await invoke('remove_plugin', {entry: entry})
}

export type PluginEntry = {
  name: string,
  version: string,
  manifest_json: string,
}

/**
 * Open the Dalamud plugin dir in the local file explorer
 */
export async function openDalamudPluginDir(): Promise<string> {
  return await invoke('open_dalamud_plugin_dir')
}
