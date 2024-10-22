use std::path::PathBuf;

use strum_macros::{Display, EnumString};

#[derive(EnumString, Debug, Display)]
pub enum CostaRouter {
    #[strum(to_string = "/home")]
    Home,
    #[strum(to_string = "/schema_editor")]
    UrlEdit,
    #[strum(to_string = "/download_app")]
    DownloadApp,
    #[strum(to_string = "/env_edit")]
    EnvEdit,
}

impl From<CostaRouter> for String {
    fn from(id: CostaRouter) -> Self {
        id.to_string()
    }
}

impl From<String> for CostaRouter {
    fn from(id: String) -> Self {
        match id.as_str() {
            "/home" => CostaRouter::Home,
            "/schema_editor" => CostaRouter::UrlEdit,
            "/download_app" => CostaRouter::DownloadApp,
            "/env_edit" => CostaRouter::EnvEdit,
            _ => CostaRouter::Home,
        }
    }
}

impl From<CostaRouter> for PathBuf {
    fn from(id: CostaRouter) -> Self {
        PathBuf::from(String::from(id))
    }
}
