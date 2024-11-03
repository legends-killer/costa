use std::time::Duration;

use debug_print::{debug_print, debug_println};
use image;
use log::{error, info};
use rqrr::{self, DeQRError};
use serde_json::json;
use tauri::api::notification::Notification;
use tauri::{AppHandle, Manager, SystemTrayEvent};
use tauri_plugin_clipboard::ClipboardManager;

use crate::{
    clipboard::{ClipboardContent, ClipboardType},
    constant::{DEFAULT_HOST, DEFAULT_PATH},
    host::host::HostOperation,
    simulator::command::{
        boot_device, find_all_web_view_windows_in_simultor, get_all_devices, open_safari_dev_tool,
        open_simulator_app,
    },
    sotre::{get_tauri_store, set_tauri_store, update_tauri_store},
    tray::operation::OperationId,
    window::costa_window,
};

use reqwest::Client;

use crate::window::costa_window::{create_download_app_window, create_url_edit_window};

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app_handle.tray_handle().get_item(&id);
            dbg!(&id);
            match OperationId::from(id.to_owned()) {
                // open the download app window
                OperationId::InstallApp => {
                    // if let Err(e) = create_download_app_window(app_handle) {
                    //     debug_print!("Open Window Error: {:?}", e);
                    //     log::error!("Open Window Error: {:?}", e);
                    //     return;
                    // }
                    // just open a url in browser
                    // let url = APP_DOWNLOAD_URL;
                    // let output = std::process::Command::new("open")
                    //     .arg(url)
                    //     .output()
                    //     .expect("failed to execute process");
                    // let output = String::from_utf8(output.stdout).unwrap();
                    let _ = create_download_app_window(app_handle);
                }
                // quit the app
                OperationId::Quit => app_handle.exit(0),
                // open Safari developer tools
                OperationId::Safari => {
                    // send system notification to tell user this is WIP
                    Notification::new("WIP")
                        .title("This feature is under development")
                        .body("Please wait for the next release")
                        .show()
                        .unwrap();
                    let app_handle = app_handle.clone();
                    // get devicemap from the store
                    let devicemap = get_tauri_store(app_handle.clone()).unwrap().simulator;
                    if let Some(device) = devicemap.get_first_booted_device() {
                        // let all_webview_windows = find_all_web_view_windows_in_simultor(&devicemap);
                        // open_safari_dev_tool(device.udid.as_str(), None);
                    }
                }
                // scan QR code in clipboard & open the url edit window
                OperationId::ClipboardSchema => {
                    let clipboard_result = read_clipboard(app_handle);
                    if let Err(e) = clipboard_result {
                        error!("QR Code Read Error: {:?}", e);
                        costa_window::create_url_edit_window(app_handle);
                        return;
                    }
                    // TODO: set the clipboard content to the store
                    let _ = update_tauri_store(
                        app_handle,
                        crate::sotre::StoreKey::ClipboardContent,
                        clipboard_result.unwrap().into(),
                    );
                    if let Err(e) = create_url_edit_window(app_handle) {
                        debug_print!("Open Window Error: {:?}", e);
                        log::error!("Open Window Error: {:?}", e);
                        return;
                    }
                    // TODO: jump to the page that the QR code represents
                }
                // send route back operation to the simulator
                OperationId::RouteBack => {
                    let host_agent = get_tauri_store(app_handle.clone())
                        .unwrap()
                        .debug_hosts
                        .unwrap();
                    host_agent.exec_operation(OperationId::RouteBack, json!({"value": id.clone()}));
                }
                // send route forward operation to the simulator
                OperationId::RouteForward => {}
                // send route refresh operation to the simulator
                OperationId::RouteRefresh => {}
                // setEnv
                OperationId::SetEnv => {
                    let clipboard_result = read_clipboard(app_handle);
                    if let Err(e) = clipboard_result {
                        error!("QR Code Read Error: {:?}", e);
                        costa_window::create_env_edit_window(app_handle);
                        return;
                    };
                    let _ = update_tauri_store(
                        app_handle,
                        crate::sotre::StoreKey::ClipboardContent,
                        clipboard_result.unwrap().into(),
                    );
                    costa_window::create_env_edit_window(app_handle);
                }
                // login to the host
                OperationId::Login => {}
                // logout from the host
                OperationId::Logout => {}
                // open the debug menu
                OperationId::DebugMenu => {
                    let host_agent = get_tauri_store(app_handle.clone())
                        .unwrap()
                        .debug_hosts
                        .unwrap();
                    host_agent.exec_operation(OperationId::DebugMenu, json!({}));
                }
                // open the simulator app
                OperationId::OpenSimulator => {
                    let app_handle = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let id_clone = id.clone();
                        let dev_id = id_clone
                            .split(OperationId::OpenSimulator.to_string().as_str())
                            .last()
                            .unwrap();
                        debug_println!("Open Simulator: {}", dev_id);
                        let mut store = get_tauri_store(app_handle.clone());
                        let mut menu_state = store.unwrap();
                        let mut devs = menu_state.simulator.devices.clone();
                        // match the device id
                        for (_versiuon, devices) in devs.iter_mut() {
                            for device in devices.iter_mut() {
                                // dbg!(device.udid.clone());
                                if device.udid == dev_id {
                                    // set recent devices
                                    let mut recent_devices = menu_state.recent_devices.clone();
                                    // only keep 5 recent devices, if the device is already in the list, move it to the first
                                    if recent_devices.contains(&dev_id.to_string()) {
                                        recent_devices.retain(|x| x != &dev_id);
                                    } else if recent_devices.len() >= 5 {
                                        recent_devices.pop();
                                    }
                                    recent_devices.insert(0, dev_id.to_string());
                                    menu_state.recent_devices = recent_devices;
                                    set_tauri_store(app_handle.clone(), menu_state.clone());
                                }
                            }
                        }
                        let devices = get_all_devices();
                        boot_device(&dev_id);
                        open_simulator_app();
                    });
                }
                OperationId::SelectHost => {
                    let host = id
                        .split(OperationId::SelectHost.to_string().as_str())
                        .last()
                        .unwrap();
                    let mut store = get_tauri_store(app_handle.clone());
                    let mut menu_state = store.unwrap();
                    let mut hosts = menu_state.debug_hosts.unwrap();
                    // set the selected host
                    info!("Select Host: {}", host);
                    hosts.set_selected_host(host.to_owned());
                    update_tauri_store(
                        app_handle,
                        crate::sotre::StoreKey::DebugHosts,
                        hosts.into(),
                    );
                }
                OperationId::None => {}
            }
        }
        _ => {}
    }
}

fn read_clipboard(app_handle: &AppHandle) -> Result<ClipboardContent, DeQRError> {
    let handle = app_handle.clone();
    let clipboard = handle.state::<ClipboardManager>();
    // text consider as a url
    if let Ok(has_text) = clipboard.has_text() {
        if has_text {
            let text = clipboard.read_text();
            debug_println!("Clipboard Text: {:?}", text);
            return Ok(ClipboardContent {
                content: text.unwrap(),
                clipboard_type: ClipboardType::Text,
            });
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
            return Ok(ClipboardContent {
                content,
                clipboard_type: ClipboardType::Image,
            });
        }
    }
    Err(DeQRError::UnknownDataType)
}
