use std::collections::HashMap;

use debug_print::debug_println;
use log::info;
use log4rs::config::runtime;
use regex::Regex;
use serde_json::json;
use tauri::Manager;

use crate::{
    clipboard::ClipboardContent,
    constant_local::{DCD_DOWNLOAD_URL, MCT_DOWNLOAD_URL, SIMULATOR_DOWNLOAD_URL},
    host::host::{Host, HostOperation, SetEnvParams},
    simulator::{self, device::DeviceMap, runtime::Runtime},
    sotre::{self, get_tauri_store, CostaStoreWrapper},
    tray::{menu::TrayMenu, operation::OperationId},
    window::costa_window,
};

#[derive(Default)]
pub struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn call_app_method(state: tauri::State<'_, MyState>) -> Result<(), String> {
    *state.s.lock().unwrap() = "new string".into();
    state.t.lock().unwrap().insert("key".into(), "value".into());
    Ok(())
}

#[tauri::command]
pub async fn get_app_store(app: tauri::AppHandle) -> Result<Option<CostaStoreWrapper>, String> {
    Ok(sotre::get_tauri_store(app))
}

#[tauri::command]
pub async fn get_clipborad_value(app: tauri::AppHandle) -> Result<String, String> {
    // get clipboard value from store
    let store = get_tauri_store(app);
    if let Some(store) = store {
        let value = store.get(sotre::StoreKey::ClipboardContent);
        if let Some(value) = value {
            let value: ClipboardContent =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            println!("clipboard value: {:?}", value);
            Ok(value.content)
        } else {
            Err("clipboard value not found".to_string())
        }
    } else {
        Err("store not found".to_string())
    }
}

#[tauri::command]
pub async fn goto_schema(app: tauri::AppHandle, schema: String) -> Result<(), String> {
    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let host = store.get(sotre::StoreKey::DebugHosts);
        if let Some(host) = host {
            let host: Host = serde_json::from_value(host).map_err(|e| e.to_string())?;
            host.exec_operation(OperationId::ClipboardSchema, json!({ "value": schema }));
        }
    }
    Ok(())
}

/**
 * 懂车帝暂时用 snssdk36 跳转
 */
