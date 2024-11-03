use debug_print::debug_println;
use strum_macros::{Display, EnumString};
use tauri::{AppHandle, LogicalSize, Manager, Size};

use crate::constant::IS_DEBUG;

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
    let window = handle.get_window(label);
    if let Some(existing_window) = window {}

    // handle.run_on_main_thread
    let local_window =
        tauri::WindowBuilder::new(handle, label, tauri::WindowUrl::App(route.into())).build()?;
    local_window.set_title("Costa");
    #[cfg(debug_assertions)]
    local_window.open_devtools();

    Ok(())
}

macro_rules! create_costa_window {
    ($handle:expr, $label:expr, $route:expr, $title:expr, $width:expr, $height:expr) => {{
        // Check if the window already exists
        let window = $handle.get_window($label);
        if let Some(existing_window) = window {
            // show the existing window, and emit a refresh event
            debug_println!("show the existing window");
            existing_window.show();
            existing_window.set_focus();
            existing_window.set_size(Size::Logical(LogicalSize {
                width: $width,
                height: $height,
            }));
            existing_window.emit("refresh", "").unwrap();
            return Ok(());
        }
        // Create a new window
        let local_window =
            tauri::WindowBuilder::new($handle, $label, tauri::WindowUrl::App($route.into()))
                .build()?;

        // Set the window title
        local_window.set_title($title)?;
        local_window.set_size(Size::Logical(LogicalSize {
            width: $width,
            height: $height,
        }));

        // Open DevTools if in debug mode
        #[cfg(debug_assertions)]
        local_window.open_devtools();

        Ok(())
    }};
}

pub fn create_url_edit_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    create_costa_window!(
        handle,
        &CostaRouter::UrlEdit.to_string(),
        CostaRouter::UrlEdit.to_string(),
        "Schema Edit",
        1000.0,
        800.0
    )
}

pub fn create_download_app_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    create_costa_window!(
        handle,
        &CostaRouter::DownloadApp.to_string(),
        CostaRouter::DownloadApp.to_string(),
        "Download App",
        1000.0,
        800.0
    )
}

pub fn create_env_edit_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    create_costa_window!(
        handle,
        &CostaRouter::EnvEdit.to_string(),
        CostaRouter::EnvEdit.to_string(),
        "Env Edit",
        1000.0,
        800.0
    )
}
