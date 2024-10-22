// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod command;
mod constant;
mod file;
mod host;
mod path;
mod simulator;
mod sotre;
mod tick;
mod tray;
mod window;
mod constant_local;

use command::handler::assamble_handler;
use file::check_file_if_exists;
use log::{debug, LevelFilter};
use path::get_sotre_path;
use sotre::{set_tauri_store, setup_tauri_store};
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tick::tick;
use tray::event::on_system_tray_event;
use tray::tray::{init_system_tray, init_system_tray_menu};

fn main() {
    // init app
    let mut app_builder = tauri::Builder::default();
    app_builder = assamble_handler(app_builder); // assemble command handler
    let app = app_builder
        .plugin(tauri_plugin_store::Builder::default().build()) // store plugin
        .plugin(tauri_plugin_clipboard::init()) // clipboard plugin
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Info)
                .build(),
        ) // log plugin
        .system_tray(init_system_tray()) // system tray plugin
        .setup(|app| {
            // remove dock icon
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            // init store if not exists
            if !check_file_if_exists(get_sotre_path()) {
                sotre::init_tauri_store(app.handle().clone());
            }
            // setup tauri store
            setup_tauri_store(app.handle().clone());
            // init tray menu
            let menu = init_system_tray_menu(Some(&app), Some(app.handle().clone()));
            let _ = app.tray_handle().set_menu(menu);
            Ok(())
        })
        .on_system_tray_event(on_system_tray_event) // listen system tray event
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    // start tick, to update system tray menu in an infinity loop
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        tick(handle).await;
    });
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
