use std::{fs, path::{Path, PathBuf}};

use super::constants::DEFAULT_VERSION;

pub enum Repository {
    Boot,
    FFXIV,
    Ex1,
    Ex2,
    Ex3,
    Ex4,
}

impl Repository {
    pub fn get_path(&self, game_path: &Path) -> PathBuf {
        match self {
            Repository::Boot => game_path.join("boot").join("ffxivboot.ver"),
            Repository::FFXIV => game_path.join("game").join("ffxivgame.ver"),
            Repository::Ex1 => game_path.join("game").join("sqpack").join("ex1").join("ex1.ver"),
            Repository::Ex2 => game_path.join("game").join("sqpack").join("ex2").join("ex2.ver"),
            Repository::Ex3 => game_path.join("game").join("sqpack").join("ex3").join("ex3.ver"),
            Repository::Ex4 => game_path.join("game").join("sqpack").join("ex4").join("ex4.ver"),
        }
    }

    pub fn get_version(&self, game_path: &Path) -> Result<String, std::io::Error> {
        // read text from file
        let ver = fs::read_to_string(self.get_path(game_path)).unwrap_or(DEFAULT_VERSION.to_string());
        Ok(ver)
    }
}