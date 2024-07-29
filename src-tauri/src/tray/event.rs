use tauri::{AppHandle, SystemTrayEvent};

use crate::{
    simulator::command::{boot_device, get_all_devices, open_simulator_app},
    sotre::{get_tauri_store, set_tauri_store},
    tray::operation::OperationId,
};

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app_handle.tray_handle().get_item(&id);
            dbg!(&id);
            match OperationId::from(id.to_owned()) {
                OperationId::InstallApp => {}
                OperationId::Quit => app_handle.exit(0),
                OperationId::Safari => {}
                OperationId::QrCode => {}
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
