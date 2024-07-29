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
        device::Device,
    },
    sotre::{get_tauri_store, set_tauri_store, AppHandleRef, StoreKey},
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
    let store = get_tauri_store(handle.clone().unwrap());
    let menu_state = store.unwrap();
    let recent_devices: Vec<&Device> = menu_state
        .recent_devices
        .iter()
        .map(|id| {
            simulators
                .simulator
                .devices
                .iter()
                .find_map(|(version, devices)| devices.iter().find(|d| d.udid == id.to_string()))
                .unwrap()
        })
        .collect();

    SystemTrayMenu::new()
        .set_devices(&simulators.simulator)
        .set_recent_devices(&recent_devices)
        .set_basic_menu()
}

pub trait CostaTray {
    fn set_devices(&self, devices: &simulator::device::DeviceMap) -> SystemTrayMenu;
    fn set_recent_devices(self, devices: &Vec<&simulator::device::Device>) -> SystemTrayMenu;
    fn set_basic_menu(&self) -> SystemTrayMenu;
}

impl CostaTray for SystemTrayMenu {
    fn set_devices(&self, devices: &simulator::device::DeviceMap) -> SystemTrayMenu {
        let sub_menu_devices = {
            let mut menu = SystemTrayMenu::new();
            for (version, devices) in devices.devices.iter() {
                for device in devices {
                    let mut menu_item = CustomMenuItem::new(
                        device.udid.clone(),
                        device.name.clone() + "-" + device.os_version.clone().unwrap().as_str(),
                    );
                    if (device.state == "Booted") {
                        // debug_println!("!!!!!!!!!!!!");
                        menu_item = menu_item.selected();
                    }
                    // debug_println!("{:?} {:?}", device.state, device.udid);
                    menu = menu.add_item(menu_item);
                }
                // menu = menu.add_item(CustomMenuItem::new(id.to_string(), label.to_string()));
            }
            SystemTraySubmenu::new("Devices", menu)
        };
        self.clone()
            .add_submenu(sub_menu_devices)
            .add_native_item(SystemTrayMenuItem::Separator)
    }
    fn set_recent_devices(mut self, devices: &Vec<&simulator::device::Device>) -> SystemTrayMenu {
        self = self.clone().add_item(
            CustomMenuItem::new("recent_device".to_string(), "Recent Devices").disabled(),
        );
        for device in devices {
            debug_println!("{:?} {:?}", device.state, device.udid);
            let mut menu_item = CustomMenuItem::new(
                device.udid.clone(),
                device.name.clone() + "-" + device.os_version.clone().unwrap().as_str(),
            );
            if device.state == "Booted" {
                menu_item = menu_item.selected();
            }
            self = self.clone().add_item(menu_item);
        }
        return self.clone().add_native_item(SystemTrayMenuItem::Separator);
    }
    fn set_basic_menu(&self) -> SystemTrayMenu {
        self.clone()
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
