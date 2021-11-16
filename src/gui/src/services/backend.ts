import {invoke} from "@tauri-apps/api/tauri"

export async function getAdvancedCombatTrackerPath(): Promise<string> {
    return await invoke<string>("find_advanced_combat_tracker")
}

export async function getSystemLocale(): Promise<string> {
    return await invoke<string>("get_system_locale")
}