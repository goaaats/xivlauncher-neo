use libxl::game::{integrity::{Integrity, IntegrityCheckModel}, repository::Repository};
use std::{path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, AtomicU32}, mpsc::{self, TryRecvError}}, thread, time::Duration};

fn get_game_path() -> PathBuf {
  let game_path = dbg!(env!("XL_TESTS_GAMEPATH"));
  Path::new(&game_path).to_path_buf()
}

#[test]
fn gen_integrity() {
  let game_path = get_game_path();
  let version = Repository::FFXIV.get_version(&game_path);

  let progress = Arc::new(AtomicU32::default());
  let report = IntegrityCheckModel::generate(&version, &game_path, progress, None);

  println!("{:#?}", report);
}

#[test]
fn check_integrity() {
  macro_rules! aw {
    ($e:expr) => {
      tokio_test::block_on($e)
    };
  }

  let start = std::time::Instant::now();

  let game_path = get_game_path();

  let ver = Repository::FFXIV.get_version(&game_path);
  let reference = aw!(Integrity::get_reference(ver)).expect("Could not get reference report.");
  let num_hashes = reference.hashes.len();

  let progress = Arc::new(AtomicU32::default());
  let thread_progress = Arc::clone(&progress);
  
  let do_progress = Arc::new(AtomicBool::new(true));
  let thread_do_progress = Arc::clone(&do_progress);

  thread::spawn(move || while thread_do_progress.load(std::sync::atomic::Ordering::SeqCst) {
    let prog = thread_progress.load(std::sync::atomic::Ordering::Relaxed);
    println!(
      "{:08}/{:08} ({:.2}%)",
      prog,
      num_hashes,
      prog as f32 / num_hashes as f32 * 100.0
    );  

    thread::sleep(Duration::from_millis(500));
  });

  let report = reference.check(&game_path, progress);

  do_progress.store(false, std::sync::atomic::Ordering::SeqCst);

  let elapsed = start.elapsed();
  println!("Hashes calculated in: {:.2?}", elapsed);
  println!("{:?}", report);
}
