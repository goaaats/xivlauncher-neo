use crate::OptOutMbCollection;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::os::windows::process::CommandExt;
use std::path::Path;

#[derive(Serialize)]
struct StartInfo {
    #[serde(rename = "WorkingDirectory")]
    working_directory: String,
    #[serde(rename = "ConfigurationPath")]
    configuration_path: String,
    #[serde(rename = "PluginDirectory")]
    plugin_directory: String,
    #[serde(rename = "DefaultPluginDirectory")]
    dev_plugin_directory: String,
    #[serde(rename = "AssetDirectory")]
    asset_directory: String,
    #[serde(rename = "Language")] // TODO Dalamud seriliased Language startinfo is a number...
    language: u32,
    #[serde(rename = "GameVersion")]
    game_version: String,
    #[serde(rename = "OptOutMbCollection")]
    opt_out_mb_collection: crate::OptOutMbCollection,
}

// TODO(Chiv): The whole libxl just for ClientLanguage seems a bit too much of a dependency
//  -> Types in own crate or duplicate type?
pub(crate) async fn launch(
    root: &std::path::Path,
    game: &std::path::Path,
    injector: &std::path::Path, // TODO newtypes for the paths
    assets: &std::path::Path,
    supported_version: String,
    language: libxl::game::ClientLanguage,
    opt_out_mb_collection: OptOutMbCollection,
    game_process_id: usize,
) -> Result<std::process::Child, Box<dyn Error>> {
    tracing::info!(
        "[HOOKS] DalamudLauncher::Run(gp:{}, cl:{}",
        game.display(),
        language
    );
    // TODO (CHIV) C#XL check for VcRedist now, do we still need with tauri's msi installer?
    let config = root.join("dalamudConfig.json");
    let plugins = root.join("installedPlugins");
    let dev_plugins = root.join("devPlugins");
    std::fs::create_dir_all(&plugins)?;
    std::fs::create_dir_all(&dev_plugins)?;

    // TODO (Chiv) async sleep/delay apparently needs a runtime <.</own implementation

    // TODO (Chiv) C#XL shows and hides overlay depending on the DalamudUpdater progress
    //  But in our case, as of now, that is already done
    // TODO (Chiv) C#XL recheck the version by reading the json, we can just pass it in, cant we?

    // TODO (chiv) second thing we need from libXL, that alongside types to its own crate?
    let actual_version = libxl::game::repository::Repository::FFXIV.get_version(game);
    if actual_version != supported_version {
        tracing::error!(
            "[HOOKS] Version mismatch! Actual: {}, supported: {}",
            &actual_version,
            &supported_version
        );
        return Err("Version mismatch!")?;
    }

    let root = root
        .canonicalize()?
        .into_os_string()
        .into_string()
        .expect("TODO");

    // TODO (Chiv) canonicalize chokes if config json does not exist, dalamud chokes if its empty
    let config = if !config.exists() {
        std::fs::write(&config, "")?;
        let t = config
            .canonicalize()?
            .into_os_string()
            .into_string()
            .expect("TODO");
        std::fs::remove_file(config);
        t
    } else {
        config
            .canonicalize()?
            .into_os_string()
            .into_string()
            .expect("TODO")
    };

    // TODO BIG (Chiv) Paths are butchered and not how Dalamud wants them.
    // TODO (CHIV) Canonicalize does not seem to do what I expect it to
    //TODO (Chiv) This should be passed in and the path construct only once on callsite?
    let start_info = StartInfo {
        working_directory: injector
            .parent()
            .unwrap()
            .canonicalize()?
            //.as_os_str()
            //.to_os_string()
            .into_os_string()
            .into_string()
            .expect("TODO"),
        configuration_path: config,
        plugin_directory: plugins
            .canonicalize()?
            .into_os_string()
            .into_string()
            .expect("TODO"),
        dev_plugin_directory: dev_plugins
            .canonicalize()?
            .into_os_string()
            .into_string()
            .expect("TODO"),
        asset_directory: assets
            .canonicalize()?
            //.as_os_str()
            //.to_os_string()
            .into_os_string()
            .into_string()
            .expect("TODO"),
        language: 1, // TODO ENUM number serliaztion
        game_version: actual_version,
        opt_out_mb_collection,
    };

    // TODO TO CHECK IF THAT _IS_ THE PROBLEM
    let start_info = StartInfo {
        working_directory: start_info.working_directory.replace("\\\\?\\", ""),
        configuration_path: start_info.configuration_path.replace("\\\\?\\", ""),
        plugin_directory: start_info.plugin_directory.replace("\\\\?\\", ""),
        dev_plugin_directory: start_info.dev_plugin_directory.replace("\\\\?\\", ""),
        asset_directory: start_info.asset_directory.replace("\\\\?\\", ""),
        ..start_info
    };

    let start_info = serde_json::to_string(&start_info)?;
    tracing::debug!("STARTINFO {}", &start_info);
    let start_info = base64::encode(start_info);
    tracing::debug!("STARTINFO BASE64 {}", &start_info);

    //TODO
    tracing::debug!("SPAWNING {:?}", &injector);
    // TODO (Chiv) Tokyo async process::Command?
    let child = if cfg!(windows) {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        std::process::Command::new(
            injector
                .canonicalize()?
                .into_os_string()
                .into_string()
                .expect("TODO")
                .replace("\\\\?\\", ""),
        )
        .current_dir(
            injector
                .parent()
                .unwrap()
                .canonicalize()?
                .into_os_string()
                .into_string()
                .expect("TODO")
                .replace("\\\\?\\", ""),
        ) // TODO duplicate
        .arg(game_process_id.to_string())
        .arg(start_info)
        .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
        .spawn()?
    } else {
        std::process::Command::new(injector)
            .current_dir(injector.parent().unwrap().canonicalize()?) // TODO duplicate
            .arg(game_process_id.to_string())
            .arg(start_info)
            .spawn()?
    };

    tracing::info!("[HOOKS] Started dalamud!");
    Ok(child)
}

async fn re_check_version(
    game: &std::path::Path,
    version: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
    todo!()
}
