use crate::{NeedsRuntime, RootPath, Settings};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

const REMOTE_BASE: &str = "https://goatcorp.github.io/dalamud-distrib/";

#[derive(Serialize, Deserialize, Debug)]
struct VersionInfo {
    #[serde(rename = "AssemblyVersion")]
    assembly: String,
    #[serde(rename = "SupportedGameVer")]
    supported_game: String,
    #[serde(rename = "RuntimeVersion")]
    runtime: String,
    #[serde(rename = "RuntimeRequired")]
    runtime_required: NeedsRuntime,
}

pub(crate) async fn update(
    root: &std::path::Path,
    settings: &Settings,
) -> Result<(String, std::path::PathBuf), Box<dyn Error>> {
    tracing::info!("[DUPDATE] Starting...");
    let url = format!(
        "{}{}version",
        REMOTE_BASE,
        if settings.testing.into() {
            beta_path(&settings.beta_kind)
        } else {
            ""
        }
    );

    // TODO (Chiv) Why does json::<VersionInfo> blow up/bytes() results in garbage?
    let remote_version_info = {
        let raw_json = reqwest::get(url).await?.text().await?;
        serde_json::from_str::<VersionInfo>(&raw_json)?
    };

    // TODO (Chiv) Is 'addon' legacy name and should be renamed?
    let addon = root.join("addon").join("Hooks");
    let current_version = addon.join(&remote_version_info.assembly);
    let runtime = root.join("runtime");
    let runtime_check = [
        runtime
            .join("host")
            .join("fxr")
            .join(&remote_version_info.runtime),
        runtime
            .join("shared")
            .join("Microsoft.NETCore.App")
            .join(&remote_version_info.runtime),
        runtime
            .join("shared")
            .join("Microsoft.WindowsDesktop.App")
            .join(&remote_version_info.runtime),
    ];

    tracing::info!(
        "[DUPDATE] Now starting for Dalamud {}",
        remote_version_info.assembly
    );

    // TODO (Chiv) The second branch is IsIntegrity in C#XL. DalamudUpdater.cs:103
    if !current_version.exists() || false {
        tracing::info!("[DUPDATE] Not found, redownloading");
        //TODO(Chiv) DalamudLoadingOverlay Progress is set here. Maybe we need a handle to send events back to the gui?
        download(&current_version, &settings).await?;
        //TODO (Chiv) Old Versions are now cleaned up

        // TODO(Chiv) in C# XL, the UID cache gets cleared here via a singleton call, but that is not
        //  really the place for it nor should the dalamud crate be linked to UID. Maybe the dalamud crate
        //  needs to communicate back whether a fresh download was needed?
        tracing::info!("[DUPDATE] Download OK!");
    }

    if remote_version_info.runtime_required.into() || settings.runtime.into() {
        tracing::info!(
            "[DUPDATE] Now starting for .NET Runtime {}",
            remote_version_info.runtime
        );
        if runtime_check.iter().any(|x| !x.exists()) {
            // TODO (Chiv) Overlay again set here
            download_runtime(&runtime, &remote_version_info.runtime).await?;
            tracing::info!("[DUPDATE] Download OK!")
        }
    }
    // TODO (Chiv) Integrity check again DalamudUpdater.cs:172
    //TODO Version write out, who is responsible?
    // TODO TOKIO async IO?
    // TODO Do we always write out the version?
    serde_json::to_writer(
        std::fs::File::create(current_version.join("version.json"))?,
        &remote_version_info,
    );
    tracing::info!(
        "[DUPDATE] All set for {}",
        remote_version_info.supported_game
    );
    Ok((
        remote_version_info.supported_game,
        current_version.join("Dalamud.Injector.exe"),
    ))
}

fn beta_path(beta_kind: &str) -> &str {
    //private static string GetBetaPath(DalamudSettings settings) =>
    //             string.IsNullOrEmpty(settings.DalamudBetaKind) ? "stg/" : $"{settings.DalamudBetaKind}/";
    if beta_kind.trim().is_empty() {
        "stg/"
    } else {
        beta_kind
    }
}

