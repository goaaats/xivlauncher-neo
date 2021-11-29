import {invoke} from '@tauri-apps/api/tauri'

/**
 * Play the FF1 victory music out the system speaker
 */
export async function playVictoryBeep() {
  return await invoke('play_victory_beep')
}
