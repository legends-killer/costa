// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constant;
mod file;
mod path;
mod simulator;
mod sotre;
mod tray;

use file::check_file_if_exists;
use path::get_sotre_path;
use tauri_plugin_log::LogTarget;
use tray::tray::{init_system_tray, init_system_tray_menu, on_system_tray_event};

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .system_tray(init_system_tray())
        .setup(|app| {
            // init tray menu
            let menu = init_system_tray_menu(Some(&app), Some(app.handle().clone()));
            let _ = app.tray_handle().set_menu(menu);
            // init store
            if !check_file_if_exists(get_sotre_path()) {
                sotre::init_tauri_store(app);
            }
            Ok(())
        })
        .on_system_tray_event(on_system_tray_event)
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let menu = init_system_tray_menu(None, Some(handle.clone()));
            let result = handle.tray_handle().set_menu(menu);
        }
    });
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
