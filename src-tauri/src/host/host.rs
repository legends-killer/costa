use debug_print::debug_println;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    constant::{DEFAULT_HOST, DEFAULT_PATH},
    tray::operation::OperationId,
};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub selected_host: Option<String>,
    pub host_map: HashMap<String, String>,
}

impl Host {
    pub fn default() -> Self {
        Self {
            selected_host: None,
            host_map: HashMap::new(),
        }
    }
    pub fn set_selected_host(&mut self, host: String) {
        self.selected_host = Some(host);
    }
    pub fn get_selected_host(&self) -> Option<String> {
        self.selected_host.clone()
    }
    pub fn get_host_by_name(&self, name: &str) -> Option<&String> {
        self.host_map.get(name)
    }
    pub fn get_host_map(&self) -> HashMap<String, String> {
        self.host_map.clone()
    }
    pub fn set_host_map(&mut self, map: HashMap<String, String>) {
        self.host_map = map;
    }
}

impl From<Host> for serde_json::Value {
    fn from(host: Host) -> serde_json::Value {
        serde_json::to_value(host).unwrap()
    }
}

pub trait HostOperation {
    fn exec_operation(&self, operation: OperationId, params: serde_json::Value);
}

impl HostOperation for Host {
    fn exec_operation(&self, operation: OperationId, params: serde_json::Value) {
        let client = Client::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();
        debug_println!("exec operation: {:?}", operation);
        match operation {
            OperationId::None => todo!(),
            OperationId::ClipboardSchema => {
                debug_println!("selected host: {:?}", self.selected_host);
                let client = client.clone();
                let schema_str = String::from(params["value"].as_str().unwrap());
                println!("schema: {:?}", schema_str);
                let url = format!(
                    "{}{}/routeSwitch",
                    self.selected_host.as_ref().unwrap(),
                    DEFAULT_PATH
                );
                tauri::async_runtime::spawn(async move {
                    let res = client.post(url).body(schema_str).send().await;
                    debug_println!("{:?}", res);
                });
            }
            OperationId::Safari => todo!(),
            OperationId::InstallApp => todo!(),
            OperationId::Quit => todo!(),
            OperationId::OpenSimulator => todo!(),
            OperationId::SelectHost => todo!(),
            OperationId::RouteBack => {
                debug_println!("route back operation: {:?}", params);
                let client = client.clone();
                let url = format!(
                    "{}{}/routeVCOperation",
                    self.selected_host.as_ref().unwrap(),
                    DEFAULT_PATH
                );
                let operation = String::from(params["value"].as_str().unwrap());
                tauri::async_runtime::spawn(async move {
                    let res = client.post(url).body(operation).send().await;
                    debug_println!("{:?}", res);
                });
            }
            OperationId::RouteForward => todo!(),
            OperationId::RouteRefresh => todo!(),
            OperationId::SetEnv => {
                let client = client.clone();
                let url = format!(
                    "{}{}/setEnv",
                    self.selected_host.as_ref().unwrap(),
                    DEFAULT_PATH
                );
                let set_env_params: SetEnvParams = serde_json::from_value(params["value"].clone()).unwrap();
                debug_println!("{:?}", set_env_params);
                tauri::async_runtime::spawn(async move {
                    let res = client.post(url).json(&set_env_params).send().await;
                    debug_println!("{:?}", res);
                });
            }
            OperationId::Login => todo!(),
            OperationId::Logout => todo!(),
            OperationId::DebugMenu => {
                let client = client.clone();
                let url = format!(
                    "{}{}/routeSwitch",
                    self.selected_host.as_ref().unwrap(),
                    DEFAULT_PATH
                );
                let body_json = json!({});
                tauri::async_runtime::spawn(async move {
                    let res = client.post(url).body("sslocal://debug").send().await;
                    debug_println!("{:?}", res);
                });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetEnvParams {
    #[serde(rename = "envType")]
    pub env_type: String,
    #[serde(rename = "isOn")]
    pub is_on: bool,
    pub name: String,
    #[serde(rename = "geckoOnline")]
    pub gecko_online: bool,
}
