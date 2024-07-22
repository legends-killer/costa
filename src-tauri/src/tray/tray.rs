use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Mutex,
};

use debug_print::debug_println;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{
    api::shell::open, App, AppHandle, CustomMenuItem, Manager, MenuEntry, State, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu,
};

use crate::{
    simulator::{
        self,
        command::{boot_device, get_all_devices, open_simulator_app},
    },
    sotre::{get_tauri_store, set_tauri_store, StoreKey},
    tray::menu::{self, TrayMenu},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItem {
    id: String,
    label: String,
}

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app_handle.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "safari_dev_tool" => {}
                "quit" => app_handle.exit(0),
                s => {
                    // let store: State<CostaStore> = app_handle.state();
                    // let mut menu_state = store.store.lock().unwrap()
                    let mut store = get_tauri_store(app_handle);
                    let mut menu_state = store.unwrap();
                    let mut devs = menu_state.simulator.devices.clone();
                    // match the id
                    for (_versiuon, devices) in devs.iter_mut() {
                        for device in devices.iter_mut() {
                            dbg!(device.udid.clone());
                            if device.udid == id {
                                // set recent devices
                                let mut recent_devices = menu_state.recent_devices.clone();
                                // only keep 5 recent devices, if the device is already in the list, move it to the first
                                if recent_devices.contains(&id) {
                                    recent_devices.retain(|x| x != &id);
                                } else if recent_devices.len() >= 5 {
                                    recent_devices.pop();
                                }
                                recent_devices.insert(0, id.clone());
                                menu_state.recent_devices = recent_devices;
                                set_tauri_store(app_handle, menu_state.clone());
                            }
                        }
                    }
                    let devices = get_all_devices();
                    boot_device(&id);
                    open_simulator_app();
                }
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn init_system_tray() -> SystemTray {
    let init_menu = SystemTrayMenu::new();
    SystemTray::new().with_menu(init_menu)
}

pub fn init_system_tray_menu(app: Option<&App>, handle: Option<AppHandle>) -> SystemTrayMenu {
    if (app.is_none() && handle.is_none()) {
        return SystemTrayMenu::new();
    }
    let mut simulators = TrayMenu {
        simulator: get_all_devices(),
    };
    // if (!app.is_none()) {
    //     simulators = get_tray_store(app.unwrap()).unwrap();
    // } else if (!handle.is_none()) {
    //     simulators = handle.unwrap().get_tray_store().unwrap();
    // }
    // debug_println!("{:?}", simulators);

    let sub_menu_devices = {
        let mut menu = SystemTrayMenu::new();
        for (version, devices) in simulators.simulator.devices.iter() {
            for device in devices {
                let mut menu_item = CustomMenuItem::new(device.udid.clone(), device.name.clone());
                if (device.state == "Booted") {
                    debug_println!("!!!!!!!!!!!!");
                    menu_item = menu_item.selected();
                }
                debug_println!("{:?} {:?}", device.state, device.udid);
                menu = menu.add_item(menu_item);
            }
            // menu = menu.add_item(CustomMenuItem::new(id.to_string(), label.to_string()));
        }
        SystemTraySubmenu::new("Devices", menu)
    };
    let sub_menu_recent_devices = {
        let mut menu = SystemTrayMenu::new();
        let store = get_tauri_store(handle.unwrap());
        let menu_state = store.unwrap();
        for recent_dev_id in menu_state.recent_devices.iter() {
            // find the device from the simulator cache
            let device = menu_state
                .simulator
                .devices
                .iter()
                .find_map(|(version, devices)| {
                    devices.iter().find(|d| d.udid == recent_dev_id.to_string())
                })
                .unwrap();

            let mut menu_item = CustomMenuItem::new(device.udid.clone(), device.name.clone());
            if device.state == "Booted" {
                menu_item = menu_item.selected();
            }
            menu = menu.add_item(menu_item);
        }
        SystemTraySubmenu::new("Recent Devices", menu)
    };
    // let sub_menu_github = {
    //     let mut menu = SystemTrayMenu::new();
    //     for (id, label, _url) in LINKS
    //         .iter()
    //         .filter(|(id, label, _url)| id.starts_with("open-github"))
    //     {
    //         menu = menu.add_item(CustomMenuItem::new(id.to_string(), label.to_string()));
    //     }

    //     SystemTraySubmenu::new("GitHub", menu)
    // };
    SystemTrayMenu::new()
        .add_submenu(sub_menu_devices)
        .add_native_item(SystemTrayMenuItem::Separator)
        // .add_item(CustomMenuItem::new("recent_devices", "Recently Used Devices").disabled())
        .add_submenu(sub_menu_recent_devices)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("qr_code".to_string(), "Scan QR Code"))
        .add_item(CustomMenuItem::new(
            "safari_dev_tool".to_string(),
            "Open Safari Dev Tool",
        ))
        .add_item(CustomMenuItem::new(
            "install_app".to_string(),
            "Install App",
        ))
}

// pub fn get_system_tray_menu(app: &App) -> SystemTrayMenu {
//     let handle = app.handle().tray_handle();
//     let menu: tauri::State<SystemTrayMenu> = handle.get_item(id)
// }

// pub fn update_system_tray_menu(app: &App, new_menu: SystemTrayMenu) {
//     let handle = app.handle();
//     let menu: tauri::State<SystemTrayMenu> = handle.state();
//     let system_tray_menu_lock_guard = menu.store.lock().unwrap();
//     let system_tray_menu = system_tray_menu_lock_guard.deref();
// }

pub fn get_basic_system_tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("visibility-toggle".to_string(), "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

pub fn set_selected_item_to_menu(
    app_handle: &AppHandle,
    id: &str,
    selected: bool,
) -> Result<(), tauri::Error> {
    let handle = app_handle.tray_handle();
    let meun_item = handle.get_item(id);
    meun_item.set_selected(selected)
}
