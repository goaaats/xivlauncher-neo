import {invoke} from '@tauri-apps/api/tauri'

/**
 * Get the system locale
 */
export async function getSystemLocale(): Promise<string> {
  return await invoke<string>('get_system_locale')
}

/**
 * Get the path to ACT installation, if it exists
 */
export async function getAdvancedCombatTrackerPath(): Promise<string> {
  return await invoke<string>('find_advanced_combat_tracker')
}
