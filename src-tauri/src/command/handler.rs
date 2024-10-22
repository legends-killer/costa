use super::webview_command;

pub fn assamble_handler(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
      webview_command::call_app_method,
      webview_command::get_app_store,
      webview_command::get_clipborad_value,
      webview_command::goto_schema,
      webview_command::set_boe,
      webview_command::set_ppe,
      webview_command::close_boe,
      webview_command::close_ppe,
    ])
}
