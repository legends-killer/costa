use std::{borrow::Borrow, ops::Deref, sync::Mutex};

use debug_print::debug_println;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use tauri::{App, AppHandle, EventLoopMessage, Manager, SystemTrayMenu};
use tauri_plugin_store::{self, Store, StoreBuilder, StoreCollection};

use crate::{
    constant::APP_NAME,
    file::check_file_if_exists,
    path::{get_app_data_dir, get_sotre_path},
    simulator::{command::get_all_devices, device::DeviceMap},
    tray::menu::TrayMenu,
};
use tauri::Wry;
use tauri_plugin_store::with_store;

pub enum StoreKey {
    Simulator,
    Tray,
    RecentDevices,
}

impl StoreKey {
    pub fn as_str(&self) -> String {
        match self {
            &StoreKey::Simulator => "simulator".to_owned(),
            &StoreKey::Tray => "tray".to_owned(),
            &StoreKey::RecentDevices => "recent_devices".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostaStoreWrapper {
    pub simulator: DeviceMap,
    pub tray: TrayMenu,
    pub recent_devices: Vec<String>,
}

// pub struct CostaStore {
//     pub store: Mutex<CostaStoreWrapper>,
// }

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
            StoreKey::RecentDevices => {
                self.recent_devices = serde_json::from_value(value)?;
                Ok(())
            }
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid key",
            ))),
        }
    }
}

// Helper enum to abstract over &App and &AppHandle
enum AppHandleRef {
    App(AppHandle),
    AppHandle(AppHandle),
}

impl From<&App> for AppHandleRef {
    fn from(app: &App) -> Self {
        AppHandleRef::App(app.app_handle())
    }
}

impl From<&mut App> for AppHandleRef {
    fn from(app: &mut App) -> Self {
        AppHandleRef::App(app.app_handle())
    }
}

impl From<&AppHandle> for AppHandleRef {
    fn from(app_handle: &AppHandle) -> Self {
        AppHandleRef::AppHandle(app_handle.clone())
    }
}

impl From<AppHandle> for AppHandleRef {
    fn from(app_handle: AppHandle) -> Self {
        AppHandleRef::AppHandle(app_handle)
    }
}

impl Into<AppHandle> for AppHandleRef {
    fn into(self) -> AppHandle {
        match self {
            AppHandleRef::App(app) => app,
            AppHandleRef::AppHandle(app_handle) => app_handle,
        }
    }
}

pub fn init_tauri_store<T: Into<AppHandleRef>>(app: T) {
    let app_handle_ref: AppHandleRef = app.into();
    // Extract the AppHandle from AppHandleRef before calling state
    let app_handle = match app_handle_ref {
        AppHandleRef::App(app_handle) => app_handle,
        AppHandleRef::AppHandle(app_handle) => app_handle,
    };
    let mut store = StoreBuilder::new(app_handle.clone(), get_sotre_path()).build();
    let store_content = CostaStoreWrapper {
        simulator: get_all_devices(),
        tray: TrayMenu {
            simulator: get_all_devices(),
        },
        recent_devices: vec![],
    };
    store
        .insert(APP_NAME.to_string(), json!(store_content))
        .unwrap();
    store.save();
    // app.manage(store);
}

pub fn get_tauri_store<T: Into<AppHandleRef>>(app: T) -> std::option::Option<CostaStoreWrapper> {
    let app_handle_ref: AppHandleRef = app.into();
    // Extract the AppHandle from AppHandleRef before calling state
    let app_handle = match app_handle_ref {
        AppHandleRef::App(app_handle) => app_handle,
        AppHandleRef::AppHandle(app_handle) => app_handle,
    };
    let stores = app_handle.state::<StoreCollection<Wry>>();
    let path = get_sotre_path();
    let mut ret: Option<CostaStoreWrapper> = None;

    with_store(app_handle.clone(), stores, path, |store| {
        ret = store
            .get(APP_NAME.to_string())
            .cloned()
            .map(|value| serde_json::from_value::<CostaStoreWrapper>(value).unwrap());
        Ok(())
    });

    ret
}

pub fn set_tauri_store<T: Into<AppHandleRef>>(app: T, new_store: CostaStoreWrapper) {
    let app_handle_ref: AppHandleRef = app.into();
    // Extract the AppHandle from AppHandleRef before calling state
    let app_handle = match app_handle_ref {
        AppHandleRef::App(app_handle) => app_handle,
        AppHandleRef::AppHandle(app_handle) => app_handle,
    };
    let stores = app_handle.state::<StoreCollection<Wry>>();
    let path = get_sotre_path();

    with_store(app_handle.clone(), stores, path, |store| {
        store
            .insert(APP_NAME.to_string(), json!(new_store))
            .unwrap();
        store.save();
        Ok(())
    });
}
