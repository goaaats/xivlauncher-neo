import {invoke} from '@tauri-apps/api/tauri'

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
 * @param path: Shortcut path
 */
export async function createAccountShortcut(path: string) {
  return await invoke('create_account_shortcut', {path})
}
