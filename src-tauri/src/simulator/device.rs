use debug_print::debug_println;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::command::{open_url, list_apps, terminate_app, install_app, uninstall_app};

/**
 * HashMap<String, Vec<Device>> is a map of devices grouped by version
 * key: version
 * value: list of devices
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceMap {
    #[serde(default = "HashMap::new")]
    pub devices: HashMap<String, Vec<Device>>,
}

impl DeviceMap {
    fn new(devices: HashMap<String, Vec<Device>>) -> Self {
        Self { devices }
    }
    pub fn add_device(&mut self, version: &str, device: Device) {
        let devices = self.devices.entry(version.to_string()).or_insert(vec![]);
        devices.push(device);
    }
    pub fn remove_device(&mut self, version: &str, udid: &str) {
        if let Some(devices) = self.devices.get_mut(version) {
            devices.retain(|d| d.udid != udid);
        }
    }
    pub fn get_device(&self, version: &str, udid: &str) -> Option<&Device> {
        self.devices
            .get(version)
            .and_then(|v| v.iter().find(|d| d.udid == udid))
    }
    pub fn get_device_by_udid(&self, udid: &str) -> Option<&Device> {
        self.devices
            .values()
            .find_map(|v| v.iter().find(|d| d.udid == udid))
    }
    pub fn get_default_device(&self) -> Option<&Device> {
        self.devices.values().next().and_then(|v| {
            // find name on iPhone 15 Pro
            v.iter().find(|d| d.name == "iPhone 15 Pro")
        })
    }
    pub fn get_first_booted_device(&self) -> Option<&Device> {
        self.devices
            .values()
            .find_map(|v| v.iter().find(|d| d.state == "Booted"))
    }
    // default
    pub fn default() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
}

impl Into<String> for DeviceMap {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".into())
    }
}

impl From<Option<serde_json::Value>> for DeviceMap {
    fn from(value: Option<serde_json::Value>) -> Self {
        serde_json::from_value(value.unwrap_or_else(|| "{}".into()))
            .unwrap_or_else(|_| DeviceMap::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    #[serde(rename = "dataPath")]
    pub data_path: String,
    #[serde(rename = "dataPathSize")]
    pub data_path_size: u64,
    #[serde(rename = "logPath")]
    pub log_path: String,
    pub udid: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "deviceTypeIdentifier")]
    pub device_type_identifier: String,
    pub state: String,
    pub name: String,
    #[serde(rename = "buildVersion", skip_serializing_if = "Option::is_none")]
    pub last_booted_at: Option<String>,
    #[serde(rename = "buildVersion", skip_serializing_if = "Option::is_none")]
    pub log_path_size: Option<u64>,
    pub os_version: Option<String>,
}

impl Device {
    fn new(
        data_path: String,
        data_path_size: u64,
        log_path: String,
        udid: String,
        is_available: bool,
        device_type_identifier: String,
        state: String,
        name: String,
        last_booted_at: Option<String>,
        log_path_size: Option<u64>,
        os_version: Option<String>,
    ) -> Self {
        Self {
            data_path,
            data_path_size,
            log_path,
            udid,
            is_available,
            device_type_identifier,
            state,
            name,
            last_booted_at,
            log_path_size,
            os_version: os_version,
        }
    }
    pub fn open_url(&self, url: &str) {
        open_url(&self.udid, url);
    }
    pub fn list_apps(&self) {
        list_apps(&self.udid);
    }
    pub fn terminate_app(&self, bundle_id: &str) {
        terminate_app(&self.udid, bundle_id);
    }
    pub fn install_app(&self, app_path: &str) {
        debug_println!("installing app: {}", app_path);
        install_app(&self.udid, app_path);
    }
    pub fn uninstall_app(&self, bundle_id: &str) {
        uninstall_app(&self.udid, bundle_id);
    }
}
