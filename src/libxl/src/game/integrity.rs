use data_encoding::HEXLOWER;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
  sync::{
    atomic::AtomicU32,
    Arc,
  },
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
  pub fn check(&self, game_path: &Path, progress: Arc<AtomicU32>) -> Integrity {
    println!("Reference version: {:#?}", self.version);

    let mut report = IntegrityCheckModel::generate(&self.version, game_path, progress, Some(self));
    let mut failed: Vec<String> = Vec::new();

    let mut has_missing = false;
    for (k, v) in report.hashes.iter_mut() {
      if self.hashes.get(k).is_none() {
        failed.push(k.clone());
        has_missing = true;
      } else if v != self.hashes.get(k).unwrap() {
        failed.push(k.clone());
      }
    }

    if report.hashes.is_empty() {
      has_missing = true;
    }

    Integrity {
      version: self.version.clone(),
      failed_files: if !failed.is_empty() { Some(failed) } else { None },
      game_path: game_path.to_path_buf(),
      missing_files: has_missing
    }
  }

  pub fn generate(version: &str, game_path: &Path, progress: Arc<AtomicU32>, reference: Option<&IntegrityCheckModel>) -> IntegrityCheckModel {
    let path_len = game_path.to_path_buf().to_str().unwrap().len();

    let paths: Vec<PathBuf> = WalkDir::new(game_path)
      .into_iter()
      .filter_map(|v| v.ok())
      // We only want files
      .filter(|v| v.path().is_file())
      // If we have a reference, we only want files that are in the reference
      .filter(|v| if let Some(reference) = reference {
        reference.hashes.contains_key(&v.path().to_str().unwrap()[path_len..])
      } else {
        true
      })
      .map(|x| x.path().to_path_buf())
      .collect();

    let hashes: HashMap<String, String> = paths
      .par_iter()
      .map(|x| {
        let result = (
          x.to_str().unwrap()[path_len..].to_string(),
          IntegrityCheckModel::make_hash(x).expect("Could not generate hash."),
        );

        progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        result
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

#[derive(Debug)]
pub struct Integrity {
  pub version: String,
  pub failed_files: Option<Vec<String>>,
  pub game_path: PathBuf,
  pub missing_files: bool,
}

impl Integrity {
  pub async fn get_reference(version: String) -> Result<IntegrityCheckModel, reqwest::Error> {
    Ok(
      reqwest::get(format!("{}/{}.json", INTEGRITY_REPO_URL, version))
        .await?
        .json::<IntegrityCheckModel>()
        .await?
    )
  }
}
