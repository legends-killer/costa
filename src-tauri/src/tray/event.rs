use debug_print::{debug_print, debug_println};
use image;
use rqrr::{self, DeQRError};
use tauri::{AppHandle, Manager, SystemTrayEvent};
use tauri_plugin_clipboard::ClipboardManager;

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
                OperationId::QrCode => {
                    if let Err(err) = read_clipboard(app_handle) {
                        debug_print!("QR Code Read Error: {:?}", err);
                    }
                }
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

fn read_clipboard(app_handle: &AppHandle) -> Result<(), DeQRError> {
    let handle = app_handle.clone();
    let clipboard = handle.state::<ClipboardManager>();
    // text consider as a url
    if let Ok(has_text) = clipboard.has_text() {
        if has_text {
            let text = clipboard.read_text();
            dbg!(text);
        }
    }
    // image consider as a qr code
    if let Ok(has_image) = clipboard.has_image() {
        if has_image {
            let image = clipboard.read_image_binary().unwrap();
            let img = image::load_from_memory(&image).unwrap().to_luma8();
            // Prepare for detection
            let mut img = rqrr::PreparedImage::prepare(img);
            // Search for grids, without decoding
            let grids = img.detect_grids();
            if (grids.len() == 0) {
                dbg!("No QR Code detected");
                return Err(DeQRError::UnknownDataType);
            }
            // Decode the grid
            let (meta, content) = grids[0].decode()?;
            debug_println!("QR Code Meta: {:?}", meta);
            debug_println!("QR Code Content: {:?}", content);
        }
    }
    Ok(())
}
