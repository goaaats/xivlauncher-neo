use std::{fs, path::{Path, PathBuf}};

use crate::game::constants::DEFAULT_VERSION;

pub enum Repository {
    Boot,
    FFXIV,
    Ex1,
    Ex2,
    Ex3,
    Ex4,
}

pub fn get_ex_ver(game_path: &Path, idx: u8) -> String {
    read_ver(&get_ex_path(game_path, idx))
}

fn get_ex_path(game_path: &Path, idx: u8) -> PathBuf {
    game_path.join("game").join("sqpack").join(format!("ex{}", idx)).join(format!("ex{}.ver", idx))
}

fn read_ver(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or(DEFAULT_VERSION.to_string())
}

impl Repository {
    pub fn get_path(&self, game_path: &Path) -> PathBuf {
        match self {
            Repository::Boot => game_path.join("boot").join("ffxivboot.ver"),
            Repository::FFXIV => game_path.join("game").join("ffxivgame.ver"),
            Repository::Ex1 => get_ex_path(game_path, 1),
            Repository::Ex2 => get_ex_path(game_path, 1),
            Repository::Ex3 => get_ex_path(game_path, 1),
            Repository::Ex4 => get_ex_path(game_path, 1),
        }
    }

    pub fn get_version(&self, game_path: &Path) -> String {
        read_ver(&self.get_path(game_path))
    }
}