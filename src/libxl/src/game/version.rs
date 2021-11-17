use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use std::{
  fs::File,
  io::{BufReader, Read},
  path::Path,
};

use crate::patch::patchlist::PatchList;

use super::{
  constants,
  platform::Platform,
  repository::{get_ex_ver, Repository},
  request,
};

#[derive(Debug)]
pub enum VersionError {
  PathConversion,

  IOError(std::io::Error),
  Reqwest(reqwest::Error),
}

pub fn get_version_report(game_path: &Path, ex_level: u8) -> Result<String, VersionError> {
  let mut version_report = String::from(get_patch_gamever_hash(game_path)?);

  for n in 1..ex_level + 1 {
    let ex_ver = get_ex_ver(game_path, n);
    version_report.push_str(&format!("\nex{}\t{}", n, ex_ver));
  }

  Ok(version_report)
}

pub async fn check_boot_version(game_path: &Path, platform: Platform) -> Result<PatchList, VersionError> {
  let ver = Repository::Boot.get_version(game_path);
  let url = constants::patch_bootver_url(ver);
  let resp = request::patch_get(platform)
    .get(url)
    .send()
    .await
    .map_err(VersionError::Reqwest)?;
  let text = resp.text().await.map_err(VersionError::Reqwest)?;

  Ok(PatchList::from(text))
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
      output.push_str(",");
    }
  }

  Ok(output)
}

fn get_file_hash(path: &Path) -> Result<String, VersionError> {
  let file = File::open(&path).map_err(VersionError::IOError)?;
  let file_length = file.metadata().unwrap().len();
  let digest = sha1_digest(BufReader::new(file))?;

  let hex = HEXLOWER.encode(digest.as_ref());

  let file_name = path
    .file_name()
    .ok_or(VersionError::PathConversion)?
    .to_str()
    .ok_or(VersionError::PathConversion)?;
  Ok(format!("{}/{}/{}", file_name, file_length, hex))
}

fn sha1_digest<R: Read>(mut reader: R) -> Result<Digest, VersionError> {
  let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
  let mut buffer = [0; 1024];

  loop {
    let count = reader.read(&mut buffer).map_err(VersionError::IOError)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }

  Ok(context.finish())
}
