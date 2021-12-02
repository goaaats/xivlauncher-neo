import {invoke} from '@tauri-apps/api/tauri'

export type PluginEntry = {
  name: string,
  version: string,
  manifest_json: string,
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

/**
 * Open the Dalamud plugin dir in the local file explorer
 */
export async function openDalamudPluginDir(): Promise<string> {
  return await invoke('open_dalamud_plugin_dir')
}
