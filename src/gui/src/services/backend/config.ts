import {invoke} from '@tauri-apps/api/tauri'

export type LauncherConfig = {
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
  addons: AddonEntry[],
  accounts: AccountEntry[],
  uid_cache: UidCacheEntry[],
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
 * Get the launcher configuration
 */
export async function getConfig(): Promise<LauncherConfig> {
  return await invoke<LauncherConfig>('get_config')
}

/**
 * Save the launcher config
 * @param config: Launcher configuration
 */
export async function saveConfig(config: LauncherConfig) {
  return await invoke('save_config', {config})
}