//TODO (CHiv) Types, especially error
pub(crate) async fn download(
    destination: &std::path::Path,
    settings: &Settings,
) -> Result<(), Box<dyn Error>> {
    //NOTE(Chiv) We dont care about deletion errors, they are either
    // (a) because the dir does not exists (we recreate it anyway) or
    // (b) because we do not have enough permissions, which we catch one line later or
    // (c) the path is no directory, which we catch later too, hopefully
    let _ = std::fs::remove_dir_all(destination);
    std::fs::create_dir_all(destination)?;
    // TODO (Chiv) C#XL downloads into a temporary file (created via UUID in the temp location).
    //  Maybe this needs to be recreated too.
    //let temporary = std::env::temp_dir().join(""/*UUID crate*/);
    let url = format!(
        "{}{}latest.zip",
        REMOTE_BASE,
        if settings.testing.into() {
            beta_path(&settings.beta_kind)
        } else {
            ""
        }
    );
    let response = reqwest::get(url).await?;
    // TODO (Chiv) This in helper method
    // TODO (Chiv) Tokio for async IO?
    // TODO (Chiv) Or just keep file in memory?
    let download = destination.join("latest.zip");
    // {
    //     let mut file = std::fs::File::create(&download)?;
    //     let mut stream = response.bytes_stream();
    //     while let Some(chunk) = stream.next().await.transpose()? {
    //         file.write(&chunk)?;
    //     }
    // }
    let mut bytes = std::io::Cursor::new(response.bytes().await?);
    extract(&mut bytes, &destination).await?;
    // TODO (Chiv) Copy/Extract newest dalamud to 'dev' folder too, DalamudUpdate.cs:260
    // TODO (Chiv) the download + extract methods should best be split up for that
    // TODO TMP, better to just reuse the zip file
    extract(&mut bytes, &destination.parent().unwrap().join("dev")).await?;
    //std::fs::remove_file(&download)?; //TODO (Chiv) In-memory vs file
    Ok(())
}

// TODO (Chiv) Async IO
async fn extract<R: std::io::Read + std::io::Seek>(
    file: R,
    destination: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(destination)?;
    // for i in 0..archive.len() {
    //     let mut file = archive.by_index(i)?;
    //     let outpath = match file.enclosed_name() {
    //         Some(path) => path.to_owned(),
    //         None => continue,
    //     };
    //
    //     {
    //         let comment = file.comment();
    //         if !comment.is_empty() {
    //             println!("File {} comment: {}", i, comment);
    //         }
    //     }
    //
    //     if (&*file.name()).ends_with('/') {
    //         println!("File {} extracted to \"{}\"", i, outpath.display());
    //         std::fs::create_dir_all(&outpath).unwrap();
    //     } else {
    //         println!(
    //             "File {} extracted to \"{}\" ({} bytes)",
    //             i,
    //             outpath.display(),
    //             file.size()
    //         );
    //         if let Some(p) = outpath.parent() {
    //             if !p.exists() {
    //                 std::fs::create_dir_all(&p).unwrap();
    //             }
    //         }
    //         let mut outfile = std::fs::File::create(&outpath).unwrap();
    //         std::io::copy(&mut file, &mut outfile).unwrap();
    //     }
    //
    //     // Get and Set permissions
    //     #[cfg(unix)]
    //         {
    //             use std::os::unix::fs::PermissionsExt;
    //
    //             if let Some(mode) = file.unix_mode() {
    //                 fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
    //             }
    //         }
    // }
    Ok(())
}

// TODO (Chiv) Unify download functions
pub(crate) async fn download_runtime(
    destination: &PathBuf,
    runtime_version: &str,
) -> Result<(), Box<dyn Error>> {
    let _ = std::fs::remove_dir_all(destination);
    std::fs::create_dir_all(destination)?;

    let dotnet_url = format!("https://dotnetcli.azureedge.net/dotnet/Runtime/{version}/dotnet-runtime-{version}-win-x64.zip",version=runtime_version);
    let desktop_url = format!("https://dotnetcli.azureedge.net/dotnet/WindowsDesktop/{version}/windowsdesktop-runtime-{version}-win-x64.zip",version=runtime_version);
    // TODO (Chiv) TMP file download in C#XL again
    let response = reqwest::get(dotnet_url).await?;
    extract(std::io::Cursor::new(response.bytes().await?), &destination).await?;
    let response = reqwest::get(desktop_url).await?;
    extract(std::io::Cursor::new(response.bytes().await?), &destination).await?;
    Ok(())
}
