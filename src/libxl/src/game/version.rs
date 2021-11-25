use data_encoding::HEXLOWER;
use std::{
  fs::File,
  io::BufReader,
  path::Path,
};

use crate::{patch::patchlist::PatchList, util};

use super::{constants, platform::Platform, repository::Repository, request};

#[derive(Debug)]
pub enum VersionError {
  PathConversion,
  NoSession,

  IOError(std::io::Error),
  Reqwest(reqwest::Error),
}

pub fn get_version_report(game_path: &Path, ex_level: u8) -> Result<String, VersionError> {
  let mut version_report = get_patch_gamever_hash(game_path)?;

  for n in 1..ex_level + 1 {
    let ex_ver = Repository::get_ex_version(game_path, n);
    version_report.push_str(&format!("\nex{}\t{}", n, ex_ver));
  }

  Ok(version_report)
}

pub async fn check_boot_version(game_path: &Path, platform: Platform) -> Result<Option<PatchList>, VersionError> {
  let ver = Repository::Boot.get_version(game_path);
  let url = constants::patch_bootver_url(&ver);
  let resp = request::patch_get(platform)
    .get(url)
    .send()
    .await
    .map_err(VersionError::Reqwest)?;
  let text = resp.text().await.map_err(VersionError::Reqwest)?;

  Ok(if text.is_empty() {
    None
  } else {
    Some(PatchList::from(text))
  })
}

#[derive(Debug)]
pub struct SessionInfo {
  pub unique_id: String,
  pub patches: Option<PatchList>,
}

impl SessionInfo {
  pub async fn register_session(sid: &str, game_path: &Path, platform: Platform) -> Result<SessionInfo, VersionError> {
    let ver = get_patch_gamever_info(game_path)?;
    let url = constants::patch_gamever_url(&ver, &sid);

    let res = request::patch_get(platform)
      .get(url)
      .header("X-Hash-Check", "enabled")
      .send()
      .await
      .map_err(VersionError::Reqwest)?;

    if !res.status().is_success() {
      return Err(VersionError::NoSession);
    }

    let uid = res
      .headers()
      .get("X-Patch-Unique-Id")
      .ok_or(VersionError::NoSession)?
      .to_str()
      .unwrap()
      .to_string();

    let body = res.text().await.map_err(VersionError::Reqwest)?;

    Ok(SessionInfo {
      unique_id: uid,
      patches: if body.is_empty() {
        None
      } else {
        Some(PatchList::from(body))
      },
    })
  }
}

pub fn get_patch_gamever_info(game_path: &Path) -> Result<String, VersionError> {
  let ver = Repository::Boot.get_version(game_path);
  let hash = get_patch_gamever_hash(game_path)?;
  Ok(format!("{}={}", ver, hash))
}

fn get_patch_gamever_hash(game_path: &Path) -> Result<String, VersionError> {
  let mut output = String::new();
  let num_hashes = constants::PATCH_GAMEVER_HASHES.len();

  for n in 0..num_hashes {
    let bin_name = constants::PATCH_GAMEVER_HASHES[n];
    let hash = get_file_hash(&game_path.join("boot").join(bin_name))?;
    output.push_str(&hash);

    if n != num_hashes - 1 {
      output.push(',');
    }
  }

  Ok(output)
}

fn get_file_hash(path: &Path) -> Result<String, VersionError> {
  let file = File::open(&path).map_err(VersionError::IOError)?;
  let file_length = file.metadata().unwrap().len();
  let digest = util::hash::sha1_digest(BufReader::new(file)).map_err(VersionError::IOError)?;

  let hex = HEXLOWER.encode(digest.as_ref());

  let file_name = path
    .file_name()
    .ok_or(VersionError::PathConversion)?
    .to_str()
    .ok_or(VersionError::PathConversion)?;
  Ok(format!("{}/{}/{}", file_name, file_length, hex))
}
