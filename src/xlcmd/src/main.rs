use std::{
    fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU32},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use dialoguer::{theme::ColorfulTheme, Input, Password};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

use clap::Parser;

use console::Emoji;
use libxl::game::region::AccountRegion;
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":)");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", ":(");

/// This is the XIVLauncher command line interface.
#[derive(Parser)]
#[clap(version = "1.0", author = "(c) XIVLauncher contributors")]
struct Opts {
    /// Generate an integrity report
    #[clap(long)]
    gen_integrity: bool,

    /// Steam service account
    #[clap(long)]
    steam_service: bool,

    /// Path the game is installed to, defaults to XL_TESTS_GAMEPATH
    game_path: Option<String>,

    /// Region to use
    #[clap(long, default_value = "Europe")]
    region: AccountRegion,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let provided_path = opts
        .game_path
        .or_else(|| std::env::var("XL_TESTS_GAMEPATH").ok())
        .expect("No game path specified in args or env!");

    let game_path = Path::new(&provided_path);

    println!("=> Game at \"{}\"", game_path.display());

    let started = Instant::now();
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    if opts.gen_integrity {
        let pb = ProgressBar::new(1);
        pb.set_style(spinner_style);
        pb.enable_steady_tick(50);
        pb.set_message("Hashing file...");

        let thread_pb = pb.clone();

        let version = libxl::game::repository::Repository::FFXIV.get_version(game_path);

        let progress = Arc::new(AtomicU32::default());
        let thread_progress: Arc<AtomicU32> = Arc::clone(&progress);

        let do_progress = Arc::new(AtomicBool::new(true));
        let thread_do_progress = Arc::clone(&do_progress);

        thread::spawn(move || {
            while thread_do_progress.load(std::sync::atomic::Ordering::SeqCst) {
                let prog = thread_progress.load(std::sync::atomic::Ordering::Relaxed);
                thread_pb.set_prefix(format!("[{}/?]", prog));

                thread::sleep(Duration::from_millis(500));
            }
        });

        let report = libxl::game::integrity::IntegrityCheckModel::generate(
            &version, game_path, progress, None,
        );

        do_progress.store(false, std::sync::atomic::Ordering::SeqCst);

        pb.finish_with_message(format!(
            "{} Done in {}",
            SPARKLE,
            HumanDuration(started.elapsed())
        ));

        let json = serde_json::to_string_pretty(&report).unwrap();
        fs::write(format!("{}.json", version), json).expect("Unable to write report file.");

        return;
    }

    let pb = ProgressBar::new(1);
    pb.set_style(spinner_style.clone());
    pb.enable_steady_tick(50);
    pb.set_message("Checking boot version...");

    let boot_check =
        libxl::game::version::check_boot_version(game_path, libxl::game::platform::Platform::Win32)
            .await
            .expect("Could not check boot version.");

    if boot_check.is_some() {
        pb.finish_with_message(format!("{} Boot patch required!", ERROR));
        return;
    }

    pb.finish_with_message(format!("{} Boot up to date!", SPARKLE));

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                    force = Some(input.clone());
                    Err("Please enter your SE account name, not your email address.")
                } else {
                    Ok(())
                }
            }
        })
        .interact_text()
        .unwrap();

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let otp: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("OTP")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if !input.is_empty() && input.len() != 6
                    || force.as_ref().map_or(false, |old| old == input)
                {
                    force = Some(input.clone());
                    Err("Not a valid OTP.")
                } else {
                    Ok(())
                }
            }
        })
        .interact_text()
        .unwrap();

    let pb = ProgressBar::new(1);
    pb.set_style(spinner_style.clone());
    pb.enable_steady_tick(50);
    pb.set_message(format!("Logging in as {}...", username));

    let oauth = libxl::game::oauth::OauthLogin::login(
        &username,
        &password,
        &otp,
        libxl::game::platform::Platform::Win32,
        opts.steam_service,
        opts.region,
    )
    .await;

    match oauth {
        Ok(oauth) => {
            pb.finish_with_message(format!(
                "{} Done in {}",
                SPARKLE,
                HumanDuration(started.elapsed())
            ));

            if !oauth.can_play
            {
                println!("You can't play with this account.");
                return;
            }

            if !oauth.terms_accepted
            {
                println!("You must accept the terms of service.");
                return;
            }

            let pb = ProgressBar::new(1);
            pb.set_style(spinner_style.clone());
            pb.enable_steady_tick(50);
            pb.set_message("Registering session...");
            
            let session = libxl::game::version::SessionInfo::register_session(&oauth.session_id, game_path, libxl::game::platform::Platform::Win32).await.unwrap();

            pb.finish();

            if session.patches.is_some() {
                println!("You must patch the game.");
                return;
            }

            println!("UID: {}", session.unique_id);
        }
        Err(err) => {
            pb.finish_with_message(format!(
                "{} Failed in {}\n{:?}",
                ERROR,
                HumanDuration(started.elapsed()),
                err
            ));
        }
    }

    
}
