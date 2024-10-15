use strum_macros::{Display, EnumString};
use tauri::{AppHandle, Manager};

use super::costa_router::CostaRouter;

#[derive(EnumString, Debug, Display)]
pub enum EnvName {
    #[strum(to_string = "boe")]
    BOE,
    #[strum(to_string = "ppe")]
    PPE,
    #[strum(to_string = "online")]
    ONLINE,
}

impl From<EnvName> for String {
    fn from(id: EnvName) -> Self {
        id.to_string()
    }
}

impl From<String> for EnvName {
    fn from(id: String) -> Self {
        match id.as_str() {
            "boe" => EnvName::BOE,
            "ppe" => EnvName::PPE,
            "online" => EnvName::ONLINE,
            _ => EnvName::ONLINE,
        }
    }
}

pub fn create_window(handle: &AppHandle, label: &str, route: &str) -> Result<(), tauri::Error> {
    let local_window =
        tauri::WindowBuilder::new(handle, label, tauri::WindowUrl::App(route.into())).build()?;
    local_window.set_title("Costa");
    Ok(())
}

pub fn create_url_edit_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    let local_window = tauri::WindowBuilder::new(
        handle,
        "Costa",
        tauri::WindowUrl::App(CostaRouter::UrlEdit.into()),
    )
    .build()?;
    local_window.set_title("Schema Edit");
    Ok(())
}

pub fn create_download_app_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    let local_window = tauri::WindowBuilder::new(
        handle,
        "Costa",
        tauri::WindowUrl::App(CostaRouter::DownloadApp.into()),
    )
    .build()?;
    local_window.set_title("Download App");
    Ok(())
}

pub fn create_env_edit_window(handle: &AppHandle, env_name: EnvName) -> Result<(), tauri::Error> {
    let local_window = tauri::WindowBuilder::new(
        handle,
        "Costa",
        tauri::WindowUrl::App(CostaRouter::EnvEdit.into()),
    )
    .build()?;
    local_window.set_title("Env Edit");
    // set the env name to the window
    local_window
        .get_window("Costa")
        .unwrap()
        .emit("set_env", env_name.to_string())?;
    Ok(())
}
