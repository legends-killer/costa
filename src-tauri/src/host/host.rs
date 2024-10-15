use debug_print::debug_println;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{constant::DEFAULT_HOST, tray::operation::OperationId};
use std::{collections::HashMap, time::Duration};
use reqwest::Client;

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
    async fn exec_operation(&self, operation: OperationId, params: Option<serde_json::Value>);
}

impl HostOperation for Host {
    async fn exec_operation(&self, operation: OperationId, params: Option<serde_json::Value>) {
        let client = Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
        match operation {
            OperationId::None => todo!(),
            OperationId::QrCode => todo!(),
            OperationId::Safari => todo!(),
            OperationId::InstallApp => todo!(),
            OperationId::Quit => todo!(),
            OperationId::OpenSimulator => todo!(),
            OperationId::SelectHost => todo!(),
            OperationId::RouteBack => todo!(),
            OperationId::RouteForward => todo!(),
            OperationId::RouteRefresh => todo!(),
            OperationId::SetPPE => todo!(),
            OperationId::SetBOE => todo!(),
            OperationId::Login => todo!(),
            OperationId::Logout => todo!(),
            OperationId::DebugMenu => {
                let client = client.clone();
                let url = format!("http://{}/costa/routeSwitch", self.selected_host.as_ref().unwrap());
                let body_json = json!({});
                tauri::async_runtime::spawn(async move {
                    let res = client.post(url).body("sslocal://debug").send().await;
                    debug_println!("{:?}", res);
                });
            },
        }
    }
}
