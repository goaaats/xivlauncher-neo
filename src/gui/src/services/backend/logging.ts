import {invoke} from '@tauri-apps/api/tauri'

export async function debug(msg: string) {
  await invoke('debug', {msg})
}

export async function info(msg: string) {
  await invoke('info', {msg})
}

export async function warn(msg: string) {
  await invoke('warn', {msg})
}

export async function error(msg: string) {
  await invoke('error', {msg})
}
