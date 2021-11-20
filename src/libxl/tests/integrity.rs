use libxl::game::{
  integrity::Integrity,
  repository::Repository,
};
use std::path::{Path, PathBuf};

fn get_game_path() -> PathBuf {
  let game_path = dbg!(env!("XL_TESTS_GAMEPATH"));
  Path::new(&game_path).to_path_buf()
}

#[test]
fn gen_integrity() {
  macro_rules! aw {
    ($e:expr) => {
      tokio_test::block_on($e)
    };
  }

  let game_path = get_game_path();

  let ver = Repository::FFXIV.get_version(&game_path);
  let model = aw!(Integrity::check(ver, &game_path));
  return;

  match model.failed {
    Some(failed) => {
      println!("failed: {:?}", failed);
    }
    None => {
      println!("success");
    }
  }
}
