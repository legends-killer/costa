/*
 * @Author: legends-killer
 * @Date: 2024-07-28 14:03:15
 * @LastEditors: legends-killer
 * @LastEditTime: 2024-07-28 14:09:44
 * @Description: 
 */

use tauri::AppHandle;

use crate::tray::tray::init_system_tray_menu;

pub async fn tick(handle: AppHandle) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
    loop {
        interval.tick().await;
        let menu = init_system_tray_menu(None, Some(handle.clone()));
        let result = handle.tray_handle().set_menu(menu);
    }
}

