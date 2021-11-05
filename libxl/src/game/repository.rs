use std::{fs, path::{Path, PathBuf}};

use super::constants::DEFAULT_VERSION;

pub enum Repository {
    Boot,
}

impl Repository {
    pub fn get_path(&self, game_path: &Path) -> PathBuf {
        match self {
            Repository::Boot => game_path.join("boot").join("ffxivboot.ver"),
        }
    }

    pub fn get_version(&self, game_path: &Path) -> Result<String, std::io::Error> {
        // read text from file
        let ver = fs::read_to_string(self.get_path(game_path)).unwrap_or(DEFAULT_VERSION.to_string());
        Ok(ver)
    }
}