pub const APP_NAME: &str = "costa";
pub const STORE_PATH: &str = "store";
pub const DEFAULT_HOST: &str = "http://127.0.0.1:9081";
pub const DEFAULT_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "9081";
pub const DEFAULT_PATH: &str = "/costa";
#[cfg(debug_assertions)]
pub const IS_DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const IS_DEBUG: bool = false;
