/*
 * @Author: legends-killer
 * @Date: 2024-07-28 14:03:15
 * @LastEditors: legends-killer
 * @LastEditTime: 2024-07-28 14:09:44
 * @Description:
 */

use debug_print::debug_println;
use tauri::AppHandle;

use crate::{
    host::{host::Host, scanner::scan_local_debug_host},
    sotre::{get_tauri_store, update_tauri_store},
    tray::tray::init_system_tray_menu,
};

pub async fn tick(handle: AppHandle) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
    loop {
        interval.tick().await;
        let host_scanner_result = process_host_scanner(handle.clone()).await;
        let meun_update_result = process_tray_menu_update(handle.clone());
    }
}

pub fn process_tray_menu_update(handle: AppHandle) {
    let menu = init_system_tray_menu(None, Some(handle.clone()));
    let result = handle.tray_handle().set_menu(menu);
}

pub async fn process_host_scanner(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let result = scan_local_debug_host().await?;
    let store = get_tauri_store(handle.clone());
    if let Some(store) = store {
        let mut selected_host = None;
        if let Some(host) = store.debug_hosts.clone() {
            selected_host = host.selected_host;
        }
        update_tauri_store(
            handle.clone(),
            crate::sotre::StoreKey::DebugHosts,
            serde_json::to_value(Host {
                selected_host,
                host_map: result,
            })?,
        )?;
    }
    Ok(())
}
