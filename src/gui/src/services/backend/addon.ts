import {invoke} from '@tauri-apps/api/tauri'

/**
 * Get the path to ACT installation, if it exists
 */
export async function getAdvancedCombatTrackerPath(): Promise<string> {
  return await invoke<string>('find_advanced_combat_tracker')
}
