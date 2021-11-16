import {path} from "@tauri-apps/api";
import {readTextFile, writeFile} from "@tauri-apps/api/fs"
import {dataDir} from "@tauri-apps/api/path"

interface LauncherConfigV3 {
    EncryptArguments: boolean
    GamePath: string
    IsDx11: boolean
    Language: string
    InGameAddonEnabled: boolean
    SteamIntegrationEnabled: boolean
    AddonList: string
    UniqueIdCacheEnabled: boolean
    AdditionalLaunchArgs: string
    CurrentAccountId: string
    AutologinEnabled: boolean
    AskBeforePatchInstall: boolean
    PatchPath: string
    KeepPatches: boolean
    DalamudInjectionDelayMs: number
    SpeedLimitBytes: number
    LauncherLanguage: string
    OptOutMbCollection: boolean
    OtpServerEnabled: boolean
    PatchAcquisitionMethod: string
    LastVersion: string
    HasShownAutoLaunchDisclaimer: boolean
    AcceptLanguage: string
}

/**
 * Launcher configuration values.
 */
export let value: LauncherConfigV3 | null = null
let filePath: string | null = null

/**
 * Load and cache the launcher configuration file.
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
    await save()
}

/**
 * Save the launcher configuration file.
 */
export async function save() {
    await writeFile({path: filePath!, contents: JSON.stringify(value)})
}

async function getPath(): Promise<string> {
    const dir = await dataDir()
    return await path.join(dir, "XIVLauncher", "launcherConfigV3.json")
}

function setupSettings(): LauncherConfigV3 {
    return {
        AcceptLanguage: "en-GB",
        AdditionalLaunchArgs: "",
        AddonList: "",
        AskBeforePatchInstall: true,
        AutologinEnabled: false,
        CurrentAccountId: "",
        DalamudInjectionDelayMs: 0,
        EncryptArguments: true,
        GamePath: "",
        HasShownAutoLaunchDisclaimer: false,
        InGameAddonEnabled: false,
        IsDx11: true,
        KeepPatches: false,
        Language: "English",
        LastVersion: "",
        LauncherLanguage: "English",
        OptOutMbCollection: false,
        OtpServerEnabled: false,
        PatchAcquisitionMethod: "",
        PatchPath: "",
        SpeedLimitBytes: 0,
        SteamIntegrationEnabled: false,
        UniqueIdCacheEnabled: false,
    }
}
