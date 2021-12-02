import {invoke} from '@tauri-apps/api/tauri'
import {AccountEntry} from '@/services/backend/config'

/**
 * Get the profile picture URL for a given character
 * @param name: Character name
 * @param server: Character server
 */
export async function getCharacterProfilePictureUrl(name: string, server: string): Promise<string> {
  return await invoke('get_profile_picture_url', {name, server})
}

/**
 * Create a shortcut to start the launcher with a specific account
 * @param entry: Account to create a shortcut of
 */
export async function createAccountShortcut(entry: AccountEntry) {
  return await invoke('create_account_shortcut', {entry})
}
