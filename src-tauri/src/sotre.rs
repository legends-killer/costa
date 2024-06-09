use std::{borrow::Borrow, ops::Deref, sync::Mutex};

use debug_print::debug_println;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use tauri::{App, AppHandle, EventLoopMessage, Manager, SystemTrayMenu};
use tauri_plugin_store::{self, Store, StoreBuilder};

use crate::{
    path::{get_app_data_dir, get_sotre_path},
    simulator::{command::get_all_devices, device::DeviceMap},
    tray::menu::TrayMenu,
};

pub enum StoreKey {
    Simulator,
    Tray,
}

impl StoreKey {
    pub fn as_str(&self) -> String {
        match self {
            &StoreKey::Simulator => "simulator".to_owned(),
            &StoreKey::Tray => "tray".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostaStoreWrapper {
    pub simulator: DeviceMap,
    pub tray: TrayMenu,
}

pub struct CostaStore {
    pub store: Mutex<CostaStoreWrapper>,
}

impl CostaStoreWrapper {
    // impl getter function
    pub fn get(&self, key: StoreKey) -> Option<serde_json::Value> {
        match key {
            StoreKey::Simulator => Some(json!(&self.simulator)),
            StoreKey::Tray => Some(json!(&self.tray)),
            _ => None,
        }
    }
    // impl setter function
    pub fn set(
        &mut self,
        key: StoreKey,
        value: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            StoreKey::Simulator => {
                self.simulator = serde_json::from_value(value)?;
                Ok(())
            }
            StoreKey::Tray => {
                self.tray = serde_json::from_value(value)?;
                Ok(())
            }
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid key",
            ))),
        }
    }
}

pub fn dbg_init_store(app: &App) {
    let all_dev: String = get_all_devices().into();
    let res = set_store(
        app,
        StoreKey::Tray,
        &json!(TrayMenu {
            simulator: get_all_devices()
        }), // .to_string(),
    );
}

pub fn set_store(
    app: &App,
    key: StoreKey,
    value: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let state: tauri::State<CostaStore> = handle.state();
    let store = &state.store.lock().unwrap();

    Ok(())
}

// pub fn get_store(app: &App, key: StoreKey) -> Option<CostaStoreWrapper> {
//     let handle = app.handle();
//     let state: tauri::State<CostaStore> = handle.state();
//     let binding = state.store.lock().unwrap();
//     let store = binding.deref();
//     let t = serde_json::from_value::<CostaStoreWrapper>();
//     match t {
//         Ok(v) => Some(v),
//         Err(e) => {
//             debug_println!("Error: {:?}", e);
//             None
//         }
//     }
// }

pub fn get_tray_store(app: &App) -> Option<TrayMenu> {
    let handle = app.handle();
    let state: tauri::State<CostaStore> = handle.state();
    let store = &state.store.lock().unwrap();
    let value = store.get(StoreKey::Tray);
    // debug_println!("store value: {:?}", value);
    match value {
        Some(v) => Some(serde_json::from_value::<TrayMenu>(v).unwrap()),
        None => {
            debug_println!("No value found for key: {}", StoreKey::Tray.as_str());
            None
        }
    }
    // serde_json::from_value::<TrayMenu>(store.get(StoreKey::Tray.as_str()).unwrap().clone())
}

pub trait StoreTrait {
    fn get_tray_store(&self) -> Option<TrayMenu>;
}

impl StoreTrait for AppHandle {
    fn get_tray_store(&self) -> Option<TrayMenu> {
        let state: tauri::State<CostaStore> = self.state();
        let store = &state.store.lock().unwrap();
        let value = store.get(StoreKey::Tray);
        // debug_println!("store value: {:?}", value);
        match value {
            Some(v) => Some(serde_json::from_value::<TrayMenu>(v).unwrap()),
            None => {
                debug_println!("No value found for key: {}", StoreKey::Tray.as_str());
                None
            }
        }
        // serde_json::from_value::<TrayMenu>(store.get(StoreKey::Tray.as_str()).unwrap().clone())
    }
}
