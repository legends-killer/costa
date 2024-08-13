use tauri::AppHandle;

pub fn create_window(handle: &AppHandle, label: &str, route: &str) -> Result<(), tauri::Error> {
    let local_window =
        tauri::WindowBuilder::new(handle, label, tauri::WindowUrl::App(route.into())).build()?;
    local_window.set_title("Costa");
    Ok(())
}

pub fn create_url_edit_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    let local_window =
        tauri::WindowBuilder::new(handle, "Costa", tauri::WindowUrl::App("/url_edit".into()))
            .build()?;
    local_window.set_title("Schema Edit");
    Ok(())
}

pub fn create_download_app_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    let local_window = tauri::WindowBuilder::new(
        handle,
        "Costa",
        tauri::WindowUrl::App("/download_app".into()),
    )
    .build()?;
    local_window.set_title("Download App");
    Ok(())
}
