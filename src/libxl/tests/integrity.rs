use libxl::game::{integrity::{Integrity, IntegrityCheckModel}, repository::Repository};
use std::{
  path::{Path, PathBuf},
  sync::{
    atomic::AtomicU32,
    mpsc::{self, TryRecvError},
    Arc,
  },
  thread,
  time::Duration,
};

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

  let (tx, rx) = mpsc::channel();

  let thread_progress = progress.clone();

  thread::spawn(move || loop {
    let prog = thread_progress.load(std::sync::atomic::Ordering::Relaxed);
    println!(
      "{:08}/{:08} ({:.2}%)",
      prog,
      num_hashes,
      prog as f32 / num_hashes as f32 * 100.0
    );

    thread::sleep(Duration::from_millis(500));
    match rx.try_recv() {
      Ok(_) | Err(TryRecvError::Disconnected) => {
        println!("Terminating.");
        break;
      }
      Err(TryRecvError::Empty) => {}
    }
  });

  let report = reference.check(&game_path, progress);

  let _ = tx.send(());

  let elapsed = start.elapsed();
  println!("Hashes calculated in: {:.2?}", elapsed);
  println!("{:?}", report);
}
