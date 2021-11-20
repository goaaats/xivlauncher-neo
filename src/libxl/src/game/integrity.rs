use data_encoding::HEXLOWER;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
};
use walkdir::WalkDir;

const INTEGRITY_REPO_URL: &str = "https://goatcorp.github.io/integrity";

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrityCheckModel {
  #[serde(rename = "Hashes")]
  pub hashes: HashMap<String, String>,

  #[serde(rename = "GameVersion")]
  pub version: String,
}

impl IntegrityCheckModel {
  pub fn generate(version: &str, game_path: &Path) -> IntegrityCheckModel {
    let paths: Vec<PathBuf> = WalkDir::new(game_path)
      .into_iter()
      .filter_map(|v| v.ok())
      .filter(|v| v.path().is_file())
      .map(|x| x.path().to_path_buf())
      .collect();

    let hashes: HashMap<String, String> = paths
      .par_iter()
      .map(|x| {
        (
          x.to_str().unwrap().to_string(),
          IntegrityCheckModel::make_hash(x).expect("Could not generate hash."),
        )
      })
      .collect();

    Self {
      hashes,
      version: version.to_string(),
    }
  }

  fn make_hash(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(&path)?;
    let digest = crate::util::hash::sha1_digest(BufReader::new(file))?;

    Ok(HEXLOWER.encode(digest.as_ref()))
  }
}

pub struct Integrity {
  pub version: String,
  pub failed: Option<Vec<String>>,
  pub game_path: PathBuf,
}

impl Integrity {
  pub async fn check(version: String, game_path: &Path) -> Integrity {
    let reference = reqwest::get(format!("{}/{}.json", INTEGRITY_REPO_URL, version))
      .await
      .expect("Failed to get integrity report")
      .json::<IntegrityCheckModel>()
      .await
      .expect("Failed to parse integrity report");

    println!("Reference version: {:#?}", reference);

    let mut report = IntegrityCheckModel::generate(&version, game_path);
    let mut failed: Vec<String> = Vec::new();

    for (k, v) in report.hashes.iter_mut() {
      if reference.hashes.get(k).is_none() {
        failed.push(k.clone());
        continue;
      } else if v != reference.hashes.get(k).unwrap() {
        failed.push(k.clone());
      }
    }

    Integrity {
      version,
      failed: if !failed.is_empty() { Some(failed) } else { None },
      game_path: game_path.to_path_buf(),
    }
  }
}