#[tauri::command]
pub async fn goto_schema_by_sslocal(app: tauri::AppHandle, url: String) -> Result<(), String> {
    debug_println!("goto schema by sslocal: {}", url);
    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let tray_menu = TrayMenu::from(store.get(sotre::StoreKey::Tray));
        let device_map = tray_menu.simulator;
        // get selected device
        let device = device_map.get_first_booted_device();
        debug_println!("booted device: {:?}", device);
        if let Some(device) = device {
            device.open_url(&url.replace("sslocal", "snssdk36"));
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn set_boe(app: tauri::AppHandle, params: SetEnvParams) -> Result<(), String> {
    debug_println!("set boe: {:?}", params);
    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let host = store.get(sotre::StoreKey::DebugHosts);
        if let Some(host) = host {
            let host: Host = serde_json::from_value(host).map_err(|e| e.to_string())?;
            if params.env_type.starts_with("boe") {
                host.exec_operation(OperationId::SetEnv, json!({ "value": params }));
            } else if params.env_type.starts_with("ppe") {
                host.exec_operation(OperationId::SetEnv, json!({ "value": params }));
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn set_ppe(app: tauri::AppHandle, params: SetEnvParams) -> Result<(), String> {
    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let host = store.get(sotre::StoreKey::DebugHosts);
        if let Some(host) = host {
            let host: Host = serde_json::from_value(host).map_err(|e| e.to_string())?;
            if params.env_type.starts_with("boe") {
                host.exec_operation(OperationId::SetEnv, json!({ "value": params }));
            } else if params.env_type.starts_with("ppe") {
                host.exec_operation(OperationId::SetEnv, json!({ "value": params }));
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn xcode_install(app: tauri::AppHandle) -> Result<(), String> {
    debug_println!("xcode install");
    // tauri::async_runtime::spawn(async move {
    let output = std::process::Command::new("/usr/bin/xcode-select")
        .arg("--install")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    info!("xcode-select install output: {}", output);
    println!("xcode install output: {}", output);
    // }
    // )
    //     .await
    // .map_err(|e| e.to_string())
    Ok(())
}

#[tauri::command]
pub async fn install_simulator(app: tauri::AppHandle, path: String) -> Result<(), String> {
    debug_println!("install simulator: {}", path);
    let res = tauri::async_runtime::spawn(async move {
        // find file with name include "Simulator_Runtime" in ~/Downloads
        let file_path = std::path::Path::new(&std::env::var("HOME").unwrap()).join("Downloads");
        let mut files = std::fs::read_dir(file_path).unwrap();
        let file = files.find(|f| {
            f.as_ref()
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .contains(&path)
        });

        if let Some(file) = file {
            info!(
                "simulator pkg found: {}",
                file.as_ref().unwrap().path().to_str().unwrap()
            );
            let output = std::process::Command::new("xcrun")
                .arg("simctl")
                .arg("runtime")
                .arg("add")
                .arg(file.unwrap().path().to_str().unwrap())
                .output()
                .expect("failed to exec install simulator");
            let output = String::from_utf8(output.stdout).unwrap();
            println!("xcode install output: {}", output);
            info!("simulator installed: {}", output);
            Ok(())
        } else {
            Err("simulator package not found in ~/Downloads, plz check")
        }
    })
    .await
    .map_err(|e| e.to_string());
    res.unwrap().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_simulator(app: tauri::AppHandle) -> Result<(), String> {
    debug_println!("open simulator download url");
    let output = std::process::Command::new("open")
        .arg(SIMULATOR_DOWNLOAD_URL)
        .output()
        .expect("failed to exec download simulator");
    let output = String::from_utf8(output.stdout).unwrap();
    info!("download simulator output: {}", output);
    Ok(())
}

#[tauri::command]
pub async fn download_app(app: tauri::AppHandle, params: String) -> Result<(), String> {
    debug_println!("download app: {}", params);
    let url = if params == "mct" {
        MCT_DOWNLOAD_URL
    } else {
        DCD_DOWNLOAD_URL
    };
    let output = std::process::Command::new("open")
        .arg(url)
        .output()
        .expect("failed to exec download app");
    let output = String::from_utf8(output.stdout).unwrap();
    info!("download app output: {}", output);
    Ok(())
}

#[tauri::command]
pub async fn get_app_pkg_list(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    // debug_println!("get app pkg list");
    // find file with name include "Simulator_Runtime" in ~/Downloads
    let file_path = std::path::Path::new(&std::env::var("HOME").unwrap()).join("Downloads");
    let files = std::fs::read_dir(file_path).unwrap();
    // find all files with name include ".app"
    let files = files
        .filter(|f| {
            let re = Regex::new(r"\.(app|app\.zip)$").unwrap();
            re.is_match(f.as_ref().unwrap().path().to_str().unwrap())
        })
        .map(|f| f.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    // info!("app pkg list: {:?}", files);
    Ok(files)
}

#[tauri::command]
pub async fn install_app(app: tauri::AppHandle, params: String) -> Result<(), String> {
    debug_println!("install app: {}", params);
    let is_zip = params.ends_with(".zip");
    let app_path = if is_zip {
        // remove the zip extension
        params.clone().replace(".zip", "")
    } else {
        params.clone()
    };

    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let tray_menu = TrayMenu::from(store.get(sotre::StoreKey::Tray));
        let device_map = tray_menu.simulator;
        // get selected device
        let device = device_map.get_first_booted_device();
        debug_println!("booted device: {:?}", device);
        if let Some(device) = device {
            if is_zip {
                // unzip the file
                let output = std::process::Command::new("unzip")
                    .arg(&params)
                    .arg("-d")
                    // assign to Downloads dir
                    .arg(
                        std::path::Path::new(&std::env::var("HOME").unwrap())
                            .join("Downloads")
                            .to_str()
                            .unwrap(),
                    )
                    .output()
                    .expect("failed to unzip app");
                let output = String::from_utf8(output.stdout).unwrap();
                info!("unzip app output: {}", output);
            }

            debug_println!("unzip done, installing app path: {}", app_path);
            device.install_app(&app_path);
        } else {
            return Err("no booted device found".to_string());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_installed_simulator_runtime_list() -> Result<HashMap<String, Runtime>, String> {
    Ok(simulator::command::get_all_runtimes())
}

#[tauri::command]
pub fn get_available_simulator_runtime_list() -> Result<Vec<String>, String> {
    // debug_println!("get app pkg list");
    // find file with name include "Simulator_Runtime" in ~/Downloads
    let file_path = std::path::Path::new(&std::env::var("HOME").unwrap()).join("Downloads");
    let files = std::fs::read_dir(file_path).unwrap();
    // find all files with name include ".app"
    let files = files
        .filter(|f| {
            let re = Regex::new(r"Simulator_Runtime.dmg").unwrap();
            re.is_match(f.as_ref().unwrap().path().to_str().unwrap())
        })
        .map(|f| f.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    // info!("app pkg list: {:?}", files);
    Ok(files)
}

#[tauri::command]
pub async fn delete_simulator_runtime(app: tauri::AppHandle, id: String) -> Result<(), String> {
    debug_println!("delete simulator runtime: {}", id);
    simulator::command::delete_runtime(app, id);
    Ok(())
}
