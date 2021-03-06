#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod commands;
mod lib;

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
      commands::config::get_settings,
      commands::config::get_addons,
      commands::config::get_accounts,
      commands::config::get_uid_cache,
      commands::config::update_settings,
      commands::config::update_addons,
      commands::config::update_accounts,
      commands::config::update_uid_cache,
      commands::game::get_headline,
      commands::game::get_banner_image_data,
      commands::game::start_backup_tool,
      commands::game::start_original_launcher,
      commands::plugin::get_plugins,
      commands::plugin::update_plugin,
      commands::plugin::remove_plugin,
      commands::plugin::open_dalamud_plugin_dir,
      commands::setup::get_system_locale,
      commands::setup::find_advanced_combat_tracker,
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
