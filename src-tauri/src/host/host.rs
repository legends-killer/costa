use serde::{Deserialize, Serialize};

use crate::constant::DEFAULT_HOST;
use std::collections::HashMap;

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
