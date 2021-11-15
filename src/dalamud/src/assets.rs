use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::error::Error;
use std::io::Write;

const ASSET_STORE_URL: &str = "https://goatcorp.github.io/DalamudAssets/";
const TEMP_ASSET_JSON: &str = include_str!("assets.json");

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    #[serde(rename = "Url")]
    url: String,
    #[serde(rename = "FileName")]
    file_name: String,
    #[serde(rename = "Hash")]
    hash: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Info {
    #[serde(rename = "Version")]
    version: u32,
    #[serde(rename = "Assets")]
    assets: Vec<Asset>,
}

//TODO Types
pub(crate) async fn ensure(assets: &std::path::Path) -> Result<(), Box<dyn Error>> {
    tracing::trace!("[DASSET] Starting asset download");
    let assets_version = assets.join("asset.ver");
    let (needed_refresh, info) = is_refresh_needed(&assets_version).await?;

    for entry in info.assets {
        let destination = assets.join(&entry.file_name);
        // NOTE (Chiv) Unwrap is okay cause we always joined a file name
        std::fs::create_dir_all(&destination.parent().unwrap())?;
        // TODO (Chiv) Refactor this...thing (local mutable block?)
        let refresh = if let Some(entry_hash) = entry.hash {
            if let Ok(mut f) = std::fs::File::open(&destination) {
                let mut sha1 = sha1::Sha1::default();
                if let Ok(_) = std::io::copy(&mut f, &mut sha1) {
                    // TODO EXISTING HASH CHECK NEEDS TESTING
                    let mut hash = format!("{:x}", sha1.finalize());
                    hash.make_ascii_uppercase();
                    // TODO (Chiv) Hash of UIRes/logo.png is wrong
                    //dbg!(&destination);
                    //dbg!(&hash);
                    //dbg!(&entry_hash);
                    //dbg!(entry_hash != hash)
                    entry_hash != hash
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        if !destination.exists() || needed_refresh || refresh {
            tracing::trace!(
                "[DASSET] Downloading {} to {}...",
                &entry.url,
                &entry.file_name
            );
            let response = reqwest::get(&entry.url).await?;
            // TODO (Chiv) Tokio for async IO?
            {
                let mut file = std::fs::File::create(&destination)?;
                let mut stream = response.bytes_stream();
                while let Some(chunk) = stream.next().await.transpose()? {
                    file.write(&chunk)?;
                }
            }
        }
    }
    // TODO Tokio async IO?
    if needed_refresh {
        std::fs::write(&assets_version, format!("{}", info.version))?
    }
    tracing::trace!("[DASSET] Assets OK");
    Ok(())
}

async fn is_refresh_needed(assets: &std::path::Path) -> Result<(bool, Info), Box<dyn Error>> {
    let local_version = if let Ok(Ok(v)) = std::fs::read_to_string(assets).map(|s| s.parse::<u32>())
    {
        v
    } else {
        tracing::error!("[DASSET] Could not read asset.ver");
        0
    };

    // TODO (Chiv) Why does json::<VersionInfo> blow up/bytes() results in garbage? <- Does it here too?
    let remote_info = {
        let raw_json = reqwest::get(format!("{}asset.json", ASSET_STORE_URL))
            .await?
            .text()
            .await?;
        // TODO (Chiv) The asset.json upstream is malformatted and serde chokes. Wonder why Newton does not.
        //serde_json::from_str::<Info>(&raw_json)?
        serde_json::from_str::<Info>(TEMP_ASSET_JSON)?
    };
    tracing::trace!(
        "[DASSET] Ver check - local:{} remote:{}",
        local_version,
        remote_info.version
    );
    Ok((remote_info.version > local_version, remote_info))
}
