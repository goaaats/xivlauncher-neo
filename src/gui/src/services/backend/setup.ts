import {invoke} from '@tauri-apps/api/tauri'

/**
 * Get the system locale
 */
export async function getSystemLocale(): Promise<string> {
  return await invoke<string>('get_system_locale')
}
