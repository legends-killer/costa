use debug_print::debug_println;
use serde_json::json;
use tauri::Manager;

use crate::{
    clipboard::ClipboardContent,
    host::host::{Host, HostOperation, SetEnvParams},
    sotre::{self, get_tauri_store, CostaStoreWrapper},
    tray::operation::OperationId, window::costa_window,
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
pub async fn close_boe(app: tauri::AppHandle) -> Result<(), String> {
    let store = get_tauri_store(app.clone());
    if let Some(store) = store {
        let host = store.get(sotre::StoreKey::DebugHosts);
        if let Some(host) = host {
            let host: Host = serde_json::from_value(host).map_err(|e| e.to_string())?;
            host.exec_operation(OperationId::SetEnv, json!({
                "env_type": "boe",
                "is_on": false,
                "name": ""
            }));
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
pub async fn close_ppe(app: tauri::AppHandle) -> Result<(), String> {
    todo!()
}
