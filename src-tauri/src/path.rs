use std::path::{Path, PathBuf};

use debug_print::debug_println;
use tauri::api::path;

use crate::constant::{APP_NAME, STORE_PATH};

pub fn get_user_home() -> PathBuf {
  path::home_dir().unwrap()
}

pub fn get_app_data_dir() -> PathBuf {
  let mut path = get_user_home();
  path.push(".".to_owned() + APP_NAME);
  path
}

pub fn get_sotre_path() -> PathBuf {
  get_app_data_dir().join(STORE_PATH)
}