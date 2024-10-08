use debug_print::debug_println;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{
    App, AppHandle, CustomMenuItem, Manager, MenuEntry, State, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu,
};

use crate::{
    host::host::Host,
    simulator::{
        self,
        command::{boot_device, get_all_devices, open_simulator_app},
        device::Device,
    },
    sotre::{get_tauri_store, set_tauri_store, AppHandleRef, StoreKey},
    tray::menu::{self, TrayMenu},
};

use super::operation::OperationId;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItem {
    id: String,
    label: String,
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
    let hosts = menu_state.debug_hosts;
    debug_println!("Hosts: {:?}", hosts);

    SystemTrayMenu::new()
        .set_devices(&simulators.simulator)
        .set_recent_devices(&recent_devices)
        .set_debug_hosts(&hosts)
        .set_operation_menu()
        .set_basic_menu()
}

pub trait CostaTray {
    fn set_devices(&self, devices: &simulator::device::DeviceMap) -> SystemTrayMenu;
    fn set_recent_devices(self, devices: &Vec<&simulator::device::Device>) -> SystemTrayMenu;
    fn set_basic_menu(&self) -> SystemTrayMenu;
    fn set_operation_menu(&self) -> SystemTrayMenu;
    fn set_debug_hosts(self, hosts: &Option<Host>) -> SystemTrayMenu;
}

impl CostaTray for SystemTrayMenu {
    fn set_devices(&self, devices: &simulator::device::DeviceMap) -> SystemTrayMenu {
        let sub_menu_devices = {
            let mut menu = SystemTrayMenu::new();
            for (version, devices) in devices.devices.iter() {
                for device in devices {
                    let mut menu_item = CustomMenuItem::new(
                        OperationId::OpenSimulator.to_string() + device.udid.clone().as_str(),
                        device.name.clone() + "-" + device.os_version.clone().unwrap().as_str(),
                    );
                    if (device.state == "Booted") {
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
            // debug_println!("{:?} {:?}", device.state, device.udid);
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
    fn set_debug_hosts(mut self, hosts: &Option<Host>) -> SystemTrayMenu {
        if let Some(hosts) = hosts {
            let sub_menu_hosts = {
                let mut menu = SystemTrayMenu::new();
                for (title, url) in hosts.host_map.iter() {
                    let mut menu_item = CustomMenuItem::new(
                        OperationId::SelectHost.to_string() + url,
                        title.clone(),
                    );
                    if hosts.selected_host != None {
                        if url == hosts.selected_host.as_ref().unwrap().as_str() {
                            menu_item = menu_item.selected();
                        }
                    }
                    menu = menu.add_item(menu_item);
                }
                SystemTraySubmenu::new("Hosts", menu)
            };
            return self
                .clone()
                // .add_item(CustomMenuItem::new(
                //     "selected_host".to_string(),
                //     hosts.selected_host.clone(),
                // ))
                .add_submenu(sub_menu_hosts)
                .add_native_item(SystemTrayMenuItem::Separator);
        } else {
            return self
                .clone()
                .add_item(
                    CustomMenuItem::new("debug_host".to_string(), "Debug Host Not Found")
                        .disabled(),
                )
                .add_native_item(SystemTrayMenuItem::Separator);
        }
    }
    fn set_basic_menu(&self) -> SystemTrayMenu {
        self.clone()
            .add_item(CustomMenuItem::new(OperationId::QrCode, "Scan QR Code"))
            .add_item(CustomMenuItem::new(
                OperationId::Safari,
                "Open Safari Dev Tool",
            ))
            .add_item(CustomMenuItem::new(OperationId::InstallApp, "Install App"))
            .add_item(CustomMenuItem::new(OperationId::Quit, "Quit"))
    }
    fn set_operation_menu(&self) -> SystemTrayMenu {
        self.clone()
            .add_item(CustomMenuItem::new(OperationId::RouteBack, "Route Back"))
            .add_item(CustomMenuItem::new(
                OperationId::RouteForward,
                "Route Forward",
            ))
            .add_item(CustomMenuItem::new(
                OperationId::RouteRefresh,
                "Route Refresh",
            ))
            .add_item(CustomMenuItem::new(OperationId::SetBOE, "Set BOE"))
            .add_item(CustomMenuItem::new(OperationId::SetPPE, "Set PPE"))
            .add_item(CustomMenuItem::new(OperationId::Login, "Login"))
            .add_item(CustomMenuItem::new(OperationId::Logout, "Logout"))
            .add_item(CustomMenuItem::new(
                OperationId::DebugMenu,
                "Open Debug Menu",
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
    }
}
