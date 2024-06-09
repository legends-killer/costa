// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constant;
mod file;
mod path;
mod simulator;
mod sotre;
mod tray;

use std::sync::Mutex;

use debug_print::debug_println;
use diesel::result;
use path::get_app_data_dir;
use serde_json::json;
use simulator::command::get_all_devices;
use sotre::{dbg_init_store, CostaStore, CostaStoreWrapper};
use tauri::{AppHandle, SystemTrayMenu};
use tauri_plugin_log::LogTarget;
use tauri_plugin_store::StoreBuilder;
use tray::{
    menu::TrayMenu,
    tray::{init_system_tray, init_system_tray_menu, on_system_tray_event},
};

fn main() {
    let app = tauri::Builder::default()
        .manage(CostaStore {
            store: Mutex::new(CostaStoreWrapper {
                simulator: get_all_devices(),
                tray: TrayMenu {
                    simulator: get_all_devices(),
                },
            }),
        })
        // .manage(SystemTrayMenuWrapper {
        //     store: Mutex::new(SystemTrayMenu::default()),
        // })
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .system_tray(init_system_tray())
        .setup(|app| {
            let menu = init_system_tray_menu(Some(&app), Some(app.handle().clone()));
            let _ = app.tray_handle().set_menu(menu);
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
