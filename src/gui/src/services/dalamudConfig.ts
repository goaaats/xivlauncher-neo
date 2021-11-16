import {path} from "@tauri-apps/api";
import {readTextFile, writeFile} from "@tauri-apps/api/fs"
import {dataDir} from "@tauri-apps/api/path"

interface DalamudConfig {
    DoDalamudTest: boolean
    DoDalamudRuntime: boolean
    DalamudBetaKind: string
    OptOutMbCollection: boolean
}

/**
 * Dalamud configuration values.
 */
export let value: DalamudConfig | null = null
let filePath: string | null = null

/**
 * Load and cache the dalamud configuration file.
 */
export async function initialize() {
    if (value !== null)
        return

    filePath = await getPath()
    try {
        const configData = await readTextFile(filePath)
        value = JSON.parse(configData)
        return
    } catch (err) {
        console.log(err)
    }

    value = setupSettings()
}

async function getPath(): Promise<string> {
    const dir = await dataDir()
    return await path.join(dir, "XIVLauncher", "dalamudConfig.json")
}

function setupSettings(): DalamudConfig {
    return {
        DoDalamudTest: false,
        DoDalamudRuntime: false,
        DalamudBetaKind: "",
        OptOutMbCollection: false,
    }
}

