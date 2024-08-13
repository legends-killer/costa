use super::webview_command;

pub fn assamble_handler(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
      webview_command::call_app_method,
      webview_command::get_app_store,
    ])
}
