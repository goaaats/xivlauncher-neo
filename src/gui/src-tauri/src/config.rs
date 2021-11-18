use std::sync::RwLock;
use log::debug;
use lazy_static::lazy_static;
use libxl::config::LauncherConfig;
use libxl::config_old::OldLauncherConfig;

lazy_static! {
  static ref CONFIG: RwLock<LauncherConfig> = RwLock::new(load_config());
}

fn load_config() -> LauncherConfig {
  match LauncherConfig::load() {
    Ok(config) => {
      debug!("Loaded config");
      return config;
    }
    Err(e) => {
      debug!("Could not load config: {:#?}", e);
    }
  }

  debug!("Attempting to upgrade old config");
  let old = match OldLauncherConfig::load() {
    Err(e) => {
      debug!("Could not load old config: {:#?}", e);
      debug!("Using default config");
      return LauncherConfig::default();
    }
    Ok(old) => {
      debug!("Loaded old config");
      old
    }
  };

  debug!("Attempting upgrade");
  match old.upgrade() {
    Ok(config) => {
      debug!("Config upgrade successful");
      config
    }
    Err(e) => {
      debug!("Upgrade failed: {:#?}", e);
      debug!("Using default config");
      LauncherConfig::default()
    }
  }
}
