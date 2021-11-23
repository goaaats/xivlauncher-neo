#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod config;
mod game;
mod setup;
mod plugin;

use libxl::either;
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
    .invoke_handler(tauri::generate_handler![
      config::get_settings,
      config::get_addons,
      config::get_accounts,
      config::get_uid_cache,
      config::update_settings,
      config::update_addons,
      config::update_accounts,
      config::update_uid_cache,
      game::start_backup_tool,
      game::start_original_launcher,
      plugin::get_plugins,
      plugin::update_plugin,
      plugin::remove_plugin,
      setup::get_system_locale,
      setup::find_advanced_combat_tracker,
      // game::get_headline,
    ])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application")
}

fn setup_log() {
  let stdout = ConsoleAppender::builder().build();

  let is_debug = cfg!(debug_assertions);
  let backend_level_filter = either!(is_debug => LevelFilter::Debug; LevelFilter::Info);
  let requests_level_filter = either!(is_debug => LevelFilter::Debug; LevelFilter::Info);
  let stdout_level_filter = either!(is_debug => LevelFilter::Debug; LevelFilter::Warn);

  let requests = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    .build("log/requests.log")
    .unwrap();

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .appender(Appender::builder().build("requests", Box::new(requests)))
    .logger(Logger::builder().build("app::backend::db", backend_level_filter))
    .logger(
      Logger::builder()
        .appender("requests")
        .additive(false)
        .build("app::requests", requests_level_filter),
    )
    .build(Root::builder().appender("stdout").build(stdout_level_filter))
    .unwrap();

  log4rs::init_config(config).unwrap();
}
