use super::webview_command;

pub fn assamble_handler(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
      webview_command::call_app_method,
      webview_command::get_app_store,
      webview_command::get_clipborad_value,
      webview_command::goto_schema,
      webview_command::set_boe,
      webview_command::set_ppe,
      webview_command::goto_schema_by_sslocal,
      webview_command::xcode_install,
      webview_command::install_simulator,
      webview_command::download_simulator,
      webview_command::get_app_pkg_list,
      webview_command::install_app,
      webview_command::download_app,
      webview_command::get_installed_simulator_runtime_list,
      webview_command::get_available_simulator_runtime_list,
      webview_command::delete_simulator_runtime,
    ])
}
