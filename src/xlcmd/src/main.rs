use std::{fmt, fs, path::Path, slice::SliceIndex, sync::{
        atomic::AtomicU32,
        mpsc::{self, TryRecvError},
        Arc,
    }, thread, time::{Duration, Instant}};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use libxl::game::oauth::AccountRegion;

use clap::{AppSettings, Parser};

use console::{Emoji};
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

/// This is the XIVLauncher command line interface.
#[derive(Parser)]
#[clap(version = "1.0", author = "(c) XIVLauncher contributors")]
struct Opts {
    /// Generate an integrity report
    #[clap(long)]
    gen_integrity: bool,

    /// Path the game is installed to
    game_path: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let game_path = Path::new("E:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn");

    let started = Instant::now();
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    if opts.gen_integrity {
        let pb = ProgressBar::new(1);
        pb.set_style(spinner_style.clone());
        pb.enable_steady_tick(50);
        pb.set_message("Hashing file...");

        let version = libxl::game::repository::Repository::FFXIV.get_version(&game_path);

        let progress = Arc::new(AtomicU32::default());

        let (tx, rx) = mpsc::channel();

        let thread_progress: Arc<AtomicU32> = Arc::clone(&progress);

        let thread_pb = pb.clone();

        thread::spawn(move || loop {
            let prog = thread_progress.load(std::sync::atomic::Ordering::Relaxed);
            thread_pb.set_prefix(format!("[{}/?]", prog));

            thread::sleep(Duration::from_millis(500));
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        });

        let report = libxl::game::integrity::IntegrityCheckModel::generate(
            &version, &game_path, progress, None,
        );
        let _ = tx.send(());

        pb.finish_with_message(format!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed())));

        let json = serde_json::to_string_pretty(&report).unwrap();
        fs::write(format!("{}.json", version), json).expect("Unable to write report file.");

        return;
    }
}
