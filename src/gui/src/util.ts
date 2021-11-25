import {dialog, fs} from '@tauri-apps/api'

/**
 * Open a file/directory picker through tauri
 * @param directory: If the picker should be for a directory or file
 */
export async function showFileDialog(directory: boolean): Promise<string> {
  const result = await dialog.open({
    directory, multiple: false,
  })

  // This should never happen with multiple: false
  if (Array.isArray(result))
    throw 'Invalid result'

  return result
}

/**
 * Determine if the given path has "boot" and "game" sub directories
 * @param path: Path to inspect
 */
export async function isGamePathValid(path: string | null): Promise<boolean> {
  if (!path) {
    return false
  }

  return await fs.readDir(path).then((children) => {
    const bootExists = children.some((child) => child.name === 'boot')
    const gameExists = children.some((child) => child.name === 'game')
    return bootExists && gameExists
  }).catch(() => false)
}
