#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

#[tokio::main]
async fn main() {
  setup_log();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}

fn setup_log() {
  let stdout = ConsoleAppender::builder().build();

  let requests = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    .build("log/requests.log")
    .unwrap();

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .appender(Appender::builder().build("requests", Box::new(requests)))
    .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
    .logger(
      Logger::builder()
        .appender("requests")
        .additive(false)
        .build("app::requests", LevelFilter::Info),
    )
    .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
    .unwrap();

  log4rs::init_config(config).unwrap();
}
