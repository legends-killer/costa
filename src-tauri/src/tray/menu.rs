use crate::simulator::device::DeviceMap;


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TrayMenu {
  pub simulator: DeviceMap,
}
impl TrayMenu {
    pub(crate) fn default() -> TrayMenu {
        TrayMenu {
            simulator: DeviceMap::default(),
        }
    }
}

impl From<Option<serde_json::Value>> for TrayMenu {
    fn from(value: Option<serde_json::Value>) -> Self {
        serde_json::from_value(value.unwrap_or_else(|| "{}".into())).unwrap_or_else(|_| TrayMenu::default())
    }
}
