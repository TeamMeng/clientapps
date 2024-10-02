use std::path::PathBuf;

const APP_DIR: &str = ".hn";
const LOG_DIR: &str = "log";
const CACHE_DIR: &str = "cache";
const DB_DIR: &str = "db";
const CONFIG_DIR: &str = "config";

#[inline]
pub fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(APP_DIR)
}

#[inline]
pub fn log_dir() -> PathBuf {
    app_dir().join(LOG_DIR)
}

#[allow(unused)]
#[inline]
pub fn cache_dir() -> PathBuf {
    app_dir().join(CACHE_DIR)
}

#[allow(unused)]
#[inline]
pub fn db_dir() -> PathBuf {
    app_dir().join(DB_DIR)
}

#[allow(unused)]
#[inline]
pub fn config_dir() -> PathBuf {
    app_dir().join(CONFIG_DIR)
}
