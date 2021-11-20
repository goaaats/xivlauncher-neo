import {invoke} from "@tauri-apps/api/tauri"

/**
 * Get the path to ACT installation, if it exists
 */
export async function getAdvancedCombatTrackerPath(): Promise<string> {
    return await invoke<string>("find_advanced_combat_tracker")
}

/**
 * Get the system locale
 */
export async function getSystemLocale(): Promise<string> {
    return await invoke<string>("get_system_locale")
}

/**
 * Get the launcher settings
 */
export async function getSettings(): Promise<LauncherSettings> {
    return await invoke<LauncherSettings>("get_settings")
}

/**
 * Get the addons list
 */
export async function getAddons(): Promise<AddonEntry[]> {
    return await invoke<AddonEntry[]>("get_addons")
}

/**
 * Get the accounts list
 */
export async function getAccounts(): Promise<AccountEntry[]> {
    return await invoke<AccountEntry[]>("get_accounts")
}

/**
 * Get the UID cache
 */
export async function getUidCache(): Promise<UidCacheEntry[]> {
    return await invoke<UidCacheEntry[]>("get_uid_cache")
}

/**
 * Set the launcher settings
 * @param settings: Launcher settings
 */
export async function setSettings(settings: LauncherSettings) {
    return await invoke("update_settings", {settings: settings})
}

/**
 * Set the addons list
 * @param addons: Addons list
 */
export async function setAddons(addons: AddonEntry[]) {
    return await invoke("update_addons", {addons: addons})
}

/**
 * Set the accounts list
 * @param accounts: Accounts list
 */
export async function setAccounts(accounts: AccountEntry[]) {
    return await invoke("update_accounts", {accounts: accounts})
}

/**
 * Set the UID cache list
 * @param uidCache: UID cache list
 */
export async function setUidCache(uidCache: UidCacheEntry[]) {
    return await invoke("update_uid_cache", {uid_cache: uidCache})
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
    client_language: string,
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
